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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:17:41 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Pmn Pin Function Control Register"]
unsafe impl ::core::marker::Send for super::Pfs {}
unsafe impl ::core::marker::Sync for super::Pfs {}
impl super::Pfs {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "P000 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p000pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P000Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P000Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "P000 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p000pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P000PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P000PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "P000 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p000pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P000PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P000PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3usize),
            )
        }
    }

    #[doc = "P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p00pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P00Pfs_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x20usize))
        }
    }

    #[doc = "P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p00pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P00PfsHa_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x22usize))
        }
    }

    #[doc = "P00%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p00pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P00PfsBy_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x23usize))
        }
    }

    #[doc = "P010 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p010pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P010Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P010Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "P010 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p010pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P010PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P010PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "P010 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p010pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P010PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P010PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(49usize),
            )
        }
    }

    #[doc = "P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p0pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P0Pfs_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x38usize))
        }
    }

    #[doc = "P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p0pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P0PfsHa_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x3ausize))
        }
    }

    #[doc = "P0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p0pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P0PfsBy_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x3busize))
        }
    }

    #[doc = "P100 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p100pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P100Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P100Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "P100 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p100pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P100PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P100PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66usize),
            )
        }
    }

    #[doc = "P100 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p100pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P100PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P100PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(67usize),
            )
        }
    }

    #[doc = "P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p10pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P10Pfs_SPEC, crate::common::RW>,
        7,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x44usize))
        }
    }

    #[doc = "P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p10pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P10PfsHa_SPEC, crate::common::RW>,
        7,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x46usize))
        }
    }

    #[doc = "P10%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p10pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P10PfsBy_SPEC, crate::common::RW>,
        7,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x47usize))
        }
    }

    #[doc = "P108 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p108pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P108Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P108Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[doc = "P108 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p108pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P108PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P108PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(98usize),
            )
        }
    }

    #[doc = "P108 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p108pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P108PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P108PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(99usize),
            )
        }
    }

    #[doc = "P109 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p109pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P109Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P109Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[doc = "P109 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p109pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P109PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P109PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(102usize),
            )
        }
    }

    #[doc = "P109 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p109pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P109PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P109PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(103usize),
            )
        }
    }

    #[doc = "P110 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p110pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P110Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P110Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[doc = "P110 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p110pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P110PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P110PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(106usize),
            )
        }
    }

    #[doc = "P110 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p110pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P110PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P110PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(107usize),
            )
        }
    }

    #[doc = "P1%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p1pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P1Pfs_SPEC, crate::common::RW>,
        5,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6cusize))
        }
    }

    #[doc = "P1%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p1pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P1PfsHa_SPEC, crate::common::RW>,
        5,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6eusize))
        }
    }

    #[doc = "P1%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p1pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P1PfsBy_SPEC, crate::common::RW>,
        5,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x6fusize))
        }
    }

    #[doc = "P200 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p200pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P200Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P200Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[doc = "P200 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p200pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P200PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P200PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(130usize),
            )
        }
    }

    #[doc = "P200 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p200pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P200PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P200PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(131usize),
            )
        }
    }

    #[doc = "P201 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p201pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P201Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P201Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[doc = "P201 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p201pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P201PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P201PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(134usize),
            )
        }
    }

    #[doc = "P201 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p201pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P201PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P201PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(135usize),
            )
        }
    }

    #[doc = "P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p20pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P20Pfs_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x88usize))
        }
    }

    #[doc = "P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p20pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P20PfsHa_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x8ausize))
        }
    }

    #[doc = "P20%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p20pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P20PfsBy_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x8busize))
        }
    }

    #[doc = "P2%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p2pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P2Pfs_SPEC, crate::common::RW>,
        5,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xa8usize))
        }
    }

    #[doc = "P2%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p2pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P2PfsHa_SPEC, crate::common::RW>,
        5,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xaausize))
        }
    }

    #[doc = "P2%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p2pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P2PfsBy_SPEC, crate::common::RW>,
        5,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xabusize))
        }
    }

    #[doc = "P300 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p300pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P300Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P300Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(192usize),
            )
        }
    }

    #[doc = "P300 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p300pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P300PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P300PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(194usize),
            )
        }
    }

    #[doc = "P300 Pin Function Control Register"]
    #[inline(always)]
    pub const fn p300pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P300PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P300PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(195usize),
            )
        }
    }

    #[doc = "P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p30pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P30Pfs_SPEC, crate::common::RW>,
        9,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xc4usize))
        }
    }

    #[doc = "P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p30pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P30PfsHa_SPEC, crate::common::RW>,
        9,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xc6usize))
        }
    }

    #[doc = "P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p30pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P30PfsBy_SPEC, crate::common::RW>,
        9,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xc7usize))
        }
    }

    #[doc = "P3%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p3pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P3Pfs_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xe8usize))
        }
    }

    #[doc = "P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p3pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P3PfsHa_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xeausize))
        }
    }

    #[doc = "P30%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p3pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P3PfsBy_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xebusize))
        }
    }

    #[doc = "P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p40pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P40Pfs_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x100usize))
        }
    }

    #[doc = "P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p40pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P40PfsHa_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x102usize))
        }
    }

    #[doc = "P40%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p40pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P40PfsBy_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x103usize))
        }
    }

    #[doc = "P4%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p4pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P4Pfs_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x128usize))
        }
    }

    #[doc = "P4%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p4pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P4PfsHa_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x12ausize))
        }
    }

    #[doc = "P4%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p4pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P4PfsBy_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x12busize))
        }
    }

    #[doc = "P50%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p50pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P50Pfs_SPEC, crate::common::RW>,
        9,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x140usize))
        }
    }

    #[doc = "P50%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p50pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P50PfsHa_SPEC, crate::common::RW>,
        9,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x142usize))
        }
    }

    #[doc = "P50%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p50pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P50PfsBy_SPEC, crate::common::RW>,
        9,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x143usize))
        }
    }

    #[doc = "P5%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p5pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P5Pfs_SPEC, crate::common::RW>,
        5,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x16cusize))
        }
    }

    #[doc = "P5%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p5pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P5PfsHa_SPEC, crate::common::RW>,
        5,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x16eusize))
        }
    }

    #[doc = "P5%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p5pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P5PfsBy_SPEC, crate::common::RW>,
        5,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x16fusize))
        }
    }

    #[doc = "P60%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p60pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P60Pfs_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x180usize))
        }
    }

    #[doc = "P60%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p60pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P60PfsHa_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x182usize))
        }
    }

    #[doc = "P60%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p60pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P60PfsBy_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x183usize))
        }
    }

    #[doc = "P6%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p6pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P6Pfs_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1a8usize))
        }
    }

    #[doc = "P6%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p6pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P6PfsHa_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1aausize))
        }
    }

    #[doc = "P6%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p6pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P6PfsBy_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1abusize))
        }
    }

    #[doc = "P70%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p70pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P70Pfs_SPEC, crate::common::RW>,
        9,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1c0usize))
        }
    }

    #[doc = "P70%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p70pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P70PfsHa_SPEC, crate::common::RW>,
        9,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1c2usize))
        }
    }

    #[doc = "P70%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p70pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P70PfsBy_SPEC, crate::common::RW>,
        9,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1c3usize))
        }
    }

    #[doc = "P80%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p80pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P80Pfs_SPEC, crate::common::RW>,
        7,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x200usize))
        }
    }

    #[doc = "P80%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p80pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P80PfsHa_SPEC, crate::common::RW>,
        7,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x202usize))
        }
    }

    #[doc = "P80%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p80pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P80PfsBy_SPEC, crate::common::RW>,
        7,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x203usize))
        }
    }

    #[doc = "P90%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p90pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P90Pfs_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x254usize))
        }
    }

    #[doc = "P90%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p90pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P90PfsHa_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x256usize))
        }
    }

    #[doc = "P90%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn p90pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P90PfsBy_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x257usize))
        }
    }

    #[doc = "PA0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn pa0pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pa0Pfs_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2a0usize))
        }
    }

    #[doc = "PA0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn pa0pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pa0PfsHa_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2a2usize))
        }
    }

    #[doc = "PA0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn pa0pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pa0PfsBy_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2a3usize))
        }
    }

    #[doc = "PA10 Pin Function Control Register"]
    #[inline(always)]
    pub const fn pa10pfs(
        &self,
    ) -> &'static crate::common::Reg<self::Pa10Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa10Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(680usize),
            )
        }
    }

    #[doc = "PA10 Pin Function Control Register"]
    #[inline(always)]
    pub const fn pa10pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::Pa10PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa10PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(682usize),
            )
        }
    }

    #[doc = "PA10 Pin Function Control Register"]
    #[inline(always)]
    pub const fn pa10pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::Pa10PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pa10PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(683usize),
            )
        }
    }

    #[doc = "PB0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn pb0pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pb0Pfs_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2c0usize))
        }
    }

    #[doc = "PB0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn pb0pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pb0PfsHa_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2c2usize))
        }
    }

    #[doc = "PB0%s Pin Function Control Register"]
    #[inline(always)]
    pub const fn pb0pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pb0PfsBy_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2c3usize))
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P000Pfs_SPEC;
impl crate::sealed::RegSpec for P000Pfs_SPEC {
    type DataType = u32;
}
#[doc = "P000 Pin Function Control Register"]
pub type P000Pfs = crate::RegValueT<P000Pfs_SPEC>;

impl P000Pfs {
    #[doc = "Port Function SelectThese bits select the peripheral function. For individual pin functions, see the MPC table"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, P000Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, P000Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, p000pfs::Pmr, P000Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,p000pfs::Pmr, P000Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, p000pfs::Asel, P000Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,p000pfs::Asel, P000Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, p000pfs::Isel, P000Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,p000pfs::Isel, P000Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, p000pfs::Dscr, P000Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,p000pfs::Dscr, P000Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, p000pfs::Ncodr, P000Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,p000pfs::Ncodr, P000Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, p000pfs::Pcr, P000Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,p000pfs::Pcr, P000Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, p000pfs::Pdr, P000Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,p000pfs::Pdr, P000Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, p000pfs::Pidr, P000Pfs_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,p000pfs::Pidr, P000Pfs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p000pfs::Podr, P000Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p000pfs::Podr, P000Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P000Pfs {
    #[inline(always)]
    fn default() -> P000Pfs {
        <crate::RegValueT<P000Pfs_SPEC> as RegisterValue<_>>::new(32768)
    }
}
pub mod p000pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Uses the pin as a general I/O pin."]
        pub const _0: Self = Self::new(0);
        #[doc = "Uses the pin as an I/O port for peripheral functions."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Used other than as analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Normal drive output"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive output"]
        pub const _01: Self = Self::new(1);
        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);
        #[doc = "High-drive output"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);
        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables an input pull-up."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (Functions as an input pin.)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (Functions as an output pin.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);
        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P000PfsHa_SPEC;
impl crate::sealed::RegSpec for P000PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "P000 Pin Function Control Register"]
pub type P000PfsHa = crate::RegValueT<P000PfsHa_SPEC>;

impl P000PfsHa {
    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p000pfs_ha::Asel,
        P000PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p000pfs_ha::Asel,
            P000PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p000pfs_ha::Isel,
        P000PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p000pfs_ha::Isel,
            P000PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p000pfs_ha::Dscr,
        P000PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p000pfs_ha::Dscr,
            P000PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p000pfs_ha::Ncodr,
        P000PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p000pfs_ha::Ncodr,
            P000PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p000pfs_ha::Pcr,
        P000PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p000pfs_ha::Pcr,
            P000PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p000pfs_ha::Pdr,
        P000PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p000pfs_ha::Pdr,
            P000PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p000pfs_ha::Pidr,
        P000PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p000pfs_ha::Pidr,
            P000PfsHa_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p000pfs_ha::Podr,
        P000PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p000pfs_ha::Podr,
            P000PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P000PfsHa {
    #[inline(always)]
    fn default() -> P000PfsHa {
        <crate::RegValueT<P000PfsHa_SPEC> as RegisterValue<_>>::new(32768)
    }
}
pub mod p000pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Used other than as analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Normal drive output"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive output"]
        pub const _01: Self = Self::new(1);
        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);
        #[doc = "High-drive output"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);
        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables an input pull-up."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (Functions as an input pin.)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (Functions as an output pin.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);
        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P000PfsBy_SPEC;
impl crate::sealed::RegSpec for P000PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "P000 Pin Function Control Register"]
pub type P000PfsBy = crate::RegValueT<P000PfsBy_SPEC>;

impl P000PfsBy {
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p000pfs_by::Ncodr,
        P000PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p000pfs_by::Ncodr,
            P000PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p000pfs_by::Pcr,
        P000PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p000pfs_by::Pcr,
            P000PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p000pfs_by::Pdr,
        P000PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p000pfs_by::Pdr,
            P000PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p000pfs_by::Pidr,
        P000PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p000pfs_by::Pidr,
            P000PfsBy_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p000pfs_by::Podr,
        P000PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p000pfs_by::Podr,
            P000PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P000PfsBy {
    #[inline(always)]
    fn default() -> P000PfsBy {
        <crate::RegValueT<P000PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p000pfs_by {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);
        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables an input pull-up."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (Functions as an input pin.)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (Functions as an output pin.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);
        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P00Pfs_SPEC;
impl crate::sealed::RegSpec for P00Pfs_SPEC {
    type DataType = u32;
}
#[doc = "P00%s Pin Function Control Register"]
pub type P00Pfs = crate::RegValueT<P00Pfs_SPEC>;

impl NoBitfieldReg<P00Pfs_SPEC> for P00Pfs {}
impl ::core::default::Default for P00Pfs {
    #[inline(always)]
    fn default() -> P00Pfs {
        <crate::RegValueT<P00Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P00PfsHa_SPEC;
impl crate::sealed::RegSpec for P00PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "P00%s Pin Function Control Register"]
pub type P00PfsHa = crate::RegValueT<P00PfsHa_SPEC>;

impl NoBitfieldReg<P00PfsHa_SPEC> for P00PfsHa {}
impl ::core::default::Default for P00PfsHa {
    #[inline(always)]
    fn default() -> P00PfsHa {
        <crate::RegValueT<P00PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P00PfsBy_SPEC;
impl crate::sealed::RegSpec for P00PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "P00%s Pin Function Control Register"]
pub type P00PfsBy = crate::RegValueT<P00PfsBy_SPEC>;

impl NoBitfieldReg<P00PfsBy_SPEC> for P00PfsBy {}
impl ::core::default::Default for P00PfsBy {
    #[inline(always)]
    fn default() -> P00PfsBy {
        <crate::RegValueT<P00PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P010Pfs_SPEC;
impl crate::sealed::RegSpec for P010Pfs_SPEC {
    type DataType = u32;
}
#[doc = "P010 Pin Function Control Register"]
pub type P010Pfs = crate::RegValueT<P010Pfs_SPEC>;

impl NoBitfieldReg<P010Pfs_SPEC> for P010Pfs {}
impl ::core::default::Default for P010Pfs {
    #[inline(always)]
    fn default() -> P010Pfs {
        <crate::RegValueT<P010Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P010PfsHa_SPEC;
impl crate::sealed::RegSpec for P010PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "P010 Pin Function Control Register"]
pub type P010PfsHa = crate::RegValueT<P010PfsHa_SPEC>;

impl NoBitfieldReg<P010PfsHa_SPEC> for P010PfsHa {}
impl ::core::default::Default for P010PfsHa {
    #[inline(always)]
    fn default() -> P010PfsHa {
        <crate::RegValueT<P010PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P010PfsBy_SPEC;
impl crate::sealed::RegSpec for P010PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "P010 Pin Function Control Register"]
pub type P010PfsBy = crate::RegValueT<P010PfsBy_SPEC>;

impl NoBitfieldReg<P010PfsBy_SPEC> for P010PfsBy {}
impl ::core::default::Default for P010PfsBy {
    #[inline(always)]
    fn default() -> P010PfsBy {
        <crate::RegValueT<P010PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0Pfs_SPEC;
impl crate::sealed::RegSpec for P0Pfs_SPEC {
    type DataType = u32;
}
#[doc = "P0%s Pin Function Control Register"]
pub type P0Pfs = crate::RegValueT<P0Pfs_SPEC>;

impl NoBitfieldReg<P0Pfs_SPEC> for P0Pfs {}
impl ::core::default::Default for P0Pfs {
    #[inline(always)]
    fn default() -> P0Pfs {
        <crate::RegValueT<P0Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0PfsHa_SPEC;
impl crate::sealed::RegSpec for P0PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "P0%s Pin Function Control Register"]
pub type P0PfsHa = crate::RegValueT<P0PfsHa_SPEC>;

impl NoBitfieldReg<P0PfsHa_SPEC> for P0PfsHa {}
impl ::core::default::Default for P0PfsHa {
    #[inline(always)]
    fn default() -> P0PfsHa {
        <crate::RegValueT<P0PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0PfsBy_SPEC;
impl crate::sealed::RegSpec for P0PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "P0%s Pin Function Control Register"]
pub type P0PfsBy = crate::RegValueT<P0PfsBy_SPEC>;

impl NoBitfieldReg<P0PfsBy_SPEC> for P0PfsBy {}
impl ::core::default::Default for P0PfsBy {
    #[inline(always)]
    fn default() -> P0PfsBy {
        <crate::RegValueT<P0PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P100Pfs_SPEC;
impl crate::sealed::RegSpec for P100Pfs_SPEC {
    type DataType = u32;
}
#[doc = "P100 Pin Function Control Register"]
pub type P100Pfs = crate::RegValueT<P100Pfs_SPEC>;

impl P100Pfs {
    #[doc = "Port Function SelectThese bits select the peripheral function. For individual pin functions, see the MPC table"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, P100Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, P100Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, p100pfs::Pmr, P100Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,p100pfs::Pmr, P100Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, p100pfs::Asel, P100Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,p100pfs::Asel, P100Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, p100pfs::Isel, P100Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,p100pfs::Isel, P100Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event on Falling"]
    #[inline(always)]
    pub fn eof(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, p100pfs::Eof, P100Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,p100pfs::Eof, P100Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event on Rising"]
    #[inline(always)]
    pub fn eor(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, p100pfs::Eor, P100Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,p100pfs::Eor, P100Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, p100pfs::Dscr, P100Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,p100pfs::Dscr, P100Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, p100pfs::Ncodr, P100Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,p100pfs::Ncodr, P100Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, p100pfs::Pcr, P100Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,p100pfs::Pcr, P100Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, p100pfs::Pdr, P100Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,p100pfs::Pdr, P100Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, p100pfs::Pidr, P100Pfs_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,p100pfs::Pidr, P100Pfs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p100pfs::Podr, P100Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p100pfs::Podr, P100Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P100Pfs {
    #[inline(always)]
    fn default() -> P100Pfs {
        <crate::RegValueT<P100Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p100pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Uses the pin as a general I/O pin."]
        pub const _0: Self = Self::new(0);
        #[doc = "Uses the pin as an I/O port for peripheral functions."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Used other than as analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eof_SPEC;
    pub type Eof = crate::EnumBitfieldStruct<u8, Eof_SPEC>;
    impl Eof {
        #[doc = "Do not care"]
        pub const _0: Self = Self::new(0);
        #[doc = "Detect falling edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eor_SPEC;
    pub type Eor = crate::EnumBitfieldStruct<u8, Eor_SPEC>;
    impl Eor {
        #[doc = "Do not care"]
        pub const _0: Self = Self::new(0);
        #[doc = "Detect rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Normal drive output"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive output"]
        pub const _01: Self = Self::new(1);
        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);
        #[doc = "High-drive output"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);
        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables an input pull-up."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (Functions as an input pin.)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (Functions as an output pin.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);
        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P100PfsHa_SPEC;
impl crate::sealed::RegSpec for P100PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "P100 Pin Function Control Register"]
pub type P100PfsHa = crate::RegValueT<P100PfsHa_SPEC>;

impl P100PfsHa {
    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p100pfs_ha::Asel,
        P100PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p100pfs_ha::Asel,
            P100PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p100pfs_ha::Isel,
        P100PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p100pfs_ha::Isel,
            P100PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Event on Falling"]
    #[inline(always)]
    pub fn eof(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        p100pfs_ha::Eof,
        P100PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            p100pfs_ha::Eof,
            P100PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Event on Rising"]
    #[inline(always)]
    pub fn eor(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        p100pfs_ha::Eor,
        P100PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            p100pfs_ha::Eor,
            P100PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p100pfs_ha::Dscr,
        P100PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p100pfs_ha::Dscr,
            P100PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p100pfs_ha::Ncodr,
        P100PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p100pfs_ha::Ncodr,
            P100PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p100pfs_ha::Pcr,
        P100PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p100pfs_ha::Pcr,
            P100PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p100pfs_ha::Pdr,
        P100PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p100pfs_ha::Pdr,
            P100PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p100pfs_ha::Pidr,
        P100PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p100pfs_ha::Pidr,
            P100PfsHa_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p100pfs_ha::Podr,
        P100PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p100pfs_ha::Podr,
            P100PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P100PfsHa {
    #[inline(always)]
    fn default() -> P100PfsHa {
        <crate::RegValueT<P100PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p100pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Used other than as analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eof_SPEC;
    pub type Eof = crate::EnumBitfieldStruct<u8, Eof_SPEC>;
    impl Eof {
        #[doc = "Do not care"]
        pub const _0: Self = Self::new(0);
        #[doc = "Detect falling edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eor_SPEC;
    pub type Eor = crate::EnumBitfieldStruct<u8, Eor_SPEC>;
    impl Eor {
        #[doc = "Do not care"]
        pub const _0: Self = Self::new(0);
        #[doc = "Detect rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Normal drive output"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive output"]
        pub const _01: Self = Self::new(1);
        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);
        #[doc = "High-drive output"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);
        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables an input pull-up."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (Functions as an input pin.)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (Functions as an output pin.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);
        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P100PfsBy_SPEC;
impl crate::sealed::RegSpec for P100PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "P100 Pin Function Control Register"]
pub type P100PfsBy = crate::RegValueT<P100PfsBy_SPEC>;

impl P100PfsBy {
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p100pfs_by::Ncodr,
        P100PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p100pfs_by::Ncodr,
            P100PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p100pfs_by::Pcr,
        P100PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p100pfs_by::Pcr,
            P100PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p100pfs_by::Pdr,
        P100PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p100pfs_by::Pdr,
            P100PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p100pfs_by::Pidr,
        P100PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p100pfs_by::Pidr,
            P100PfsBy_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p100pfs_by::Podr,
        P100PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p100pfs_by::Podr,
            P100PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P100PfsBy {
    #[inline(always)]
    fn default() -> P100PfsBy {
        <crate::RegValueT<P100PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p100pfs_by {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);
        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables an input pull-up."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (Functions as an input pin.)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (Functions as an output pin.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);
        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P10Pfs_SPEC;
impl crate::sealed::RegSpec for P10Pfs_SPEC {
    type DataType = u32;
}
#[doc = "P10%s Pin Function Control Register"]
pub type P10Pfs = crate::RegValueT<P10Pfs_SPEC>;

impl NoBitfieldReg<P10Pfs_SPEC> for P10Pfs {}
impl ::core::default::Default for P10Pfs {
    #[inline(always)]
    fn default() -> P10Pfs {
        <crate::RegValueT<P10Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P10PfsHa_SPEC;
impl crate::sealed::RegSpec for P10PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "P10%s Pin Function Control Register"]
pub type P10PfsHa = crate::RegValueT<P10PfsHa_SPEC>;

impl NoBitfieldReg<P10PfsHa_SPEC> for P10PfsHa {}
impl ::core::default::Default for P10PfsHa {
    #[inline(always)]
    fn default() -> P10PfsHa {
        <crate::RegValueT<P10PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P10PfsBy_SPEC;
impl crate::sealed::RegSpec for P10PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "P10%s Pin Function Control Register"]
pub type P10PfsBy = crate::RegValueT<P10PfsBy_SPEC>;

impl NoBitfieldReg<P10PfsBy_SPEC> for P10PfsBy {}
impl ::core::default::Default for P10PfsBy {
    #[inline(always)]
    fn default() -> P10PfsBy {
        <crate::RegValueT<P10PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P108Pfs_SPEC;
impl crate::sealed::RegSpec for P108Pfs_SPEC {
    type DataType = u32;
}
#[doc = "P108 Pin Function Control Register"]
pub type P108Pfs = crate::RegValueT<P108Pfs_SPEC>;

impl P108Pfs {
    #[doc = "Port Function SelectThese bits select the peripheral function. For individual pin functions, see the MPC table"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, P108Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, P108Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, p108pfs::Pmr, P108Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,p108pfs::Pmr, P108Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, p108pfs::Asel, P108Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,p108pfs::Asel, P108Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, p108pfs::Isel, P108Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,p108pfs::Isel, P108Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event on Falling"]
    #[inline(always)]
    pub fn eof(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, p108pfs::Eof, P108Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,p108pfs::Eof, P108Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event on Rising"]
    #[inline(always)]
    pub fn eor(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, p108pfs::Eor, P108Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,p108pfs::Eor, P108Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, p108pfs::Dscr, P108Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,p108pfs::Dscr, P108Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, p108pfs::Ncodr, P108Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,p108pfs::Ncodr, P108Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, p108pfs::Pcr, P108Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,p108pfs::Pcr, P108Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, p108pfs::Pdr, P108Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,p108pfs::Pdr, P108Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, p108pfs::Pidr, P108Pfs_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,p108pfs::Pidr, P108Pfs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p108pfs::Podr, P108Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p108pfs::Podr, P108Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P108Pfs {
    #[inline(always)]
    fn default() -> P108Pfs {
        <crate::RegValueT<P108Pfs_SPEC> as RegisterValue<_>>::new(66576)
    }
}
pub mod p108pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Uses the pin as a general I/O pin."]
        pub const _0: Self = Self::new(0);
        #[doc = "Uses the pin as an I/O port for peripheral functions."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Used other than as analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eof_SPEC;
    pub type Eof = crate::EnumBitfieldStruct<u8, Eof_SPEC>;
    impl Eof {
        #[doc = "Do not care"]
        pub const _0: Self = Self::new(0);
        #[doc = "Detect falling edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eor_SPEC;
    pub type Eor = crate::EnumBitfieldStruct<u8, Eor_SPEC>;
    impl Eor {
        #[doc = "Do not care"]
        pub const _0: Self = Self::new(0);
        #[doc = "Detect rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Normal drive output"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive output"]
        pub const _01: Self = Self::new(1);
        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);
        #[doc = "High-drive output"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);
        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables an input pull-up."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (Functions as an input pin.)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (Functions as an output pin.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);
        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P108PfsHa_SPEC;
impl crate::sealed::RegSpec for P108PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "P108 Pin Function Control Register"]
pub type P108PfsHa = crate::RegValueT<P108PfsHa_SPEC>;

impl P108PfsHa {
    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p108pfs_ha::Asel,
        P108PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p108pfs_ha::Asel,
            P108PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p108pfs_ha::Isel,
        P108PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p108pfs_ha::Isel,
            P108PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Event on Falling"]
    #[inline(always)]
    pub fn eof(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        p108pfs_ha::Eof,
        P108PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            p108pfs_ha::Eof,
            P108PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Event on Rising"]
    #[inline(always)]
    pub fn eor(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        p108pfs_ha::Eor,
        P108PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            p108pfs_ha::Eor,
            P108PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p108pfs_ha::Dscr,
        P108PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p108pfs_ha::Dscr,
            P108PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p108pfs_ha::Ncodr,
        P108PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p108pfs_ha::Ncodr,
            P108PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p108pfs_ha::Pcr,
        P108PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p108pfs_ha::Pcr,
            P108PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p108pfs_ha::Pdr,
        P108PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p108pfs_ha::Pdr,
            P108PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p108pfs_ha::Pidr,
        P108PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p108pfs_ha::Pidr,
            P108PfsHa_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p108pfs_ha::Podr,
        P108PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p108pfs_ha::Podr,
            P108PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P108PfsHa {
    #[inline(always)]
    fn default() -> P108PfsHa {
        <crate::RegValueT<P108PfsHa_SPEC> as RegisterValue<_>>::new(1040)
    }
}
pub mod p108pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Used other than as analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eof_SPEC;
    pub type Eof = crate::EnumBitfieldStruct<u8, Eof_SPEC>;
    impl Eof {
        #[doc = "Do not care"]
        pub const _0: Self = Self::new(0);
        #[doc = "Detect falling edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eor_SPEC;
    pub type Eor = crate::EnumBitfieldStruct<u8, Eor_SPEC>;
    impl Eor {
        #[doc = "Do not care"]
        pub const _0: Self = Self::new(0);
        #[doc = "Detect rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Normal drive output"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive output"]
        pub const _01: Self = Self::new(1);
        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);
        #[doc = "High-drive output"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);
        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables an input pull-up."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (Functions as an input pin.)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (Functions as an output pin.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);
        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P108PfsBy_SPEC;
impl crate::sealed::RegSpec for P108PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "P108 Pin Function Control Register"]
pub type P108PfsBy = crate::RegValueT<P108PfsBy_SPEC>;

impl P108PfsBy {
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p108pfs_by::Ncodr,
        P108PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p108pfs_by::Ncodr,
            P108PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p108pfs_by::Pcr,
        P108PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p108pfs_by::Pcr,
            P108PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p108pfs_by::Pdr,
        P108PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p108pfs_by::Pdr,
            P108PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p108pfs_by::Pidr,
        P108PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p108pfs_by::Pidr,
            P108PfsBy_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p108pfs_by::Podr,
        P108PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p108pfs_by::Podr,
            P108PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P108PfsBy {
    #[inline(always)]
    fn default() -> P108PfsBy {
        <crate::RegValueT<P108PfsBy_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod p108pfs_by {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);
        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables an input pull-up."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (Functions as an input pin.)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (Functions as an output pin.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);
        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P109Pfs_SPEC;
impl crate::sealed::RegSpec for P109Pfs_SPEC {
    type DataType = u32;
}
#[doc = "P109 Pin Function Control Register"]
pub type P109Pfs = crate::RegValueT<P109Pfs_SPEC>;

impl NoBitfieldReg<P109Pfs_SPEC> for P109Pfs {}
impl ::core::default::Default for P109Pfs {
    #[inline(always)]
    fn default() -> P109Pfs {
        <crate::RegValueT<P109Pfs_SPEC> as RegisterValue<_>>::new(66576)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P109PfsHa_SPEC;
impl crate::sealed::RegSpec for P109PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "P109 Pin Function Control Register"]
pub type P109PfsHa = crate::RegValueT<P109PfsHa_SPEC>;

impl NoBitfieldReg<P109PfsHa_SPEC> for P109PfsHa {}
impl ::core::default::Default for P109PfsHa {
    #[inline(always)]
    fn default() -> P109PfsHa {
        <crate::RegValueT<P109PfsHa_SPEC> as RegisterValue<_>>::new(1040)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P109PfsBy_SPEC;
impl crate::sealed::RegSpec for P109PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "P109 Pin Function Control Register"]
pub type P109PfsBy = crate::RegValueT<P109PfsBy_SPEC>;

impl NoBitfieldReg<P109PfsBy_SPEC> for P109PfsBy {}
impl ::core::default::Default for P109PfsBy {
    #[inline(always)]
    fn default() -> P109PfsBy {
        <crate::RegValueT<P109PfsBy_SPEC> as RegisterValue<_>>::new(16)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P110Pfs_SPEC;
impl crate::sealed::RegSpec for P110Pfs_SPEC {
    type DataType = u32;
}
#[doc = "P110 Pin Function Control Register"]
pub type P110Pfs = crate::RegValueT<P110Pfs_SPEC>;

impl P110Pfs {
    #[doc = "Port Function SelectThese bits select the peripheral function. For individual pin functions, see the MPC table"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, P110Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, P110Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, p110pfs::Pmr, P110Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,p110pfs::Pmr, P110Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, p110pfs::Asel, P110Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,p110pfs::Asel, P110Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, p110pfs::Isel, P110Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,p110pfs::Isel, P110Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event on Falling"]
    #[inline(always)]
    pub fn eof(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, p110pfs::Eof, P110Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,p110pfs::Eof, P110Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event on Rising"]
    #[inline(always)]
    pub fn eor(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, p110pfs::Eor, P110Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,p110pfs::Eor, P110Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, p110pfs::Dscr, P110Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,p110pfs::Dscr, P110Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, p110pfs::Ncodr, P110Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,p110pfs::Ncodr, P110Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, p110pfs::Pcr, P110Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,p110pfs::Pcr, P110Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, p110pfs::Pdr, P110Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,p110pfs::Pdr, P110Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, p110pfs::Pidr, P110Pfs_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,p110pfs::Pidr, P110Pfs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p110pfs::Podr, P110Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p110pfs::Podr, P110Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P110Pfs {
    #[inline(always)]
    fn default() -> P110Pfs {
        <crate::RegValueT<P110Pfs_SPEC> as RegisterValue<_>>::new(65552)
    }
}
pub mod p110pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Uses the pin as a general I/O pin."]
        pub const _0: Self = Self::new(0);
        #[doc = "Uses the pin as an I/O port for peripheral functions."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Used other than as analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eof_SPEC;
    pub type Eof = crate::EnumBitfieldStruct<u8, Eof_SPEC>;
    impl Eof {
        #[doc = "Do not care"]
        pub const _0: Self = Self::new(0);
        #[doc = "Detect falling edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eor_SPEC;
    pub type Eor = crate::EnumBitfieldStruct<u8, Eor_SPEC>;
    impl Eor {
        #[doc = "Do not care"]
        pub const _0: Self = Self::new(0);
        #[doc = "Detect rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Normal drive output"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive output"]
        pub const _01: Self = Self::new(1);
        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);
        #[doc = "High-drive output"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);
        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables an input pull-up."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (Functions as an input pin.)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (Functions as an output pin.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);
        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P110PfsHa_SPEC;
impl crate::sealed::RegSpec for P110PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "P110 Pin Function Control Register"]
pub type P110PfsHa = crate::RegValueT<P110PfsHa_SPEC>;

impl P110PfsHa {
    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p110pfs_ha::Asel,
        P110PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p110pfs_ha::Asel,
            P110PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p110pfs_ha::Isel,
        P110PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p110pfs_ha::Isel,
            P110PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Event on Falling"]
    #[inline(always)]
    pub fn eof(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        p110pfs_ha::Eof,
        P110PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            p110pfs_ha::Eof,
            P110PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Event on Rising"]
    #[inline(always)]
    pub fn eor(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        p110pfs_ha::Eor,
        P110PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            p110pfs_ha::Eor,
            P110PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p110pfs_ha::Dscr,
        P110PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p110pfs_ha::Dscr,
            P110PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p110pfs_ha::Ncodr,
        P110PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p110pfs_ha::Ncodr,
            P110PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p110pfs_ha::Pcr,
        P110PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p110pfs_ha::Pcr,
            P110PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p110pfs_ha::Pdr,
        P110PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p110pfs_ha::Pdr,
            P110PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p110pfs_ha::Pidr,
        P110PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p110pfs_ha::Pidr,
            P110PfsHa_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p110pfs_ha::Podr,
        P110PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p110pfs_ha::Podr,
            P110PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P110PfsHa {
    #[inline(always)]
    fn default() -> P110PfsHa {
        <crate::RegValueT<P110PfsHa_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod p110pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Used other than as analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eof_SPEC;
    pub type Eof = crate::EnumBitfieldStruct<u8, Eof_SPEC>;
    impl Eof {
        #[doc = "Do not care"]
        pub const _0: Self = Self::new(0);
        #[doc = "Detect falling edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eor_SPEC;
    pub type Eor = crate::EnumBitfieldStruct<u8, Eor_SPEC>;
    impl Eor {
        #[doc = "Do not care"]
        pub const _0: Self = Self::new(0);
        #[doc = "Detect rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Normal drive output"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive output"]
        pub const _01: Self = Self::new(1);
        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);
        #[doc = "High-drive output"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);
        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables an input pull-up."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (Functions as an input pin.)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (Functions as an output pin.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);
        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P110PfsBy_SPEC;
impl crate::sealed::RegSpec for P110PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "P110 Pin Function Control Register"]
pub type P110PfsBy = crate::RegValueT<P110PfsBy_SPEC>;

impl P110PfsBy {
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p110pfs_by::Ncodr,
        P110PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p110pfs_by::Ncodr,
            P110PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p110pfs_by::Pcr,
        P110PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p110pfs_by::Pcr,
            P110PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p110pfs_by::Pdr,
        P110PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p110pfs_by::Pdr,
            P110PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p110pfs_by::Pidr,
        P110PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p110pfs_by::Pidr,
            P110PfsBy_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p110pfs_by::Podr,
        P110PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p110pfs_by::Podr,
            P110PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P110PfsBy {
    #[inline(always)]
    fn default() -> P110PfsBy {
        <crate::RegValueT<P110PfsBy_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod p110pfs_by {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);
        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables an input pull-up."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (Functions as an input pin.)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (Functions as an output pin.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);
        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1Pfs_SPEC;
impl crate::sealed::RegSpec for P1Pfs_SPEC {
    type DataType = u32;
}
#[doc = "P1%s Pin Function Control Register"]
pub type P1Pfs = crate::RegValueT<P1Pfs_SPEC>;

impl NoBitfieldReg<P1Pfs_SPEC> for P1Pfs {}
impl ::core::default::Default for P1Pfs {
    #[inline(always)]
    fn default() -> P1Pfs {
        <crate::RegValueT<P1Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1PfsHa_SPEC;
impl crate::sealed::RegSpec for P1PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "P1%s Pin Function Control Register"]
pub type P1PfsHa = crate::RegValueT<P1PfsHa_SPEC>;

impl NoBitfieldReg<P1PfsHa_SPEC> for P1PfsHa {}
impl ::core::default::Default for P1PfsHa {
    #[inline(always)]
    fn default() -> P1PfsHa {
        <crate::RegValueT<P1PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1PfsBy_SPEC;
impl crate::sealed::RegSpec for P1PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "P1%s Pin Function Control Register"]
pub type P1PfsBy = crate::RegValueT<P1PfsBy_SPEC>;

impl NoBitfieldReg<P1PfsBy_SPEC> for P1PfsBy {}
impl ::core::default::Default for P1PfsBy {
    #[inline(always)]
    fn default() -> P1PfsBy {
        <crate::RegValueT<P1PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P200Pfs_SPEC;
impl crate::sealed::RegSpec for P200Pfs_SPEC {
    type DataType = u32;
}
#[doc = "P200 Pin Function Control Register"]
pub type P200Pfs = crate::RegValueT<P200Pfs_SPEC>;

impl NoBitfieldReg<P200Pfs_SPEC> for P200Pfs {}
impl ::core::default::Default for P200Pfs {
    #[inline(always)]
    fn default() -> P200Pfs {
        <crate::RegValueT<P200Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P200PfsHa_SPEC;
impl crate::sealed::RegSpec for P200PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "P200 Pin Function Control Register"]
pub type P200PfsHa = crate::RegValueT<P200PfsHa_SPEC>;

impl NoBitfieldReg<P200PfsHa_SPEC> for P200PfsHa {}
impl ::core::default::Default for P200PfsHa {
    #[inline(always)]
    fn default() -> P200PfsHa {
        <crate::RegValueT<P200PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P200PfsBy_SPEC;
impl crate::sealed::RegSpec for P200PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "P200 Pin Function Control Register"]
pub type P200PfsBy = crate::RegValueT<P200PfsBy_SPEC>;

impl NoBitfieldReg<P200PfsBy_SPEC> for P200PfsBy {}
impl ::core::default::Default for P200PfsBy {
    #[inline(always)]
    fn default() -> P200PfsBy {
        <crate::RegValueT<P200PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P201Pfs_SPEC;
impl crate::sealed::RegSpec for P201Pfs_SPEC {
    type DataType = u32;
}
#[doc = "P201 Pin Function Control Register"]
pub type P201Pfs = crate::RegValueT<P201Pfs_SPEC>;

impl P201Pfs {
    #[doc = "Port Function SelectThese bits select the peripheral function. For individual pin functions, see the MPC table"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, P201Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, P201Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, p201pfs::Pmr, P201Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,p201pfs::Pmr, P201Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, p201pfs::Asel, P201Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,p201pfs::Asel, P201Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, p201pfs::Isel, P201Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,p201pfs::Isel, P201Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event on Falling"]
    #[inline(always)]
    pub fn eof(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, p201pfs::Eof, P201Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,p201pfs::Eof, P201Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event on Rising"]
    #[inline(always)]
    pub fn eor(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, p201pfs::Eor, P201Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,p201pfs::Eor, P201Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, p201pfs::Dscr, P201Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,p201pfs::Dscr, P201Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, p201pfs::Ncodr, P201Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,p201pfs::Ncodr, P201Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, p201pfs::Pcr, P201Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,p201pfs::Pcr, P201Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, p201pfs::Pdr, P201Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,p201pfs::Pdr, P201Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, p201pfs::Pidr, P201Pfs_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,p201pfs::Pidr, P201Pfs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p201pfs::Podr, P201Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p201pfs::Podr, P201Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P201Pfs {
    #[inline(always)]
    fn default() -> P201Pfs {
        <crate::RegValueT<P201Pfs_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod p201pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Uses the pin as a general I/O pin."]
        pub const _0: Self = Self::new(0);
        #[doc = "Uses the pin as an I/O port for peripheral functions."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Used other than as analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eof_SPEC;
    pub type Eof = crate::EnumBitfieldStruct<u8, Eof_SPEC>;
    impl Eof {
        #[doc = "Do not care"]
        pub const _0: Self = Self::new(0);
        #[doc = "Detect falling edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eor_SPEC;
    pub type Eor = crate::EnumBitfieldStruct<u8, Eor_SPEC>;
    impl Eor {
        #[doc = "Do not care"]
        pub const _0: Self = Self::new(0);
        #[doc = "Detect rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Normal drive output"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive output"]
        pub const _01: Self = Self::new(1);
        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);
        #[doc = "High-drive output"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);
        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables an input pull-up."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (Functions as an input pin.)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (Functions as an output pin.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);
        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P201PfsHa_SPEC;
impl crate::sealed::RegSpec for P201PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "P201 Pin Function Control Register"]
pub type P201PfsHa = crate::RegValueT<P201PfsHa_SPEC>;

impl P201PfsHa {
    #[doc = "Analog Input enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        p201pfs_ha::Asel,
        P201PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            p201pfs_ha::Asel,
            P201PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "IRQ input enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        p201pfs_ha::Isel,
        P201PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            p201pfs_ha::Isel,
            P201PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Event on Falling"]
    #[inline(always)]
    pub fn eof(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        p201pfs_ha::Eof,
        P201PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            p201pfs_ha::Eof,
            P201PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Event on Rising"]
    #[inline(always)]
    pub fn eor(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        p201pfs_ha::Eor,
        P201PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            p201pfs_ha::Eor,
            P201PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Drive Strength Control Register"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        p201pfs_ha::Dscr,
        P201PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            p201pfs_ha::Dscr,
            P201PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p201pfs_ha::Ncodr,
        P201PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p201pfs_ha::Ncodr,
            P201PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p201pfs_ha::Pcr,
        P201PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p201pfs_ha::Pcr,
            P201PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p201pfs_ha::Pdr,
        P201PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p201pfs_ha::Pdr,
            P201PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p201pfs_ha::Pidr,
        P201PfsHa_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p201pfs_ha::Pidr,
            P201PfsHa_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p201pfs_ha::Podr,
        P201PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p201pfs_ha::Podr,
            P201PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P201PfsHa {
    #[inline(always)]
    fn default() -> P201PfsHa {
        <crate::RegValueT<P201PfsHa_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod p201pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Used other than as analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eof_SPEC;
    pub type Eof = crate::EnumBitfieldStruct<u8, Eof_SPEC>;
    impl Eof {
        #[doc = "Do not care"]
        pub const _0: Self = Self::new(0);
        #[doc = "Detect falling edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eor_SPEC;
    pub type Eor = crate::EnumBitfieldStruct<u8, Eor_SPEC>;
    impl Eor {
        #[doc = "Do not care"]
        pub const _0: Self = Self::new(0);
        #[doc = "Detect rising edge"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Normal drive output"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive output"]
        pub const _01: Self = Self::new(1);
        #[doc = "Setting prohibited"]
        pub const _10: Self = Self::new(2);
        #[doc = "High-drive output"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);
        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables an input pull-up."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (Functions as an input pin.)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (Functions as an output pin.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);
        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P201PfsBy_SPEC;
impl crate::sealed::RegSpec for P201PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "P201 Pin Function Control Register"]
pub type P201PfsBy = crate::RegValueT<P201PfsBy_SPEC>;

impl P201PfsBy {
    #[doc = "N-Channel Open Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        p201pfs_by::Ncodr,
        P201PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            p201pfs_by::Ncodr,
            P201PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        p201pfs_by::Pcr,
        P201PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            p201pfs_by::Pcr,
            P201PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        p201pfs_by::Pdr,
        P201PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            p201pfs_by::Pdr,
            P201PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Port Input Data"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        p201pfs_by::Pidr,
        P201PfsBy_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            p201pfs_by::Pidr,
            P201PfsBy_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p201pfs_by::Podr,
        P201PfsBy_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p201pfs_by::Podr,
            P201PfsBy_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P201PfsBy {
    #[inline(always)]
    fn default() -> P201PfsBy {
        <crate::RegValueT<P201PfsBy_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod p201pfs_by {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ncodr_SPEC;
    pub type Ncodr = crate::EnumBitfieldStruct<u8, Ncodr_SPEC>;
    impl Ncodr {
        #[doc = "CMOS output"]
        pub const _0: Self = Self::new(0);
        #[doc = "NMOS open-drain output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disables an input pull-up."]
        pub const _0: Self = Self::new(0);
        #[doc = "Enables an input pull-up."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (Functions as an input pin.)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (Functions as an output pin.)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low input"]
        pub const _0: Self = Self::new(0);
        #[doc = "High input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P20Pfs_SPEC;
impl crate::sealed::RegSpec for P20Pfs_SPEC {
    type DataType = u32;
}
#[doc = "P20%s Pin Function Control Register"]
pub type P20Pfs = crate::RegValueT<P20Pfs_SPEC>;

impl NoBitfieldReg<P20Pfs_SPEC> for P20Pfs {}
impl ::core::default::Default for P20Pfs {
    #[inline(always)]
    fn default() -> P20Pfs {
        <crate::RegValueT<P20Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P20PfsHa_SPEC;
impl crate::sealed::RegSpec for P20PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "P20%s Pin Function Control Register"]
pub type P20PfsHa = crate::RegValueT<P20PfsHa_SPEC>;

impl NoBitfieldReg<P20PfsHa_SPEC> for P20PfsHa {}
impl ::core::default::Default for P20PfsHa {
    #[inline(always)]
    fn default() -> P20PfsHa {
        <crate::RegValueT<P20PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P20PfsBy_SPEC;
impl crate::sealed::RegSpec for P20PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "P20%s Pin Function Control Register"]
pub type P20PfsBy = crate::RegValueT<P20PfsBy_SPEC>;

impl NoBitfieldReg<P20PfsBy_SPEC> for P20PfsBy {}
impl ::core::default::Default for P20PfsBy {
    #[inline(always)]
    fn default() -> P20PfsBy {
        <crate::RegValueT<P20PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2Pfs_SPEC;
impl crate::sealed::RegSpec for P2Pfs_SPEC {
    type DataType = u32;
}
#[doc = "P2%s Pin Function Control Register"]
pub type P2Pfs = crate::RegValueT<P2Pfs_SPEC>;

impl NoBitfieldReg<P2Pfs_SPEC> for P2Pfs {}
impl ::core::default::Default for P2Pfs {
    #[inline(always)]
    fn default() -> P2Pfs {
        <crate::RegValueT<P2Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2PfsHa_SPEC;
impl crate::sealed::RegSpec for P2PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "P2%s Pin Function Control Register"]
pub type P2PfsHa = crate::RegValueT<P2PfsHa_SPEC>;

impl NoBitfieldReg<P2PfsHa_SPEC> for P2PfsHa {}
impl ::core::default::Default for P2PfsHa {
    #[inline(always)]
    fn default() -> P2PfsHa {
        <crate::RegValueT<P2PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2PfsBy_SPEC;
impl crate::sealed::RegSpec for P2PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "P2%s Pin Function Control Register"]
pub type P2PfsBy = crate::RegValueT<P2PfsBy_SPEC>;

impl NoBitfieldReg<P2PfsBy_SPEC> for P2PfsBy {}
impl ::core::default::Default for P2PfsBy {
    #[inline(always)]
    fn default() -> P2PfsBy {
        <crate::RegValueT<P2PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P300Pfs_SPEC;
impl crate::sealed::RegSpec for P300Pfs_SPEC {
    type DataType = u32;
}
#[doc = "P300 Pin Function Control Register"]
pub type P300Pfs = crate::RegValueT<P300Pfs_SPEC>;

impl NoBitfieldReg<P300Pfs_SPEC> for P300Pfs {}
impl ::core::default::Default for P300Pfs {
    #[inline(always)]
    fn default() -> P300Pfs {
        <crate::RegValueT<P300Pfs_SPEC> as RegisterValue<_>>::new(65552)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P300PfsHa_SPEC;
impl crate::sealed::RegSpec for P300PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "P300 Pin Function Control Register"]
pub type P300PfsHa = crate::RegValueT<P300PfsHa_SPEC>;

impl NoBitfieldReg<P300PfsHa_SPEC> for P300PfsHa {}
impl ::core::default::Default for P300PfsHa {
    #[inline(always)]
    fn default() -> P300PfsHa {
        <crate::RegValueT<P300PfsHa_SPEC> as RegisterValue<_>>::new(16)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P300PfsBy_SPEC;
impl crate::sealed::RegSpec for P300PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "P300 Pin Function Control Register"]
pub type P300PfsBy = crate::RegValueT<P300PfsBy_SPEC>;

impl NoBitfieldReg<P300PfsBy_SPEC> for P300PfsBy {}
impl ::core::default::Default for P300PfsBy {
    #[inline(always)]
    fn default() -> P300PfsBy {
        <crate::RegValueT<P300PfsBy_SPEC> as RegisterValue<_>>::new(16)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P30Pfs_SPEC;
impl crate::sealed::RegSpec for P30Pfs_SPEC {
    type DataType = u32;
}
#[doc = "P30%s Pin Function Control Register"]
pub type P30Pfs = crate::RegValueT<P30Pfs_SPEC>;

impl NoBitfieldReg<P30Pfs_SPEC> for P30Pfs {}
impl ::core::default::Default for P30Pfs {
    #[inline(always)]
    fn default() -> P30Pfs {
        <crate::RegValueT<P30Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P30PfsHa_SPEC;
impl crate::sealed::RegSpec for P30PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "P30%s Pin Function Control Register"]
pub type P30PfsHa = crate::RegValueT<P30PfsHa_SPEC>;

impl NoBitfieldReg<P30PfsHa_SPEC> for P30PfsHa {}
impl ::core::default::Default for P30PfsHa {
    #[inline(always)]
    fn default() -> P30PfsHa {
        <crate::RegValueT<P30PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P30PfsBy_SPEC;
impl crate::sealed::RegSpec for P30PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "P30%s Pin Function Control Register"]
pub type P30PfsBy = crate::RegValueT<P30PfsBy_SPEC>;

impl NoBitfieldReg<P30PfsBy_SPEC> for P30PfsBy {}
impl ::core::default::Default for P30PfsBy {
    #[inline(always)]
    fn default() -> P30PfsBy {
        <crate::RegValueT<P30PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P3Pfs_SPEC;
impl crate::sealed::RegSpec for P3Pfs_SPEC {
    type DataType = u32;
}
#[doc = "P3%s Pin Function Control Register"]
pub type P3Pfs = crate::RegValueT<P3Pfs_SPEC>;

impl NoBitfieldReg<P3Pfs_SPEC> for P3Pfs {}
impl ::core::default::Default for P3Pfs {
    #[inline(always)]
    fn default() -> P3Pfs {
        <crate::RegValueT<P3Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P3PfsHa_SPEC;
impl crate::sealed::RegSpec for P3PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "P30%s Pin Function Control Register"]
pub type P3PfsHa = crate::RegValueT<P3PfsHa_SPEC>;

impl NoBitfieldReg<P3PfsHa_SPEC> for P3PfsHa {}
impl ::core::default::Default for P3PfsHa {
    #[inline(always)]
    fn default() -> P3PfsHa {
        <crate::RegValueT<P3PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P3PfsBy_SPEC;
impl crate::sealed::RegSpec for P3PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "P30%s Pin Function Control Register"]
pub type P3PfsBy = crate::RegValueT<P3PfsBy_SPEC>;

impl NoBitfieldReg<P3PfsBy_SPEC> for P3PfsBy {}
impl ::core::default::Default for P3PfsBy {
    #[inline(always)]
    fn default() -> P3PfsBy {
        <crate::RegValueT<P3PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P40Pfs_SPEC;
impl crate::sealed::RegSpec for P40Pfs_SPEC {
    type DataType = u32;
}
#[doc = "P40%s Pin Function Control Register"]
pub type P40Pfs = crate::RegValueT<P40Pfs_SPEC>;

impl NoBitfieldReg<P40Pfs_SPEC> for P40Pfs {}
impl ::core::default::Default for P40Pfs {
    #[inline(always)]
    fn default() -> P40Pfs {
        <crate::RegValueT<P40Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P40PfsHa_SPEC;
impl crate::sealed::RegSpec for P40PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "P40%s Pin Function Control Register"]
pub type P40PfsHa = crate::RegValueT<P40PfsHa_SPEC>;

impl NoBitfieldReg<P40PfsHa_SPEC> for P40PfsHa {}
impl ::core::default::Default for P40PfsHa {
    #[inline(always)]
    fn default() -> P40PfsHa {
        <crate::RegValueT<P40PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P40PfsBy_SPEC;
impl crate::sealed::RegSpec for P40PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "P40%s Pin Function Control Register"]
pub type P40PfsBy = crate::RegValueT<P40PfsBy_SPEC>;

impl NoBitfieldReg<P40PfsBy_SPEC> for P40PfsBy {}
impl ::core::default::Default for P40PfsBy {
    #[inline(always)]
    fn default() -> P40PfsBy {
        <crate::RegValueT<P40PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P4Pfs_SPEC;
impl crate::sealed::RegSpec for P4Pfs_SPEC {
    type DataType = u32;
}
#[doc = "P4%s Pin Function Control Register"]
pub type P4Pfs = crate::RegValueT<P4Pfs_SPEC>;

impl NoBitfieldReg<P4Pfs_SPEC> for P4Pfs {}
impl ::core::default::Default for P4Pfs {
    #[inline(always)]
    fn default() -> P4Pfs {
        <crate::RegValueT<P4Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P4PfsHa_SPEC;
impl crate::sealed::RegSpec for P4PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "P4%s Pin Function Control Register"]
pub type P4PfsHa = crate::RegValueT<P4PfsHa_SPEC>;

impl NoBitfieldReg<P4PfsHa_SPEC> for P4PfsHa {}
impl ::core::default::Default for P4PfsHa {
    #[inline(always)]
    fn default() -> P4PfsHa {
        <crate::RegValueT<P4PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P4PfsBy_SPEC;
impl crate::sealed::RegSpec for P4PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "P4%s Pin Function Control Register"]
pub type P4PfsBy = crate::RegValueT<P4PfsBy_SPEC>;

impl NoBitfieldReg<P4PfsBy_SPEC> for P4PfsBy {}
impl ::core::default::Default for P4PfsBy {
    #[inline(always)]
    fn default() -> P4PfsBy {
        <crate::RegValueT<P4PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P50Pfs_SPEC;
impl crate::sealed::RegSpec for P50Pfs_SPEC {
    type DataType = u32;
}
#[doc = "P50%s Pin Function Control Register"]
pub type P50Pfs = crate::RegValueT<P50Pfs_SPEC>;

impl NoBitfieldReg<P50Pfs_SPEC> for P50Pfs {}
impl ::core::default::Default for P50Pfs {
    #[inline(always)]
    fn default() -> P50Pfs {
        <crate::RegValueT<P50Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P50PfsHa_SPEC;
impl crate::sealed::RegSpec for P50PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "P50%s Pin Function Control Register"]
pub type P50PfsHa = crate::RegValueT<P50PfsHa_SPEC>;

impl NoBitfieldReg<P50PfsHa_SPEC> for P50PfsHa {}
impl ::core::default::Default for P50PfsHa {
    #[inline(always)]
    fn default() -> P50PfsHa {
        <crate::RegValueT<P50PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P50PfsBy_SPEC;
impl crate::sealed::RegSpec for P50PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "P50%s Pin Function Control Register"]
pub type P50PfsBy = crate::RegValueT<P50PfsBy_SPEC>;

impl NoBitfieldReg<P50PfsBy_SPEC> for P50PfsBy {}
impl ::core::default::Default for P50PfsBy {
    #[inline(always)]
    fn default() -> P50PfsBy {
        <crate::RegValueT<P50PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P5Pfs_SPEC;
impl crate::sealed::RegSpec for P5Pfs_SPEC {
    type DataType = u32;
}
#[doc = "P5%s Pin Function Control Register"]
pub type P5Pfs = crate::RegValueT<P5Pfs_SPEC>;

impl NoBitfieldReg<P5Pfs_SPEC> for P5Pfs {}
impl ::core::default::Default for P5Pfs {
    #[inline(always)]
    fn default() -> P5Pfs {
        <crate::RegValueT<P5Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P5PfsHa_SPEC;
impl crate::sealed::RegSpec for P5PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "P5%s Pin Function Control Register"]
pub type P5PfsHa = crate::RegValueT<P5PfsHa_SPEC>;

impl NoBitfieldReg<P5PfsHa_SPEC> for P5PfsHa {}
impl ::core::default::Default for P5PfsHa {
    #[inline(always)]
    fn default() -> P5PfsHa {
        <crate::RegValueT<P5PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P5PfsBy_SPEC;
impl crate::sealed::RegSpec for P5PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "P5%s Pin Function Control Register"]
pub type P5PfsBy = crate::RegValueT<P5PfsBy_SPEC>;

impl NoBitfieldReg<P5PfsBy_SPEC> for P5PfsBy {}
impl ::core::default::Default for P5PfsBy {
    #[inline(always)]
    fn default() -> P5PfsBy {
        <crate::RegValueT<P5PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P60Pfs_SPEC;
impl crate::sealed::RegSpec for P60Pfs_SPEC {
    type DataType = u32;
}
#[doc = "P60%s Pin Function Control Register"]
pub type P60Pfs = crate::RegValueT<P60Pfs_SPEC>;

impl NoBitfieldReg<P60Pfs_SPEC> for P60Pfs {}
impl ::core::default::Default for P60Pfs {
    #[inline(always)]
    fn default() -> P60Pfs {
        <crate::RegValueT<P60Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P60PfsHa_SPEC;
impl crate::sealed::RegSpec for P60PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "P60%s Pin Function Control Register"]
pub type P60PfsHa = crate::RegValueT<P60PfsHa_SPEC>;

impl NoBitfieldReg<P60PfsHa_SPEC> for P60PfsHa {}
impl ::core::default::Default for P60PfsHa {
    #[inline(always)]
    fn default() -> P60PfsHa {
        <crate::RegValueT<P60PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P60PfsBy_SPEC;
impl crate::sealed::RegSpec for P60PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "P60%s Pin Function Control Register"]
pub type P60PfsBy = crate::RegValueT<P60PfsBy_SPEC>;

impl NoBitfieldReg<P60PfsBy_SPEC> for P60PfsBy {}
impl ::core::default::Default for P60PfsBy {
    #[inline(always)]
    fn default() -> P60PfsBy {
        <crate::RegValueT<P60PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P6Pfs_SPEC;
impl crate::sealed::RegSpec for P6Pfs_SPEC {
    type DataType = u32;
}
#[doc = "P6%s Pin Function Control Register"]
pub type P6Pfs = crate::RegValueT<P6Pfs_SPEC>;

impl NoBitfieldReg<P6Pfs_SPEC> for P6Pfs {}
impl ::core::default::Default for P6Pfs {
    #[inline(always)]
    fn default() -> P6Pfs {
        <crate::RegValueT<P6Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P6PfsHa_SPEC;
impl crate::sealed::RegSpec for P6PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "P6%s Pin Function Control Register"]
pub type P6PfsHa = crate::RegValueT<P6PfsHa_SPEC>;

impl NoBitfieldReg<P6PfsHa_SPEC> for P6PfsHa {}
impl ::core::default::Default for P6PfsHa {
    #[inline(always)]
    fn default() -> P6PfsHa {
        <crate::RegValueT<P6PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P6PfsBy_SPEC;
impl crate::sealed::RegSpec for P6PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "P6%s Pin Function Control Register"]
pub type P6PfsBy = crate::RegValueT<P6PfsBy_SPEC>;

impl NoBitfieldReg<P6PfsBy_SPEC> for P6PfsBy {}
impl ::core::default::Default for P6PfsBy {
    #[inline(always)]
    fn default() -> P6PfsBy {
        <crate::RegValueT<P6PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P70Pfs_SPEC;
impl crate::sealed::RegSpec for P70Pfs_SPEC {
    type DataType = u32;
}
#[doc = "P70%s Pin Function Control Register"]
pub type P70Pfs = crate::RegValueT<P70Pfs_SPEC>;

impl NoBitfieldReg<P70Pfs_SPEC> for P70Pfs {}
impl ::core::default::Default for P70Pfs {
    #[inline(always)]
    fn default() -> P70Pfs {
        <crate::RegValueT<P70Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P70PfsHa_SPEC;
impl crate::sealed::RegSpec for P70PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "P70%s Pin Function Control Register"]
pub type P70PfsHa = crate::RegValueT<P70PfsHa_SPEC>;

impl NoBitfieldReg<P70PfsHa_SPEC> for P70PfsHa {}
impl ::core::default::Default for P70PfsHa {
    #[inline(always)]
    fn default() -> P70PfsHa {
        <crate::RegValueT<P70PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P70PfsBy_SPEC;
impl crate::sealed::RegSpec for P70PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "P70%s Pin Function Control Register"]
pub type P70PfsBy = crate::RegValueT<P70PfsBy_SPEC>;

impl NoBitfieldReg<P70PfsBy_SPEC> for P70PfsBy {}
impl ::core::default::Default for P70PfsBy {
    #[inline(always)]
    fn default() -> P70PfsBy {
        <crate::RegValueT<P70PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P80Pfs_SPEC;
impl crate::sealed::RegSpec for P80Pfs_SPEC {
    type DataType = u32;
}
#[doc = "P80%s Pin Function Control Register"]
pub type P80Pfs = crate::RegValueT<P80Pfs_SPEC>;

impl NoBitfieldReg<P80Pfs_SPEC> for P80Pfs {}
impl ::core::default::Default for P80Pfs {
    #[inline(always)]
    fn default() -> P80Pfs {
        <crate::RegValueT<P80Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P80PfsHa_SPEC;
impl crate::sealed::RegSpec for P80PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "P80%s Pin Function Control Register"]
pub type P80PfsHa = crate::RegValueT<P80PfsHa_SPEC>;

impl NoBitfieldReg<P80PfsHa_SPEC> for P80PfsHa {}
impl ::core::default::Default for P80PfsHa {
    #[inline(always)]
    fn default() -> P80PfsHa {
        <crate::RegValueT<P80PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P80PfsBy_SPEC;
impl crate::sealed::RegSpec for P80PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "P80%s Pin Function Control Register"]
pub type P80PfsBy = crate::RegValueT<P80PfsBy_SPEC>;

impl NoBitfieldReg<P80PfsBy_SPEC> for P80PfsBy {}
impl ::core::default::Default for P80PfsBy {
    #[inline(always)]
    fn default() -> P80PfsBy {
        <crate::RegValueT<P80PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P90Pfs_SPEC;
impl crate::sealed::RegSpec for P90Pfs_SPEC {
    type DataType = u32;
}
#[doc = "P90%s Pin Function Control Register"]
pub type P90Pfs = crate::RegValueT<P90Pfs_SPEC>;

impl NoBitfieldReg<P90Pfs_SPEC> for P90Pfs {}
impl ::core::default::Default for P90Pfs {
    #[inline(always)]
    fn default() -> P90Pfs {
        <crate::RegValueT<P90Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P90PfsHa_SPEC;
impl crate::sealed::RegSpec for P90PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "P90%s Pin Function Control Register"]
pub type P90PfsHa = crate::RegValueT<P90PfsHa_SPEC>;

impl NoBitfieldReg<P90PfsHa_SPEC> for P90PfsHa {}
impl ::core::default::Default for P90PfsHa {
    #[inline(always)]
    fn default() -> P90PfsHa {
        <crate::RegValueT<P90PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P90PfsBy_SPEC;
impl crate::sealed::RegSpec for P90PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "P90%s Pin Function Control Register"]
pub type P90PfsBy = crate::RegValueT<P90PfsBy_SPEC>;

impl NoBitfieldReg<P90PfsBy_SPEC> for P90PfsBy {}
impl ::core::default::Default for P90PfsBy {
    #[inline(always)]
    fn default() -> P90PfsBy {
        <crate::RegValueT<P90PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pa0Pfs_SPEC;
impl crate::sealed::RegSpec for Pa0Pfs_SPEC {
    type DataType = u32;
}
#[doc = "PA0%s Pin Function Control Register"]
pub type Pa0Pfs = crate::RegValueT<Pa0Pfs_SPEC>;

impl NoBitfieldReg<Pa0Pfs_SPEC> for Pa0Pfs {}
impl ::core::default::Default for Pa0Pfs {
    #[inline(always)]
    fn default() -> Pa0Pfs {
        <crate::RegValueT<Pa0Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pa0PfsHa_SPEC;
impl crate::sealed::RegSpec for Pa0PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "PA0%s Pin Function Control Register"]
pub type Pa0PfsHa = crate::RegValueT<Pa0PfsHa_SPEC>;

impl NoBitfieldReg<Pa0PfsHa_SPEC> for Pa0PfsHa {}
impl ::core::default::Default for Pa0PfsHa {
    #[inline(always)]
    fn default() -> Pa0PfsHa {
        <crate::RegValueT<Pa0PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pa0PfsBy_SPEC;
impl crate::sealed::RegSpec for Pa0PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "PA0%s Pin Function Control Register"]
pub type Pa0PfsBy = crate::RegValueT<Pa0PfsBy_SPEC>;

impl NoBitfieldReg<Pa0PfsBy_SPEC> for Pa0PfsBy {}
impl ::core::default::Default for Pa0PfsBy {
    #[inline(always)]
    fn default() -> Pa0PfsBy {
        <crate::RegValueT<Pa0PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pa10Pfs_SPEC;
impl crate::sealed::RegSpec for Pa10Pfs_SPEC {
    type DataType = u32;
}
#[doc = "PA10 Pin Function Control Register"]
pub type Pa10Pfs = crate::RegValueT<Pa10Pfs_SPEC>;

impl NoBitfieldReg<Pa10Pfs_SPEC> for Pa10Pfs {}
impl ::core::default::Default for Pa10Pfs {
    #[inline(always)]
    fn default() -> Pa10Pfs {
        <crate::RegValueT<Pa10Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pa10PfsHa_SPEC;
impl crate::sealed::RegSpec for Pa10PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "PA10 Pin Function Control Register"]
pub type Pa10PfsHa = crate::RegValueT<Pa10PfsHa_SPEC>;

impl NoBitfieldReg<Pa10PfsHa_SPEC> for Pa10PfsHa {}
impl ::core::default::Default for Pa10PfsHa {
    #[inline(always)]
    fn default() -> Pa10PfsHa {
        <crate::RegValueT<Pa10PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pa10PfsBy_SPEC;
impl crate::sealed::RegSpec for Pa10PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "PA10 Pin Function Control Register"]
pub type Pa10PfsBy = crate::RegValueT<Pa10PfsBy_SPEC>;

impl NoBitfieldReg<Pa10PfsBy_SPEC> for Pa10PfsBy {}
impl ::core::default::Default for Pa10PfsBy {
    #[inline(always)]
    fn default() -> Pa10PfsBy {
        <crate::RegValueT<Pa10PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pb0Pfs_SPEC;
impl crate::sealed::RegSpec for Pb0Pfs_SPEC {
    type DataType = u32;
}
#[doc = "PB0%s Pin Function Control Register"]
pub type Pb0Pfs = crate::RegValueT<Pb0Pfs_SPEC>;

impl NoBitfieldReg<Pb0Pfs_SPEC> for Pb0Pfs {}
impl ::core::default::Default for Pb0Pfs {
    #[inline(always)]
    fn default() -> Pb0Pfs {
        <crate::RegValueT<Pb0Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pb0PfsHa_SPEC;
impl crate::sealed::RegSpec for Pb0PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "PB0%s Pin Function Control Register"]
pub type Pb0PfsHa = crate::RegValueT<Pb0PfsHa_SPEC>;

impl NoBitfieldReg<Pb0PfsHa_SPEC> for Pb0PfsHa {}
impl ::core::default::Default for Pb0PfsHa {
    #[inline(always)]
    fn default() -> Pb0PfsHa {
        <crate::RegValueT<Pb0PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pb0PfsBy_SPEC;
impl crate::sealed::RegSpec for Pb0PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "PB0%s Pin Function Control Register"]
pub type Pb0PfsBy = crate::RegValueT<Pb0PfsBy_SPEC>;

impl NoBitfieldReg<Pb0PfsBy_SPEC> for Pb0PfsBy {}
impl ::core::default::Default for Pb0PfsBy {
    #[inline(always)]
    fn default() -> Pb0PfsBy {
        <crate::RegValueT<Pb0PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}
