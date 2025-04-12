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
// Generated from SVD 1.00.01, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:19:56 +0000

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
    #[doc = "Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p00pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P00Pfs_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x0usize))
        }
    }

    #[doc = "Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p00pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P00PfsHa_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x0usize))
        }
    }

    #[doc = "Port 00%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p00pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P00PfsBy_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x0usize))
        }
    }

    #[doc = "Port 0%s Pin Function Select Register"]
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

    #[doc = "Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p0pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P0PfsHa_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x38usize))
        }
    }

    #[doc = "Port 0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p0pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P0PfsBy_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x38usize))
        }
    }

    #[doc = "Port 10%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p10pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P10Pfs_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x40usize))
        }
    }

    #[doc = "Port 10%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p10pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P10PfsHa_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x40usize))
        }
    }

    #[doc = "Port 10%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p10pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P10PfsBy_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x40usize))
        }
    }

    #[doc = "Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p1pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P1Pfs_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x70usize))
        }
    }

    #[doc = "Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p1pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P1PfsHa_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x70usize))
        }
    }

    #[doc = "Port 1%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p1pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P1PfsBy_SPEC, crate::common::RW>,
        4,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x70usize))
        }
    }

    #[doc = "Port 200 Pin Function Select Register"]
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

    #[doc = "Port 200 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p200pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P200PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P200PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[doc = "Port 200 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p200pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P200PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P200PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[doc = "Port 201 Pin Function Select Register"]
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

    #[doc = "Port 201 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p201pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P201PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P201PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[doc = "Port 201 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p201pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P201PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P201PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[doc = "Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p20pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P20Pfs_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x88usize))
        }
    }

    #[doc = "Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p20pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P20PfsHa_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x88usize))
        }
    }

    #[doc = "Port 20%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p20pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P20PfsBy_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x88usize))
        }
    }

    #[doc = "Port 208 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p208pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P208Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P208Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(160usize),
            )
        }
    }

    #[doc = "Port 208 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p208pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P208PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P208PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(160usize),
            )
        }
    }

    #[doc = "Port 208 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p208pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P208PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P208PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(160usize),
            )
        }
    }

    #[doc = "Port 209 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p209pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P209Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P209Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(164usize),
            )
        }
    }

    #[doc = "Port 209 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p209pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P209PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P209PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(164usize),
            )
        }
    }

    #[doc = "Port 209 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p209pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P209PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P209PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(164usize),
            )
        }
    }

    #[doc = "Port 210 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p210pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P210Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P210Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(168usize),
            )
        }
    }

    #[doc = "Port 210 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p210pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P210PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P210PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(168usize),
            )
        }
    }

    #[doc = "Port 210 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p210pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P210PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P210PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(168usize),
            )
        }
    }

    #[doc = "Port 211 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p211pfs(
        &self,
    ) -> &'static crate::common::Reg<self::P211Pfs_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P211Pfs_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(172usize),
            )
        }
    }

    #[doc = "Port 211 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p211pfs_ha(
        &self,
    ) -> &'static crate::common::Reg<self::P211PfsHa_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P211PfsHa_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(172usize),
            )
        }
    }

    #[doc = "Port 211 Pin Function Select Register"]
    #[inline(always)]
    pub const fn p211pfs_by(
        &self,
    ) -> &'static crate::common::Reg<self::P211PfsBy_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P211PfsBy_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(172usize),
            )
        }
    }

    #[doc = "Port 2%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p2pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P2Pfs_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xb0usize))
        }
    }

    #[doc = "Port 2%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p2pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P2PfsHa_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xb0usize))
        }
    }

    #[doc = "Port 2%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p2pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P2PfsBy_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xb0usize))
        }
    }

    #[doc = "Port 30%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p30pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P30Pfs_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xc0usize))
        }
    }

    #[doc = "Port 30%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p30pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P30PfsHa_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xc0usize))
        }
    }

    #[doc = "Port 30%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p30pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P30PfsBy_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xc0usize))
        }
    }

    #[doc = "Port 3%s Pin Function Select Register"]
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

    #[doc = "Port 3%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p3pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P3PfsHa_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xe8usize))
        }
    }

    #[doc = "Port 3%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p3pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P3PfsBy_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xe8usize))
        }
    }

    #[doc = "Port 40%s Pin Function Select Register"]
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

    #[doc = "Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p40pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P40PfsHa_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x100usize))
        }
    }

    #[doc = "Port 40%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p40pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P40PfsBy_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x100usize))
        }
    }

    #[doc = "Port 4%s Pin Function Select Register"]
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

    #[doc = "Port 4%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p4pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P4PfsHa_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x128usize))
        }
    }

    #[doc = "Port 4%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p4pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P4PfsBy_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x128usize))
        }
    }

    #[doc = "Port 50%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p50pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P50Pfs_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x140usize))
        }
    }

    #[doc = "Port 50%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p50pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P50PfsHa_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x140usize))
        }
    }

    #[doc = "Port 50%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p50pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P50PfsBy_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x140usize))
        }
    }

    #[doc = "Port 5%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p5pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P5Pfs_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x168usize))
        }
    }

    #[doc = "Port 5%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p5pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P5PfsHa_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x168usize))
        }
    }

    #[doc = "Port 5%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p5pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P5PfsBy_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x168usize))
        }
    }

    #[doc = "Port 60%s Pin Function Select Register"]
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

    #[doc = "Port 60%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p60pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P60PfsHa_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x180usize))
        }
    }

    #[doc = "Port 60%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p60pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P60PfsBy_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x180usize))
        }
    }

    #[doc = "Port 6%s Pin Function Select Register"]
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

    #[doc = "Port 6%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p6pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P6PfsHa_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1a8usize))
        }
    }

    #[doc = "Port 6%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p6pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P6PfsBy_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1a8usize))
        }
    }

    #[doc = "Port 70%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p70pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P70Pfs_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1c0usize))
        }
    }

    #[doc = "Port 70%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p70pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P70PfsHa_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1c0usize))
        }
    }

    #[doc = "Port 70%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p70pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P70PfsBy_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1c0usize))
        }
    }

    #[doc = "Port 7%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p7pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P7Pfs_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1e8usize))
        }
    }

    #[doc = "Port 7%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p7pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P7PfsHa_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1e8usize))
        }
    }

    #[doc = "Port 7%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p7pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P7PfsBy_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1e8usize))
        }
    }

    #[doc = "Port 80%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p80pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P80Pfs_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x200usize))
        }
    }

    #[doc = "Port 80%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p80pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P80PfsHa_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x200usize))
        }
    }

    #[doc = "Port 80%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p80pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P80PfsBy_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x200usize))
        }
    }

    #[doc = "Port 8%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p8pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P8Pfs_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x238usize))
        }
    }

    #[doc = "Port 8%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p8pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P8PfsHa_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x238usize))
        }
    }

    #[doc = "Port 8%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p8pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P8PfsBy_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x238usize))
        }
    }

    #[doc = "Port 90%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p90pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P90Pfs_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x240usize))
        }
    }

    #[doc = "Port 90%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p90pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P90PfsHa_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x240usize))
        }
    }

    #[doc = "Port 90%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p90pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P90PfsBy_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x240usize))
        }
    }

    #[doc = "Port 9%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p9pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P9Pfs_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x268usize))
        }
    }

    #[doc = "Port 9%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p9pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P9PfsHa_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x268usize))
        }
    }

    #[doc = "Port 9%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn p9pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::P9PfsBy_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x268usize))
        }
    }

    #[doc = "Port A0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa0pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pa0Pfs_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x280usize))
        }
    }

    #[doc = "Port A0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa0pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pa0PfsHa_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x280usize))
        }
    }

    #[doc = "Port A0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pa0pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pa0PfsBy_SPEC, crate::common::RW>,
        10,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x280usize))
        }
    }

    #[doc = "Port An Pin Function Select Register"]
    #[inline(always)]
    pub const fn papfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Papfs_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2a8usize))
        }
    }

    #[doc = "Port An Pin Function Select Register"]
    #[inline(always)]
    pub const fn papfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::PapfsHa_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2a8usize))
        }
    }

    #[doc = "Port An Pin Function Select Register"]
    #[inline(always)]
    pub const fn papfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::PapfsBy_SPEC, crate::common::RW>,
        6,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2a8usize))
        }
    }

    #[doc = "Port B0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb0pfs(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pb0Pfs_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2c0usize))
        }
    }

    #[doc = "Port B0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb0pfs_ha(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pb0PfsHa_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2c0usize))
        }
    }

    #[doc = "Port B0%s Pin Function Select Register"]
    #[inline(always)]
    pub const fn pb0pfs_by(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Pb0PfsBy_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x2c0usize))
        }
    }

    #[doc = "Write-Protect Register for Secure"]
    #[inline(always)]
    pub const fn pwpr_s(&self) -> &'static crate::common::Reg<self::PwprS_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PwprS_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1300usize),
            )
        }
    }

    #[doc = "Port Security Attribution register"]
    #[inline(always)]
    pub const fn psar(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Psar_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x558usize))
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P00Pfs_SPEC;
impl crate::sealed::RegSpec for P00Pfs_SPEC {
    type DataType = u32;
}
#[doc = "Port 00%s Pin Function Select Register"]
pub type P00Pfs = crate::RegValueT<P00Pfs_SPEC>;

impl P00Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p00pfs::Podr, P00Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p00pfs::Podr, P00Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, p00pfs::Pidr, P00Pfs_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,p00pfs::Pidr, P00Pfs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, p00pfs::Pdr, P00Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,p00pfs::Pdr, P00Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, p00pfs::Pcr, P00Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,p00pfs::Pcr, P00Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, p00pfs::Ncodr, P00Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,p00pfs::Ncodr, P00Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, p00pfs::Dscr, P00Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,p00pfs::Dscr, P00Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, p00pfs::Eofr, P00Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,p00pfs::Eofr, P00Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, p00pfs::Isel, P00Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,p00pfs::Isel, P00Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, p00pfs::Asel, P00Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,p00pfs::Asel, P00Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, p00pfs::Pmr, P00Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,p00pfs::Pmr, P00Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, P00Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, P00Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P00Pfs {
    #[inline(always)]
    fn default() -> P00Pfs {
        <crate::RegValueT<P00Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p00pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);
        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);
        #[doc = "High-speed high-drive"]
        pub const _10: Self = Self::new(2);
        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
        pub const _00: Self = Self::new(0);
        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);
        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);
        #[doc = "Detect both edges"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P00PfsHa_SPEC;
impl crate::sealed::RegSpec for P00PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "Port 00%s Pin Function Select Register"]
pub type P00PfsHa = crate::RegValueT<P00PfsHa_SPEC>;

impl P00PfsHa {
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p00pfs_ha::Pmr, P00PfsHa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p00pfs_ha::Pmr, P00PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, P00PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, P00PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P00PfsHa {
    #[inline(always)]
    fn default() -> P00PfsHa {
        <crate::RegValueT<P00PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p00pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P00PfsBy_SPEC;
impl crate::sealed::RegSpec for P00PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "Port 00%s Pin Function Select Register"]
pub type P00PfsBy = crate::RegValueT<P00PfsBy_SPEC>;

impl P00PfsBy {
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, P00PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, P00PfsBy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P00PfsBy {
    #[inline(always)]
    fn default() -> P00PfsBy {
        <crate::RegValueT<P00PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0Pfs_SPEC;
impl crate::sealed::RegSpec for P0Pfs_SPEC {
    type DataType = u32;
}
#[doc = "Port 0%s Pin Function Select Register"]
pub type P0Pfs = crate::RegValueT<P0Pfs_SPEC>;

impl P0Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p0pfs::Podr, P0Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p0pfs::Podr, P0Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, p0pfs::Pidr, P0Pfs_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,p0pfs::Pidr, P0Pfs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, p0pfs::Pdr, P0Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,p0pfs::Pdr, P0Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, p0pfs::Pcr, P0Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,p0pfs::Pcr, P0Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, p0pfs::Ncodr, P0Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,p0pfs::Ncodr, P0Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, p0pfs::Dscr, P0Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,p0pfs::Dscr, P0Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, p0pfs::Eofr, P0Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,p0pfs::Eofr, P0Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, p0pfs::Isel, P0Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,p0pfs::Isel, P0Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, p0pfs::Asel, P0Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,p0pfs::Asel, P0Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, p0pfs::Pmr, P0Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,p0pfs::Pmr, P0Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, P0Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, P0Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P0Pfs {
    #[inline(always)]
    fn default() -> P0Pfs {
        <crate::RegValueT<P0Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p0pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);
        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);
        #[doc = "High-speed high-drive"]
        pub const _10: Self = Self::new(2);
        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
        pub const _00: Self = Self::new(0);
        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);
        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);
        #[doc = "Detect both edges"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0PfsHa_SPEC;
impl crate::sealed::RegSpec for P0PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "Port 0%s Pin Function Select Register"]
pub type P0PfsHa = crate::RegValueT<P0PfsHa_SPEC>;

impl P0PfsHa {
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p0pfs_ha::Pmr, P0PfsHa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p0pfs_ha::Pmr, P0PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, P0PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, P0PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P0PfsHa {
    #[inline(always)]
    fn default() -> P0PfsHa {
        <crate::RegValueT<P0PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p0pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0PfsBy_SPEC;
impl crate::sealed::RegSpec for P0PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "Port 0%s Pin Function Select Register"]
pub type P0PfsBy = crate::RegValueT<P0PfsBy_SPEC>;

impl P0PfsBy {
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, P0PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, P0PfsBy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P0PfsBy {
    #[inline(always)]
    fn default() -> P0PfsBy {
        <crate::RegValueT<P0PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P10Pfs_SPEC;
impl crate::sealed::RegSpec for P10Pfs_SPEC {
    type DataType = u32;
}
#[doc = "Port 10%s Pin Function Select Register"]
pub type P10Pfs = crate::RegValueT<P10Pfs_SPEC>;

impl P10Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p10pfs::Podr, P10Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p10pfs::Podr, P10Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, p10pfs::Pidr, P10Pfs_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,p10pfs::Pidr, P10Pfs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, p10pfs::Pdr, P10Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,p10pfs::Pdr, P10Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, p10pfs::Pcr, P10Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,p10pfs::Pcr, P10Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, p10pfs::Ncodr, P10Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,p10pfs::Ncodr, P10Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, p10pfs::Dscr, P10Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,p10pfs::Dscr, P10Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, p10pfs::Eofr, P10Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,p10pfs::Eofr, P10Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, p10pfs::Isel, P10Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,p10pfs::Isel, P10Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, p10pfs::Asel, P10Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,p10pfs::Asel, P10Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, p10pfs::Pmr, P10Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,p10pfs::Pmr, P10Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, P10Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, P10Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P10Pfs {
    #[inline(always)]
    fn default() -> P10Pfs {
        <crate::RegValueT<P10Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p10pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);
        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);
        #[doc = "High-speed high-drive"]
        pub const _10: Self = Self::new(2);
        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
        pub const _00: Self = Self::new(0);
        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);
        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);
        #[doc = "Detect both edges"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P10PfsHa_SPEC;
impl crate::sealed::RegSpec for P10PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "Port 10%s Pin Function Select Register"]
pub type P10PfsHa = crate::RegValueT<P10PfsHa_SPEC>;

impl P10PfsHa {
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p10pfs_ha::Pmr, P10PfsHa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p10pfs_ha::Pmr, P10PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, P10PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, P10PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P10PfsHa {
    #[inline(always)]
    fn default() -> P10PfsHa {
        <crate::RegValueT<P10PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p10pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P10PfsBy_SPEC;
impl crate::sealed::RegSpec for P10PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "Port 10%s Pin Function Select Register"]
pub type P10PfsBy = crate::RegValueT<P10PfsBy_SPEC>;

impl P10PfsBy {
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, P10PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, P10PfsBy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P10PfsBy {
    #[inline(always)]
    fn default() -> P10PfsBy {
        <crate::RegValueT<P10PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1Pfs_SPEC;
impl crate::sealed::RegSpec for P1Pfs_SPEC {
    type DataType = u32;
}
#[doc = "Port 1%s Pin Function Select Register"]
pub type P1Pfs = crate::RegValueT<P1Pfs_SPEC>;

impl P1Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p1pfs::Podr, P1Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p1pfs::Podr, P1Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, p1pfs::Pidr, P1Pfs_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,p1pfs::Pidr, P1Pfs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, p1pfs::Pdr, P1Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,p1pfs::Pdr, P1Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, p1pfs::Pcr, P1Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,p1pfs::Pcr, P1Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, p1pfs::Ncodr, P1Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,p1pfs::Ncodr, P1Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, p1pfs::Dscr, P1Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,p1pfs::Dscr, P1Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, p1pfs::Eofr, P1Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,p1pfs::Eofr, P1Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, p1pfs::Isel, P1Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,p1pfs::Isel, P1Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, p1pfs::Asel, P1Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,p1pfs::Asel, P1Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, p1pfs::Pmr, P1Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,p1pfs::Pmr, P1Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, P1Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, P1Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P1Pfs {
    #[inline(always)]
    fn default() -> P1Pfs {
        <crate::RegValueT<P1Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p1pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);
        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);
        #[doc = "High-speed high-drive"]
        pub const _10: Self = Self::new(2);
        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
        pub const _00: Self = Self::new(0);
        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);
        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);
        #[doc = "Detect both edges"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1PfsHa_SPEC;
impl crate::sealed::RegSpec for P1PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "Port 1%s Pin Function Select Register"]
pub type P1PfsHa = crate::RegValueT<P1PfsHa_SPEC>;

impl P1PfsHa {
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p1pfs_ha::Pmr, P1PfsHa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p1pfs_ha::Pmr, P1PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, P1PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, P1PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P1PfsHa {
    #[inline(always)]
    fn default() -> P1PfsHa {
        <crate::RegValueT<P1PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p1pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1PfsBy_SPEC;
impl crate::sealed::RegSpec for P1PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "Port 1%s Pin Function Select Register"]
pub type P1PfsBy = crate::RegValueT<P1PfsBy_SPEC>;

impl P1PfsBy {
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, P1PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, P1PfsBy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
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
#[doc = "Port 200 Pin Function Select Register"]
pub type P200Pfs = crate::RegValueT<P200Pfs_SPEC>;

impl P200Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p200pfs::Podr, P200Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p200pfs::Podr, P200Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, p200pfs::Pidr, P200Pfs_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,p200pfs::Pidr, P200Pfs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, p200pfs::Pdr, P200Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,p200pfs::Pdr, P200Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, p200pfs::Pcr, P200Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,p200pfs::Pcr, P200Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, p200pfs::Ncodr, P200Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,p200pfs::Ncodr, P200Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, p200pfs::Dscr, P200Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,p200pfs::Dscr, P200Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, p200pfs::Eofr, P200Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,p200pfs::Eofr, P200Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, p200pfs::Isel, P200Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,p200pfs::Isel, P200Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, p200pfs::Asel, P200Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,p200pfs::Asel, P200Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, p200pfs::Pmr, P200Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,p200pfs::Pmr, P200Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, P200Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, P200Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P200Pfs {
    #[inline(always)]
    fn default() -> P200Pfs {
        <crate::RegValueT<P200Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p200pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);
        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);
        #[doc = "High-speed high-drive"]
        pub const _10: Self = Self::new(2);
        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
        pub const _00: Self = Self::new(0);
        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);
        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);
        #[doc = "Detect both edges"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P200PfsHa_SPEC;
impl crate::sealed::RegSpec for P200PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "Port 200 Pin Function Select Register"]
pub type P200PfsHa = crate::RegValueT<P200PfsHa_SPEC>;

impl P200PfsHa {
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p200pfs_ha::Pmr,
        P200PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p200pfs_ha::Pmr,
            P200PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, P200PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, P200PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P200PfsHa {
    #[inline(always)]
    fn default() -> P200PfsHa {
        <crate::RegValueT<P200PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p200pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P200PfsBy_SPEC;
impl crate::sealed::RegSpec for P200PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "Port 200 Pin Function Select Register"]
pub type P200PfsBy = crate::RegValueT<P200PfsBy_SPEC>;

impl P200PfsBy {
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, P200PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, P200PfsBy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
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
#[doc = "Port 201 Pin Function Select Register"]
pub type P201Pfs = crate::RegValueT<P201Pfs_SPEC>;

impl P201Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p201pfs::Podr, P201Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p201pfs::Podr, P201Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, p201pfs::Pidr, P201Pfs_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,p201pfs::Pidr, P201Pfs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, p201pfs::Pdr, P201Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,p201pfs::Pdr, P201Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, p201pfs::Pcr, P201Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,p201pfs::Pcr, P201Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, p201pfs::Ncodr, P201Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,p201pfs::Ncodr, P201Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, p201pfs::Dscr, P201Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,p201pfs::Dscr, P201Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, p201pfs::Eofr, P201Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,p201pfs::Eofr, P201Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, p201pfs::Isel, P201Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,p201pfs::Isel, P201Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, p201pfs::Asel, P201Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,p201pfs::Asel, P201Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, p201pfs::Pmr, P201Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,p201pfs::Pmr, P201Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, P201Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, P201Pfs_SPEC,crate::common::RW>::from_register(self,0)
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
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);
        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);
        #[doc = "High-speed high-drive"]
        pub const _10: Self = Self::new(2);
        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
        pub const _00: Self = Self::new(0);
        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);
        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);
        #[doc = "Detect both edges"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P201PfsHa_SPEC;
impl crate::sealed::RegSpec for P201PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "Port 201 Pin Function Select Register"]
pub type P201PfsHa = crate::RegValueT<P201PfsHa_SPEC>;

impl P201PfsHa {
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p201pfs_ha::Pmr,
        P201PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p201pfs_ha::Pmr,
            P201PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, P201PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, P201PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P201PfsHa {
    #[inline(always)]
    fn default() -> P201PfsHa {
        <crate::RegValueT<P201PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p201pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P201PfsBy_SPEC;
impl crate::sealed::RegSpec for P201PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "Port 201 Pin Function Select Register"]
pub type P201PfsBy = crate::RegValueT<P201PfsBy_SPEC>;

impl P201PfsBy {
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, P201PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, P201PfsBy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P201PfsBy {
    #[inline(always)]
    fn default() -> P201PfsBy {
        <crate::RegValueT<P201PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P20Pfs_SPEC;
impl crate::sealed::RegSpec for P20Pfs_SPEC {
    type DataType = u32;
}
#[doc = "Port 20%s Pin Function Select Register"]
pub type P20Pfs = crate::RegValueT<P20Pfs_SPEC>;

impl P20Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p20pfs::Podr, P20Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p20pfs::Podr, P20Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, p20pfs::Pidr, P20Pfs_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,p20pfs::Pidr, P20Pfs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, p20pfs::Pdr, P20Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,p20pfs::Pdr, P20Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, p20pfs::Pcr, P20Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,p20pfs::Pcr, P20Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, p20pfs::Ncodr, P20Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,p20pfs::Ncodr, P20Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, p20pfs::Dscr, P20Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,p20pfs::Dscr, P20Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, p20pfs::Eofr, P20Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,p20pfs::Eofr, P20Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, p20pfs::Isel, P20Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,p20pfs::Isel, P20Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, p20pfs::Asel, P20Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,p20pfs::Asel, P20Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, p20pfs::Pmr, P20Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,p20pfs::Pmr, P20Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, P20Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, P20Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P20Pfs {
    #[inline(always)]
    fn default() -> P20Pfs {
        <crate::RegValueT<P20Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p20pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);
        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);
        #[doc = "High-speed high-drive"]
        pub const _10: Self = Self::new(2);
        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
        pub const _00: Self = Self::new(0);
        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);
        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);
        #[doc = "Detect both edges"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P20PfsHa_SPEC;
impl crate::sealed::RegSpec for P20PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "Port 20%s Pin Function Select Register"]
pub type P20PfsHa = crate::RegValueT<P20PfsHa_SPEC>;

impl P20PfsHa {
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p20pfs_ha::Pmr, P20PfsHa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p20pfs_ha::Pmr, P20PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, P20PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, P20PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P20PfsHa {
    #[inline(always)]
    fn default() -> P20PfsHa {
        <crate::RegValueT<P20PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p20pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P20PfsBy_SPEC;
impl crate::sealed::RegSpec for P20PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "Port 20%s Pin Function Select Register"]
pub type P20PfsBy = crate::RegValueT<P20PfsBy_SPEC>;

impl P20PfsBy {
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, P20PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, P20PfsBy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P20PfsBy {
    #[inline(always)]
    fn default() -> P20PfsBy {
        <crate::RegValueT<P20PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P208Pfs_SPEC;
impl crate::sealed::RegSpec for P208Pfs_SPEC {
    type DataType = u32;
}
#[doc = "Port 208 Pin Function Select Register"]
pub type P208Pfs = crate::RegValueT<P208Pfs_SPEC>;

impl P208Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p208pfs::Podr, P208Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p208pfs::Podr, P208Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, p208pfs::Pidr, P208Pfs_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,p208pfs::Pidr, P208Pfs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, p208pfs::Pdr, P208Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,p208pfs::Pdr, P208Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, p208pfs::Pcr, P208Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,p208pfs::Pcr, P208Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, p208pfs::Ncodr, P208Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,p208pfs::Ncodr, P208Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, p208pfs::Dscr, P208Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,p208pfs::Dscr, P208Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, p208pfs::Eofr, P208Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,p208pfs::Eofr, P208Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, p208pfs::Isel, P208Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,p208pfs::Isel, P208Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, p208pfs::Asel, P208Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,p208pfs::Asel, P208Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, p208pfs::Pmr, P208Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,p208pfs::Pmr, P208Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, P208Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, P208Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P208Pfs {
    #[inline(always)]
    fn default() -> P208Pfs {
        <crate::RegValueT<P208Pfs_SPEC> as RegisterValue<_>>::new(65552)
    }
}
pub mod p208pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);
        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);
        #[doc = "High-speed high-drive"]
        pub const _10: Self = Self::new(2);
        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
        pub const _00: Self = Self::new(0);
        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);
        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);
        #[doc = "Detect both edges"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P208PfsHa_SPEC;
impl crate::sealed::RegSpec for P208PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "Port 208 Pin Function Select Register"]
pub type P208PfsHa = crate::RegValueT<P208PfsHa_SPEC>;

impl P208PfsHa {
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p208pfs_ha::Pmr,
        P208PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p208pfs_ha::Pmr,
            P208PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, P208PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, P208PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P208PfsHa {
    #[inline(always)]
    fn default() -> P208PfsHa {
        <crate::RegValueT<P208PfsHa_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod p208pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P208PfsBy_SPEC;
impl crate::sealed::RegSpec for P208PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "Port 208 Pin Function Select Register"]
pub type P208PfsBy = crate::RegValueT<P208PfsBy_SPEC>;

impl P208PfsBy {
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, P208PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, P208PfsBy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P208PfsBy {
    #[inline(always)]
    fn default() -> P208PfsBy {
        <crate::RegValueT<P208PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P209Pfs_SPEC;
impl crate::sealed::RegSpec for P209Pfs_SPEC {
    type DataType = u32;
}
#[doc = "Port 209 Pin Function Select Register"]
pub type P209Pfs = crate::RegValueT<P209Pfs_SPEC>;

impl P209Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p209pfs::Podr, P209Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p209pfs::Podr, P209Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, p209pfs::Pidr, P209Pfs_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,p209pfs::Pidr, P209Pfs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, p209pfs::Pdr, P209Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,p209pfs::Pdr, P209Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, p209pfs::Pcr, P209Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,p209pfs::Pcr, P209Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, p209pfs::Ncodr, P209Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,p209pfs::Ncodr, P209Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, p209pfs::Dscr, P209Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,p209pfs::Dscr, P209Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, p209pfs::Eofr, P209Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,p209pfs::Eofr, P209Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, p209pfs::Isel, P209Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,p209pfs::Isel, P209Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, p209pfs::Asel, P209Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,p209pfs::Asel, P209Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, p209pfs::Pmr, P209Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,p209pfs::Pmr, P209Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, P209Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, P209Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P209Pfs {
    #[inline(always)]
    fn default() -> P209Pfs {
        <crate::RegValueT<P209Pfs_SPEC> as RegisterValue<_>>::new(66560)
    }
}
pub mod p209pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);
        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);
        #[doc = "High-speed high-drive"]
        pub const _10: Self = Self::new(2);
        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
        pub const _00: Self = Self::new(0);
        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);
        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);
        #[doc = "Detect both edges"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P209PfsHa_SPEC;
impl crate::sealed::RegSpec for P209PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "Port 209 Pin Function Select Register"]
pub type P209PfsHa = crate::RegValueT<P209PfsHa_SPEC>;

impl P209PfsHa {
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p209pfs_ha::Pmr,
        P209PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p209pfs_ha::Pmr,
            P209PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, P209PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, P209PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P209PfsHa {
    #[inline(always)]
    fn default() -> P209PfsHa {
        <crate::RegValueT<P209PfsHa_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod p209pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P209PfsBy_SPEC;
impl crate::sealed::RegSpec for P209PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "Port 209 Pin Function Select Register"]
pub type P209PfsBy = crate::RegValueT<P209PfsBy_SPEC>;

impl P209PfsBy {
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, P209PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, P209PfsBy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P209PfsBy {
    #[inline(always)]
    fn default() -> P209PfsBy {
        <crate::RegValueT<P209PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P210Pfs_SPEC;
impl crate::sealed::RegSpec for P210Pfs_SPEC {
    type DataType = u32;
}
#[doc = "Port 210 Pin Function Select Register"]
pub type P210Pfs = crate::RegValueT<P210Pfs_SPEC>;

impl P210Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p210pfs::Podr, P210Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p210pfs::Podr, P210Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, p210pfs::Pidr, P210Pfs_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,p210pfs::Pidr, P210Pfs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, p210pfs::Pdr, P210Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,p210pfs::Pdr, P210Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, p210pfs::Pcr, P210Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,p210pfs::Pcr, P210Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, p210pfs::Ncodr, P210Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,p210pfs::Ncodr, P210Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, p210pfs::Dscr, P210Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,p210pfs::Dscr, P210Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, p210pfs::Eofr, P210Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,p210pfs::Eofr, P210Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, p210pfs::Isel, P210Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,p210pfs::Isel, P210Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, p210pfs::Asel, P210Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,p210pfs::Asel, P210Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, p210pfs::Pmr, P210Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,p210pfs::Pmr, P210Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, P210Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, P210Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P210Pfs {
    #[inline(always)]
    fn default() -> P210Pfs {
        <crate::RegValueT<P210Pfs_SPEC> as RegisterValue<_>>::new(66576)
    }
}
pub mod p210pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);
        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);
        #[doc = "High-speed high-drive"]
        pub const _10: Self = Self::new(2);
        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
        pub const _00: Self = Self::new(0);
        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);
        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);
        #[doc = "Detect both edges"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P210PfsHa_SPEC;
impl crate::sealed::RegSpec for P210PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "Port 210 Pin Function Select Register"]
pub type P210PfsHa = crate::RegValueT<P210PfsHa_SPEC>;

impl P210PfsHa {
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p210pfs_ha::Pmr,
        P210PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p210pfs_ha::Pmr,
            P210PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, P210PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, P210PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P210PfsHa {
    #[inline(always)]
    fn default() -> P210PfsHa {
        <crate::RegValueT<P210PfsHa_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod p210pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P210PfsBy_SPEC;
impl crate::sealed::RegSpec for P210PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "Port 210 Pin Function Select Register"]
pub type P210PfsBy = crate::RegValueT<P210PfsBy_SPEC>;

impl P210PfsBy {
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, P210PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, P210PfsBy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P210PfsBy {
    #[inline(always)]
    fn default() -> P210PfsBy {
        <crate::RegValueT<P210PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P211Pfs_SPEC;
impl crate::sealed::RegSpec for P211Pfs_SPEC {
    type DataType = u32;
}
#[doc = "Port 211 Pin Function Select Register"]
pub type P211Pfs = crate::RegValueT<P211Pfs_SPEC>;

impl P211Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p211pfs::Podr, P211Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p211pfs::Podr, P211Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, p211pfs::Pidr, P211Pfs_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,p211pfs::Pidr, P211Pfs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, p211pfs::Pdr, P211Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,p211pfs::Pdr, P211Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, p211pfs::Pcr, P211Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,p211pfs::Pcr, P211Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, p211pfs::Ncodr, P211Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,p211pfs::Ncodr, P211Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, p211pfs::Dscr, P211Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,p211pfs::Dscr, P211Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, p211pfs::Eofr, P211Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,p211pfs::Eofr, P211Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, p211pfs::Isel, P211Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,p211pfs::Isel, P211Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, p211pfs::Asel, P211Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,p211pfs::Asel, P211Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, p211pfs::Pmr, P211Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,p211pfs::Pmr, P211Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, P211Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, P211Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P211Pfs {
    #[inline(always)]
    fn default() -> P211Pfs {
        <crate::RegValueT<P211Pfs_SPEC> as RegisterValue<_>>::new(65552)
    }
}
pub mod p211pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);
        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);
        #[doc = "High-speed high-drive"]
        pub const _10: Self = Self::new(2);
        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
        pub const _00: Self = Self::new(0);
        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);
        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);
        #[doc = "Detect both edges"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P211PfsHa_SPEC;
impl crate::sealed::RegSpec for P211PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "Port 211 Pin Function Select Register"]
pub type P211PfsHa = crate::RegValueT<P211PfsHa_SPEC>;

impl P211PfsHa {
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        p211pfs_ha::Pmr,
        P211PfsHa_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            p211pfs_ha::Pmr,
            P211PfsHa_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, P211PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, P211PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P211PfsHa {
    #[inline(always)]
    fn default() -> P211PfsHa {
        <crate::RegValueT<P211PfsHa_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod p211pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P211PfsBy_SPEC;
impl crate::sealed::RegSpec for P211PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "Port 211 Pin Function Select Register"]
pub type P211PfsBy = crate::RegValueT<P211PfsBy_SPEC>;

impl P211PfsBy {
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, P211PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, P211PfsBy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P211PfsBy {
    #[inline(always)]
    fn default() -> P211PfsBy {
        <crate::RegValueT<P211PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2Pfs_SPEC;
impl crate::sealed::RegSpec for P2Pfs_SPEC {
    type DataType = u32;
}
#[doc = "Port 2%s Pin Function Select Register"]
pub type P2Pfs = crate::RegValueT<P2Pfs_SPEC>;

impl P2Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p2pfs::Podr, P2Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p2pfs::Podr, P2Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, p2pfs::Pidr, P2Pfs_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,p2pfs::Pidr, P2Pfs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, p2pfs::Pdr, P2Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,p2pfs::Pdr, P2Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, p2pfs::Pcr, P2Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,p2pfs::Pcr, P2Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, p2pfs::Ncodr, P2Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,p2pfs::Ncodr, P2Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, p2pfs::Dscr, P2Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,p2pfs::Dscr, P2Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, p2pfs::Eofr, P2Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,p2pfs::Eofr, P2Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, p2pfs::Isel, P2Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,p2pfs::Isel, P2Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, p2pfs::Asel, P2Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,p2pfs::Asel, P2Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, p2pfs::Pmr, P2Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,p2pfs::Pmr, P2Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, P2Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, P2Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P2Pfs {
    #[inline(always)]
    fn default() -> P2Pfs {
        <crate::RegValueT<P2Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p2pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);
        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);
        #[doc = "High-speed high-drive"]
        pub const _10: Self = Self::new(2);
        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
        pub const _00: Self = Self::new(0);
        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);
        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);
        #[doc = "Detect both edges"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2PfsHa_SPEC;
impl crate::sealed::RegSpec for P2PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "Port 2%s Pin Function Select Register"]
pub type P2PfsHa = crate::RegValueT<P2PfsHa_SPEC>;

impl P2PfsHa {
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p2pfs_ha::Pmr, P2PfsHa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p2pfs_ha::Pmr, P2PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, P2PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, P2PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P2PfsHa {
    #[inline(always)]
    fn default() -> P2PfsHa {
        <crate::RegValueT<P2PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p2pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2PfsBy_SPEC;
impl crate::sealed::RegSpec for P2PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "Port 2%s Pin Function Select Register"]
pub type P2PfsBy = crate::RegValueT<P2PfsBy_SPEC>;

impl P2PfsBy {
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, P2PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, P2PfsBy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P2PfsBy {
    #[inline(always)]
    fn default() -> P2PfsBy {
        <crate::RegValueT<P2PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P30Pfs_SPEC;
impl crate::sealed::RegSpec for P30Pfs_SPEC {
    type DataType = u32;
}
#[doc = "Port 30%s Pin Function Select Register"]
pub type P30Pfs = crate::RegValueT<P30Pfs_SPEC>;

impl P30Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p30pfs::Podr, P30Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p30pfs::Podr, P30Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, p30pfs::Pidr, P30Pfs_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,p30pfs::Pidr, P30Pfs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, p30pfs::Pdr, P30Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,p30pfs::Pdr, P30Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, p30pfs::Pcr, P30Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,p30pfs::Pcr, P30Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, p30pfs::Ncodr, P30Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,p30pfs::Ncodr, P30Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, p30pfs::Dscr, P30Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,p30pfs::Dscr, P30Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, p30pfs::Eofr, P30Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,p30pfs::Eofr, P30Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, p30pfs::Isel, P30Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,p30pfs::Isel, P30Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, p30pfs::Asel, P30Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,p30pfs::Asel, P30Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, p30pfs::Pmr, P30Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,p30pfs::Pmr, P30Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, P30Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, P30Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P30Pfs {
    #[inline(always)]
    fn default() -> P30Pfs {
        <crate::RegValueT<P30Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p30pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);
        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);
        #[doc = "High-speed high-drive"]
        pub const _10: Self = Self::new(2);
        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
        pub const _00: Self = Self::new(0);
        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);
        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);
        #[doc = "Detect both edges"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P30PfsHa_SPEC;
impl crate::sealed::RegSpec for P30PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "Port 30%s Pin Function Select Register"]
pub type P30PfsHa = crate::RegValueT<P30PfsHa_SPEC>;

impl P30PfsHa {
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p30pfs_ha::Pmr, P30PfsHa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p30pfs_ha::Pmr, P30PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, P30PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, P30PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P30PfsHa {
    #[inline(always)]
    fn default() -> P30PfsHa {
        <crate::RegValueT<P30PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p30pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P30PfsBy_SPEC;
impl crate::sealed::RegSpec for P30PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "Port 30%s Pin Function Select Register"]
pub type P30PfsBy = crate::RegValueT<P30PfsBy_SPEC>;

impl P30PfsBy {
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, P30PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, P30PfsBy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
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
#[doc = "Port 3%s Pin Function Select Register"]
pub type P3Pfs = crate::RegValueT<P3Pfs_SPEC>;

impl P3Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p3pfs::Podr, P3Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p3pfs::Podr, P3Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, p3pfs::Pidr, P3Pfs_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,p3pfs::Pidr, P3Pfs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, p3pfs::Pdr, P3Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,p3pfs::Pdr, P3Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, p3pfs::Pcr, P3Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,p3pfs::Pcr, P3Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, p3pfs::Ncodr, P3Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,p3pfs::Ncodr, P3Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, p3pfs::Dscr, P3Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,p3pfs::Dscr, P3Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, p3pfs::Eofr, P3Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,p3pfs::Eofr, P3Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, p3pfs::Isel, P3Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,p3pfs::Isel, P3Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, p3pfs::Asel, P3Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,p3pfs::Asel, P3Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, p3pfs::Pmr, P3Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,p3pfs::Pmr, P3Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, P3Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, P3Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P3Pfs {
    #[inline(always)]
    fn default() -> P3Pfs {
        <crate::RegValueT<P3Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p3pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);
        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);
        #[doc = "High-speed high-drive"]
        pub const _10: Self = Self::new(2);
        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
        pub const _00: Self = Self::new(0);
        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);
        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);
        #[doc = "Detect both edges"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P3PfsHa_SPEC;
impl crate::sealed::RegSpec for P3PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "Port 3%s Pin Function Select Register"]
pub type P3PfsHa = crate::RegValueT<P3PfsHa_SPEC>;

impl P3PfsHa {
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p3pfs_ha::Pmr, P3PfsHa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p3pfs_ha::Pmr, P3PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, P3PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, P3PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P3PfsHa {
    #[inline(always)]
    fn default() -> P3PfsHa {
        <crate::RegValueT<P3PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p3pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P3PfsBy_SPEC;
impl crate::sealed::RegSpec for P3PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "Port 3%s Pin Function Select Register"]
pub type P3PfsBy = crate::RegValueT<P3PfsBy_SPEC>;

impl P3PfsBy {
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, P3PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, P3PfsBy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
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
#[doc = "Port 40%s Pin Function Select Register"]
pub type P40Pfs = crate::RegValueT<P40Pfs_SPEC>;

impl P40Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p40pfs::Podr, P40Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p40pfs::Podr, P40Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, p40pfs::Pidr, P40Pfs_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,p40pfs::Pidr, P40Pfs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, p40pfs::Pdr, P40Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,p40pfs::Pdr, P40Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, p40pfs::Pcr, P40Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,p40pfs::Pcr, P40Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, p40pfs::Ncodr, P40Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,p40pfs::Ncodr, P40Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, p40pfs::Dscr, P40Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,p40pfs::Dscr, P40Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, p40pfs::Eofr, P40Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,p40pfs::Eofr, P40Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, p40pfs::Isel, P40Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,p40pfs::Isel, P40Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, p40pfs::Asel, P40Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,p40pfs::Asel, P40Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, p40pfs::Pmr, P40Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,p40pfs::Pmr, P40Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, P40Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, P40Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P40Pfs {
    #[inline(always)]
    fn default() -> P40Pfs {
        <crate::RegValueT<P40Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p40pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);
        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);
        #[doc = "High-speed high-drive"]
        pub const _10: Self = Self::new(2);
        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
        pub const _00: Self = Self::new(0);
        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);
        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);
        #[doc = "Detect both edges"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P40PfsHa_SPEC;
impl crate::sealed::RegSpec for P40PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "Port 40%s Pin Function Select Register"]
pub type P40PfsHa = crate::RegValueT<P40PfsHa_SPEC>;

impl P40PfsHa {
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p40pfs_ha::Pmr, P40PfsHa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p40pfs_ha::Pmr, P40PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, P40PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, P40PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P40PfsHa {
    #[inline(always)]
    fn default() -> P40PfsHa {
        <crate::RegValueT<P40PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p40pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P40PfsBy_SPEC;
impl crate::sealed::RegSpec for P40PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "Port 40%s Pin Function Select Register"]
pub type P40PfsBy = crate::RegValueT<P40PfsBy_SPEC>;

impl P40PfsBy {
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, P40PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, P40PfsBy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
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
#[doc = "Port 4%s Pin Function Select Register"]
pub type P4Pfs = crate::RegValueT<P4Pfs_SPEC>;

impl P4Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p4pfs::Podr, P4Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p4pfs::Podr, P4Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, p4pfs::Pidr, P4Pfs_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,p4pfs::Pidr, P4Pfs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, p4pfs::Pdr, P4Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,p4pfs::Pdr, P4Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, p4pfs::Pcr, P4Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,p4pfs::Pcr, P4Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, p4pfs::Ncodr, P4Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,p4pfs::Ncodr, P4Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, p4pfs::Dscr, P4Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,p4pfs::Dscr, P4Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, p4pfs::Eofr, P4Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,p4pfs::Eofr, P4Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, p4pfs::Isel, P4Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,p4pfs::Isel, P4Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, p4pfs::Asel, P4Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,p4pfs::Asel, P4Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, p4pfs::Pmr, P4Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,p4pfs::Pmr, P4Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, P4Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, P4Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P4Pfs {
    #[inline(always)]
    fn default() -> P4Pfs {
        <crate::RegValueT<P4Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p4pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);
        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);
        #[doc = "High-speed high-drive"]
        pub const _10: Self = Self::new(2);
        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
        pub const _00: Self = Self::new(0);
        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);
        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);
        #[doc = "Detect both edges"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P4PfsHa_SPEC;
impl crate::sealed::RegSpec for P4PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "Port 4%s Pin Function Select Register"]
pub type P4PfsHa = crate::RegValueT<P4PfsHa_SPEC>;

impl P4PfsHa {
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p4pfs_ha::Pmr, P4PfsHa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p4pfs_ha::Pmr, P4PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, P4PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, P4PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P4PfsHa {
    #[inline(always)]
    fn default() -> P4PfsHa {
        <crate::RegValueT<P4PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p4pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P4PfsBy_SPEC;
impl crate::sealed::RegSpec for P4PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "Port 4%s Pin Function Select Register"]
pub type P4PfsBy = crate::RegValueT<P4PfsBy_SPEC>;

impl P4PfsBy {
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, P4PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, P4PfsBy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
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
#[doc = "Port 50%s Pin Function Select Register"]
pub type P50Pfs = crate::RegValueT<P50Pfs_SPEC>;

impl P50Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p50pfs::Podr, P50Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p50pfs::Podr, P50Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, p50pfs::Pidr, P50Pfs_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,p50pfs::Pidr, P50Pfs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, p50pfs::Pdr, P50Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,p50pfs::Pdr, P50Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, p50pfs::Pcr, P50Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,p50pfs::Pcr, P50Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, p50pfs::Ncodr, P50Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,p50pfs::Ncodr, P50Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, p50pfs::Dscr, P50Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,p50pfs::Dscr, P50Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, p50pfs::Eofr, P50Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,p50pfs::Eofr, P50Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, p50pfs::Isel, P50Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,p50pfs::Isel, P50Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, p50pfs::Asel, P50Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,p50pfs::Asel, P50Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, p50pfs::Pmr, P50Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,p50pfs::Pmr, P50Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, P50Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, P50Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P50Pfs {
    #[inline(always)]
    fn default() -> P50Pfs {
        <crate::RegValueT<P50Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p50pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);
        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);
        #[doc = "High-speed high-drive"]
        pub const _10: Self = Self::new(2);
        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
        pub const _00: Self = Self::new(0);
        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);
        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);
        #[doc = "Detect both edges"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P50PfsHa_SPEC;
impl crate::sealed::RegSpec for P50PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "Port 50%s Pin Function Select Register"]
pub type P50PfsHa = crate::RegValueT<P50PfsHa_SPEC>;

impl P50PfsHa {
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p50pfs_ha::Pmr, P50PfsHa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p50pfs_ha::Pmr, P50PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, P50PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, P50PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P50PfsHa {
    #[inline(always)]
    fn default() -> P50PfsHa {
        <crate::RegValueT<P50PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p50pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P50PfsBy_SPEC;
impl crate::sealed::RegSpec for P50PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "Port 50%s Pin Function Select Register"]
pub type P50PfsBy = crate::RegValueT<P50PfsBy_SPEC>;

impl P50PfsBy {
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, P50PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, P50PfsBy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
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
#[doc = "Port 5%s Pin Function Select Register"]
pub type P5Pfs = crate::RegValueT<P5Pfs_SPEC>;

impl P5Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p5pfs::Podr, P5Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p5pfs::Podr, P5Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, p5pfs::Pidr, P5Pfs_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,p5pfs::Pidr, P5Pfs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, p5pfs::Pdr, P5Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,p5pfs::Pdr, P5Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, p5pfs::Pcr, P5Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,p5pfs::Pcr, P5Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, p5pfs::Ncodr, P5Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,p5pfs::Ncodr, P5Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, p5pfs::Dscr, P5Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,p5pfs::Dscr, P5Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, p5pfs::Eofr, P5Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,p5pfs::Eofr, P5Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, p5pfs::Isel, P5Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,p5pfs::Isel, P5Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, p5pfs::Asel, P5Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,p5pfs::Asel, P5Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, p5pfs::Pmr, P5Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,p5pfs::Pmr, P5Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, P5Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, P5Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P5Pfs {
    #[inline(always)]
    fn default() -> P5Pfs {
        <crate::RegValueT<P5Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p5pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);
        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);
        #[doc = "High-speed high-drive"]
        pub const _10: Self = Self::new(2);
        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
        pub const _00: Self = Self::new(0);
        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);
        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);
        #[doc = "Detect both edges"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P5PfsHa_SPEC;
impl crate::sealed::RegSpec for P5PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "Port 5%s Pin Function Select Register"]
pub type P5PfsHa = crate::RegValueT<P5PfsHa_SPEC>;

impl P5PfsHa {
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p5pfs_ha::Pmr, P5PfsHa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p5pfs_ha::Pmr, P5PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, P5PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, P5PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P5PfsHa {
    #[inline(always)]
    fn default() -> P5PfsHa {
        <crate::RegValueT<P5PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p5pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P5PfsBy_SPEC;
impl crate::sealed::RegSpec for P5PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "Port 5%s Pin Function Select Register"]
pub type P5PfsBy = crate::RegValueT<P5PfsBy_SPEC>;

impl P5PfsBy {
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, P5PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, P5PfsBy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
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
#[doc = "Port 60%s Pin Function Select Register"]
pub type P60Pfs = crate::RegValueT<P60Pfs_SPEC>;

impl P60Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p60pfs::Podr, P60Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p60pfs::Podr, P60Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, p60pfs::Pidr, P60Pfs_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,p60pfs::Pidr, P60Pfs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, p60pfs::Pdr, P60Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,p60pfs::Pdr, P60Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, p60pfs::Pcr, P60Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,p60pfs::Pcr, P60Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, p60pfs::Ncodr, P60Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,p60pfs::Ncodr, P60Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, p60pfs::Dscr, P60Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,p60pfs::Dscr, P60Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, p60pfs::Eofr, P60Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,p60pfs::Eofr, P60Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, p60pfs::Isel, P60Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,p60pfs::Isel, P60Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, p60pfs::Asel, P60Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,p60pfs::Asel, P60Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, p60pfs::Pmr, P60Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,p60pfs::Pmr, P60Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, P60Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, P60Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P60Pfs {
    #[inline(always)]
    fn default() -> P60Pfs {
        <crate::RegValueT<P60Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p60pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);
        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);
        #[doc = "High-speed high-drive"]
        pub const _10: Self = Self::new(2);
        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
        pub const _00: Self = Self::new(0);
        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);
        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);
        #[doc = "Detect both edges"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P60PfsHa_SPEC;
impl crate::sealed::RegSpec for P60PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "Port 60%s Pin Function Select Register"]
pub type P60PfsHa = crate::RegValueT<P60PfsHa_SPEC>;

impl P60PfsHa {
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p60pfs_ha::Pmr, P60PfsHa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p60pfs_ha::Pmr, P60PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, P60PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, P60PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P60PfsHa {
    #[inline(always)]
    fn default() -> P60PfsHa {
        <crate::RegValueT<P60PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p60pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P60PfsBy_SPEC;
impl crate::sealed::RegSpec for P60PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "Port 60%s Pin Function Select Register"]
pub type P60PfsBy = crate::RegValueT<P60PfsBy_SPEC>;

impl P60PfsBy {
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, P60PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, P60PfsBy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
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
#[doc = "Port 6%s Pin Function Select Register"]
pub type P6Pfs = crate::RegValueT<P6Pfs_SPEC>;

impl P6Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p6pfs::Podr, P6Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p6pfs::Podr, P6Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, p6pfs::Pidr, P6Pfs_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,p6pfs::Pidr, P6Pfs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, p6pfs::Pdr, P6Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,p6pfs::Pdr, P6Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, p6pfs::Pcr, P6Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,p6pfs::Pcr, P6Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, p6pfs::Ncodr, P6Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,p6pfs::Ncodr, P6Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, p6pfs::Dscr, P6Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,p6pfs::Dscr, P6Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, p6pfs::Eofr, P6Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,p6pfs::Eofr, P6Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, p6pfs::Isel, P6Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,p6pfs::Isel, P6Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, p6pfs::Asel, P6Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,p6pfs::Asel, P6Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, p6pfs::Pmr, P6Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,p6pfs::Pmr, P6Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, P6Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, P6Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P6Pfs {
    #[inline(always)]
    fn default() -> P6Pfs {
        <crate::RegValueT<P6Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p6pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);
        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);
        #[doc = "High-speed high-drive"]
        pub const _10: Self = Self::new(2);
        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
        pub const _00: Self = Self::new(0);
        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);
        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);
        #[doc = "Detect both edges"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P6PfsHa_SPEC;
impl crate::sealed::RegSpec for P6PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "Port 6%s Pin Function Select Register"]
pub type P6PfsHa = crate::RegValueT<P6PfsHa_SPEC>;

impl P6PfsHa {
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p6pfs_ha::Pmr, P6PfsHa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p6pfs_ha::Pmr, P6PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, P6PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, P6PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P6PfsHa {
    #[inline(always)]
    fn default() -> P6PfsHa {
        <crate::RegValueT<P6PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p6pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P6PfsBy_SPEC;
impl crate::sealed::RegSpec for P6PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "Port 6%s Pin Function Select Register"]
pub type P6PfsBy = crate::RegValueT<P6PfsBy_SPEC>;

impl P6PfsBy {
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, P6PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, P6PfsBy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
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
#[doc = "Port 70%s Pin Function Select Register"]
pub type P70Pfs = crate::RegValueT<P70Pfs_SPEC>;

impl P70Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p70pfs::Podr, P70Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p70pfs::Podr, P70Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, p70pfs::Pidr, P70Pfs_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,p70pfs::Pidr, P70Pfs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, p70pfs::Pdr, P70Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,p70pfs::Pdr, P70Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, p70pfs::Pcr, P70Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,p70pfs::Pcr, P70Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, p70pfs::Ncodr, P70Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,p70pfs::Ncodr, P70Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, p70pfs::Dscr, P70Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,p70pfs::Dscr, P70Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, p70pfs::Eofr, P70Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,p70pfs::Eofr, P70Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, p70pfs::Isel, P70Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,p70pfs::Isel, P70Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, p70pfs::Asel, P70Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,p70pfs::Asel, P70Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, p70pfs::Pmr, P70Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,p70pfs::Pmr, P70Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, P70Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, P70Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P70Pfs {
    #[inline(always)]
    fn default() -> P70Pfs {
        <crate::RegValueT<P70Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p70pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);
        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);
        #[doc = "High-speed high-drive"]
        pub const _10: Self = Self::new(2);
        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
        pub const _00: Self = Self::new(0);
        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);
        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);
        #[doc = "Detect both edges"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P70PfsHa_SPEC;
impl crate::sealed::RegSpec for P70PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "Port 70%s Pin Function Select Register"]
pub type P70PfsHa = crate::RegValueT<P70PfsHa_SPEC>;

impl P70PfsHa {
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p70pfs_ha::Pmr, P70PfsHa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p70pfs_ha::Pmr, P70PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, P70PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, P70PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P70PfsHa {
    #[inline(always)]
    fn default() -> P70PfsHa {
        <crate::RegValueT<P70PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p70pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P70PfsBy_SPEC;
impl crate::sealed::RegSpec for P70PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "Port 70%s Pin Function Select Register"]
pub type P70PfsBy = crate::RegValueT<P70PfsBy_SPEC>;

impl P70PfsBy {
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, P70PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, P70PfsBy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P70PfsBy {
    #[inline(always)]
    fn default() -> P70PfsBy {
        <crate::RegValueT<P70PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P7Pfs_SPEC;
impl crate::sealed::RegSpec for P7Pfs_SPEC {
    type DataType = u32;
}
#[doc = "Port 7%s Pin Function Select Register"]
pub type P7Pfs = crate::RegValueT<P7Pfs_SPEC>;

impl P7Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p7pfs::Podr, P7Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p7pfs::Podr, P7Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, p7pfs::Pidr, P7Pfs_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,p7pfs::Pidr, P7Pfs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, p7pfs::Pdr, P7Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,p7pfs::Pdr, P7Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, p7pfs::Pcr, P7Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,p7pfs::Pcr, P7Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, p7pfs::Ncodr, P7Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,p7pfs::Ncodr, P7Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, p7pfs::Dscr, P7Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,p7pfs::Dscr, P7Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, p7pfs::Eofr, P7Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,p7pfs::Eofr, P7Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, p7pfs::Isel, P7Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,p7pfs::Isel, P7Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, p7pfs::Asel, P7Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,p7pfs::Asel, P7Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, p7pfs::Pmr, P7Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,p7pfs::Pmr, P7Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, P7Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, P7Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P7Pfs {
    #[inline(always)]
    fn default() -> P7Pfs {
        <crate::RegValueT<P7Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p7pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);
        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);
        #[doc = "High-speed high-drive"]
        pub const _10: Self = Self::new(2);
        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
        pub const _00: Self = Self::new(0);
        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);
        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);
        #[doc = "Detect both edges"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P7PfsHa_SPEC;
impl crate::sealed::RegSpec for P7PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "Port 7%s Pin Function Select Register"]
pub type P7PfsHa = crate::RegValueT<P7PfsHa_SPEC>;

impl P7PfsHa {
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p7pfs_ha::Pmr, P7PfsHa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p7pfs_ha::Pmr, P7PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, P7PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, P7PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P7PfsHa {
    #[inline(always)]
    fn default() -> P7PfsHa {
        <crate::RegValueT<P7PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p7pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P7PfsBy_SPEC;
impl crate::sealed::RegSpec for P7PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "Port 7%s Pin Function Select Register"]
pub type P7PfsBy = crate::RegValueT<P7PfsBy_SPEC>;

impl P7PfsBy {
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, P7PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, P7PfsBy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P7PfsBy {
    #[inline(always)]
    fn default() -> P7PfsBy {
        <crate::RegValueT<P7PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P80Pfs_SPEC;
impl crate::sealed::RegSpec for P80Pfs_SPEC {
    type DataType = u32;
}
#[doc = "Port 80%s Pin Function Select Register"]
pub type P80Pfs = crate::RegValueT<P80Pfs_SPEC>;

impl P80Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p80pfs::Podr, P80Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p80pfs::Podr, P80Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, p80pfs::Pidr, P80Pfs_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,p80pfs::Pidr, P80Pfs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, p80pfs::Pdr, P80Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,p80pfs::Pdr, P80Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, p80pfs::Pcr, P80Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,p80pfs::Pcr, P80Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, p80pfs::Ncodr, P80Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,p80pfs::Ncodr, P80Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, p80pfs::Dscr, P80Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,p80pfs::Dscr, P80Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, p80pfs::Eofr, P80Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,p80pfs::Eofr, P80Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, p80pfs::Isel, P80Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,p80pfs::Isel, P80Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, p80pfs::Asel, P80Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,p80pfs::Asel, P80Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, p80pfs::Pmr, P80Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,p80pfs::Pmr, P80Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, P80Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, P80Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P80Pfs {
    #[inline(always)]
    fn default() -> P80Pfs {
        <crate::RegValueT<P80Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p80pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);
        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);
        #[doc = "High-speed high-drive"]
        pub const _10: Self = Self::new(2);
        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
        pub const _00: Self = Self::new(0);
        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);
        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);
        #[doc = "Detect both edges"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P80PfsHa_SPEC;
impl crate::sealed::RegSpec for P80PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "Port 80%s Pin Function Select Register"]
pub type P80PfsHa = crate::RegValueT<P80PfsHa_SPEC>;

impl P80PfsHa {
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p80pfs_ha::Pmr, P80PfsHa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p80pfs_ha::Pmr, P80PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, P80PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, P80PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P80PfsHa {
    #[inline(always)]
    fn default() -> P80PfsHa {
        <crate::RegValueT<P80PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p80pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P80PfsBy_SPEC;
impl crate::sealed::RegSpec for P80PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "Port 80%s Pin Function Select Register"]
pub type P80PfsBy = crate::RegValueT<P80PfsBy_SPEC>;

impl P80PfsBy {
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, P80PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, P80PfsBy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P80PfsBy {
    #[inline(always)]
    fn default() -> P80PfsBy {
        <crate::RegValueT<P80PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P8Pfs_SPEC;
impl crate::sealed::RegSpec for P8Pfs_SPEC {
    type DataType = u32;
}
#[doc = "Port 8%s Pin Function Select Register"]
pub type P8Pfs = crate::RegValueT<P8Pfs_SPEC>;

impl P8Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p8pfs::Podr, P8Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p8pfs::Podr, P8Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, p8pfs::Pidr, P8Pfs_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,p8pfs::Pidr, P8Pfs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, p8pfs::Pdr, P8Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,p8pfs::Pdr, P8Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, p8pfs::Pcr, P8Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,p8pfs::Pcr, P8Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, p8pfs::Ncodr, P8Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,p8pfs::Ncodr, P8Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, p8pfs::Dscr, P8Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,p8pfs::Dscr, P8Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, p8pfs::Eofr, P8Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,p8pfs::Eofr, P8Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, p8pfs::Isel, P8Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,p8pfs::Isel, P8Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, p8pfs::Asel, P8Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,p8pfs::Asel, P8Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, p8pfs::Pmr, P8Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,p8pfs::Pmr, P8Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, P8Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, P8Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P8Pfs {
    #[inline(always)]
    fn default() -> P8Pfs {
        <crate::RegValueT<P8Pfs_SPEC> as RegisterValue<_>>::new(318832640)
    }
}
pub mod p8pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);
        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);
        #[doc = "High-speed high-drive"]
        pub const _10: Self = Self::new(2);
        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
        pub const _00: Self = Self::new(0);
        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);
        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);
        #[doc = "Detect both edges"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P8PfsHa_SPEC;
impl crate::sealed::RegSpec for P8PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "Port 8%s Pin Function Select Register"]
pub type P8PfsHa = crate::RegValueT<P8PfsHa_SPEC>;

impl P8PfsHa {
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p8pfs_ha::Pmr, P8PfsHa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p8pfs_ha::Pmr, P8PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, P8PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, P8PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P8PfsHa {
    #[inline(always)]
    fn default() -> P8PfsHa {
        <crate::RegValueT<P8PfsHa_SPEC> as RegisterValue<_>>::new(4865)
    }
}
pub mod p8pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P8PfsBy_SPEC;
impl crate::sealed::RegSpec for P8PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "Port 8%s Pin Function Select Register"]
pub type P8PfsBy = crate::RegValueT<P8PfsBy_SPEC>;

impl P8PfsBy {
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, P8PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, P8PfsBy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P8PfsBy {
    #[inline(always)]
    fn default() -> P8PfsBy {
        <crate::RegValueT<P8PfsBy_SPEC> as RegisterValue<_>>::new(19)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P90Pfs_SPEC;
impl crate::sealed::RegSpec for P90Pfs_SPEC {
    type DataType = u32;
}
#[doc = "Port 90%s Pin Function Select Register"]
pub type P90Pfs = crate::RegValueT<P90Pfs_SPEC>;

impl P90Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p90pfs::Podr, P90Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p90pfs::Podr, P90Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, p90pfs::Pidr, P90Pfs_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,p90pfs::Pidr, P90Pfs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, p90pfs::Pdr, P90Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,p90pfs::Pdr, P90Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, p90pfs::Pcr, P90Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,p90pfs::Pcr, P90Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, p90pfs::Ncodr, P90Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,p90pfs::Ncodr, P90Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, p90pfs::Dscr, P90Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,p90pfs::Dscr, P90Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, p90pfs::Eofr, P90Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,p90pfs::Eofr, P90Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, p90pfs::Isel, P90Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,p90pfs::Isel, P90Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, p90pfs::Asel, P90Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,p90pfs::Asel, P90Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, p90pfs::Pmr, P90Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,p90pfs::Pmr, P90Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, P90Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, P90Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P90Pfs {
    #[inline(always)]
    fn default() -> P90Pfs {
        <crate::RegValueT<P90Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p90pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);
        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);
        #[doc = "High-speed high-drive"]
        pub const _10: Self = Self::new(2);
        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
        pub const _00: Self = Self::new(0);
        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);
        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);
        #[doc = "Detect both edges"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P90PfsHa_SPEC;
impl crate::sealed::RegSpec for P90PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "Port 90%s Pin Function Select Register"]
pub type P90PfsHa = crate::RegValueT<P90PfsHa_SPEC>;

impl P90PfsHa {
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p90pfs_ha::Pmr, P90PfsHa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p90pfs_ha::Pmr, P90PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, P90PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, P90PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P90PfsHa {
    #[inline(always)]
    fn default() -> P90PfsHa {
        <crate::RegValueT<P90PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p90pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P90PfsBy_SPEC;
impl crate::sealed::RegSpec for P90PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "Port 90%s Pin Function Select Register"]
pub type P90PfsBy = crate::RegValueT<P90PfsBy_SPEC>;

impl P90PfsBy {
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, P90PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, P90PfsBy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P90PfsBy {
    #[inline(always)]
    fn default() -> P90PfsBy {
        <crate::RegValueT<P90PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P9Pfs_SPEC;
impl crate::sealed::RegSpec for P9Pfs_SPEC {
    type DataType = u32;
}
#[doc = "Port 9%s Pin Function Select Register"]
pub type P9Pfs = crate::RegValueT<P9Pfs_SPEC>;

impl P9Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p9pfs::Podr, P9Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p9pfs::Podr, P9Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, p9pfs::Pidr, P9Pfs_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,p9pfs::Pidr, P9Pfs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, p9pfs::Pdr, P9Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,p9pfs::Pdr, P9Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, p9pfs::Pcr, P9Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,p9pfs::Pcr, P9Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, p9pfs::Ncodr, P9Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,p9pfs::Ncodr, P9Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, p9pfs::Dscr, P9Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,p9pfs::Dscr, P9Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, p9pfs::Eofr, P9Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,p9pfs::Eofr, P9Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, p9pfs::Isel, P9Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,p9pfs::Isel, P9Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, p9pfs::Asel, P9Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,p9pfs::Asel, P9Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, p9pfs::Pmr, P9Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,p9pfs::Pmr, P9Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, P9Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, P9Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P9Pfs {
    #[inline(always)]
    fn default() -> P9Pfs {
        <crate::RegValueT<P9Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p9pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);
        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);
        #[doc = "High-speed high-drive"]
        pub const _10: Self = Self::new(2);
        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
        pub const _00: Self = Self::new(0);
        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);
        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);
        #[doc = "Detect both edges"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P9PfsHa_SPEC;
impl crate::sealed::RegSpec for P9PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "Port 9%s Pin Function Select Register"]
pub type P9PfsHa = crate::RegValueT<P9PfsHa_SPEC>;

impl P9PfsHa {
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, p9pfs_ha::Pmr, P9PfsHa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,p9pfs_ha::Pmr, P9PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, P9PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, P9PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P9PfsHa {
    #[inline(always)]
    fn default() -> P9PfsHa {
        <crate::RegValueT<P9PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod p9pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P9PfsBy_SPEC;
impl crate::sealed::RegSpec for P9PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "Port 9%s Pin Function Select Register"]
pub type P9PfsBy = crate::RegValueT<P9PfsBy_SPEC>;

impl P9PfsBy {
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, P9PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, P9PfsBy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P9PfsBy {
    #[inline(always)]
    fn default() -> P9PfsBy {
        <crate::RegValueT<P9PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pa0Pfs_SPEC;
impl crate::sealed::RegSpec for Pa0Pfs_SPEC {
    type DataType = u32;
}
#[doc = "Port A0%s Pin Function Select Register"]
pub type Pa0Pfs = crate::RegValueT<Pa0Pfs_SPEC>;

impl Pa0Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, pa0pfs::Podr, Pa0Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,pa0pfs::Podr, Pa0Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, pa0pfs::Pidr, Pa0Pfs_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,pa0pfs::Pidr, Pa0Pfs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, pa0pfs::Pdr, Pa0Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,pa0pfs::Pdr, Pa0Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, pa0pfs::Pcr, Pa0Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,pa0pfs::Pcr, Pa0Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, pa0pfs::Ncodr, Pa0Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,pa0pfs::Ncodr, Pa0Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, pa0pfs::Dscr, Pa0Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,pa0pfs::Dscr, Pa0Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, pa0pfs::Eofr, Pa0Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,pa0pfs::Eofr, Pa0Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, pa0pfs::Isel, Pa0Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,pa0pfs::Isel, Pa0Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, pa0pfs::Asel, Pa0Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,pa0pfs::Asel, Pa0Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, pa0pfs::Pmr, Pa0Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,pa0pfs::Pmr, Pa0Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, Pa0Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, Pa0Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pa0Pfs {
    #[inline(always)]
    fn default() -> Pa0Pfs {
        <crate::RegValueT<Pa0Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pa0pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);
        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);
        #[doc = "High-speed high-drive"]
        pub const _10: Self = Self::new(2);
        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
        pub const _00: Self = Self::new(0);
        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);
        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);
        #[doc = "Detect both edges"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pa0PfsHa_SPEC;
impl crate::sealed::RegSpec for Pa0PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "Port A0%s Pin Function Select Register"]
pub type Pa0PfsHa = crate::RegValueT<Pa0PfsHa_SPEC>;

impl Pa0PfsHa {
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, pa0pfs_ha::Pmr, Pa0PfsHa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,pa0pfs_ha::Pmr, Pa0PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, Pa0PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, Pa0PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pa0PfsHa {
    #[inline(always)]
    fn default() -> Pa0PfsHa {
        <crate::RegValueT<Pa0PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pa0pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pa0PfsBy_SPEC;
impl crate::sealed::RegSpec for Pa0PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "Port A0%s Pin Function Select Register"]
pub type Pa0PfsBy = crate::RegValueT<Pa0PfsBy_SPEC>;

impl Pa0PfsBy {
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, Pa0PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, Pa0PfsBy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pa0PfsBy {
    #[inline(always)]
    fn default() -> Pa0PfsBy {
        <crate::RegValueT<Pa0PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Papfs_SPEC;
impl crate::sealed::RegSpec for Papfs_SPEC {
    type DataType = u32;
}
#[doc = "Port An Pin Function Select Register"]
pub type Papfs = crate::RegValueT<Papfs_SPEC>;

impl Papfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, papfs::Podr, Papfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,papfs::Podr, Papfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, papfs::Pidr, Papfs_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,papfs::Pidr, Papfs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, papfs::Pdr, Papfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,papfs::Pdr, Papfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, papfs::Pcr, Papfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,papfs::Pcr, Papfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, papfs::Ncodr, Papfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,papfs::Ncodr, Papfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, papfs::Dscr, Papfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,papfs::Dscr, Papfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, papfs::Eofr, Papfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,papfs::Eofr, Papfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, papfs::Isel, Papfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,papfs::Isel, Papfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, papfs::Asel, Papfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,papfs::Asel, Papfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, papfs::Pmr, Papfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,papfs::Pmr, Papfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, Papfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, Papfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Papfs {
    #[inline(always)]
    fn default() -> Papfs {
        <crate::RegValueT<Papfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod papfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);
        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);
        #[doc = "High-speed high-drive"]
        pub const _10: Self = Self::new(2);
        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
        pub const _00: Self = Self::new(0);
        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);
        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);
        #[doc = "Detect both edges"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PapfsHa_SPEC;
impl crate::sealed::RegSpec for PapfsHa_SPEC {
    type DataType = u16;
}
#[doc = "Port An Pin Function Select Register"]
pub type PapfsHa = crate::RegValueT<PapfsHa_SPEC>;

impl PapfsHa {
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, papfs_ha::Pmr, PapfsHa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,papfs_ha::Pmr, PapfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, PapfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, PapfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PapfsHa {
    #[inline(always)]
    fn default() -> PapfsHa {
        <crate::RegValueT<PapfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod papfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PapfsBy_SPEC;
impl crate::sealed::RegSpec for PapfsBy_SPEC {
    type DataType = u8;
}
#[doc = "Port An Pin Function Select Register"]
pub type PapfsBy = crate::RegValueT<PapfsBy_SPEC>;

impl PapfsBy {
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, PapfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, PapfsBy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PapfsBy {
    #[inline(always)]
    fn default() -> PapfsBy {
        <crate::RegValueT<PapfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pb0Pfs_SPEC;
impl crate::sealed::RegSpec for Pb0Pfs_SPEC {
    type DataType = u32;
}
#[doc = "Port B0%s Pin Function Select Register"]
pub type Pb0Pfs = crate::RegValueT<Pb0Pfs_SPEC>;

impl Pb0Pfs {
    #[doc = "Port Output Data"]
    #[inline(always)]
    pub fn podr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, pb0pfs::Podr, Pb0Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,pb0pfs::Podr, Pb0Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pmn State"]
    #[inline(always)]
    pub fn pidr(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, pb0pfs::Pidr, Pb0Pfs_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,pb0pfs::Pidr, Pb0Pfs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Port Direction"]
    #[inline(always)]
    pub fn pdr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, pb0pfs::Pdr, Pb0Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,pb0pfs::Pdr, Pb0Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pull-up Control"]
    #[inline(always)]
    pub fn pcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, pb0pfs::Pcr, Pb0Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,pb0pfs::Pcr, Pb0Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, pb0pfs::Ncodr, Pb0Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,pb0pfs::Ncodr, Pb0Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Drive Capability"]
    #[inline(always)]
    pub fn dscr(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, pb0pfs::Dscr, Pb0Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,pb0pfs::Dscr, Pb0Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event on Falling/Event on Rising"]
    #[inline(always)]
    pub fn eofr(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, pb0pfs::Eofr, Pb0Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,pb0pfs::Eofr, Pb0Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IRQ Input Enable"]
    #[inline(always)]
    pub fn isel(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, pb0pfs::Isel, Pb0Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,pb0pfs::Isel, Pb0Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Analog Input Enable"]
    #[inline(always)]
    pub fn asel(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, pb0pfs::Asel, Pb0Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,pb0pfs::Asel, Pb0Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, pb0pfs::Pmr, Pb0Pfs_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,pb0pfs::Pmr, Pb0Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, Pb0Pfs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, Pb0Pfs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pb0Pfs {
    #[inline(always)]
    fn default() -> Pb0Pfs {
        <crate::RegValueT<Pb0Pfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pb0pfs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Podr_SPEC;
    pub type Podr = crate::EnumBitfieldStruct<u8, Podr_SPEC>;
    impl Podr {
        #[doc = "Low output"]
        pub const _0: Self = Self::new(0);
        #[doc = "High output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pidr_SPEC;
    pub type Pidr = crate::EnumBitfieldStruct<u8, Pidr_SPEC>;
    impl Pidr {
        #[doc = "Low level"]
        pub const _0: Self = Self::new(0);
        #[doc = "High level"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdr_SPEC;
    pub type Pdr = crate::EnumBitfieldStruct<u8, Pdr_SPEC>;
    impl Pdr {
        #[doc = "Input (functions as an input pin)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output (functions as an output pin)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcr_SPEC;
    pub type Pcr = crate::EnumBitfieldStruct<u8, Pcr_SPEC>;
    impl Pcr {
        #[doc = "Disable input pull-up"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable input pull-up"]
        pub const _1: Self = Self::new(1);
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
    pub struct Dscr_SPEC;
    pub type Dscr = crate::EnumBitfieldStruct<u8, Dscr_SPEC>;
    impl Dscr {
        #[doc = "Low drive"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle drive"]
        pub const _01: Self = Self::new(1);
        #[doc = "High-speed high-drive"]
        pub const _10: Self = Self::new(2);
        #[doc = "High drive"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eofr_SPEC;
    pub type Eofr = crate::EnumBitfieldStruct<u8, Eofr_SPEC>;
    impl Eofr {
        #[doc = "Don\'t care"]
        pub const _00: Self = Self::new(0);
        #[doc = "Detect rising edge"]
        pub const _01: Self = Self::new(1);
        #[doc = "Detect falling edge"]
        pub const _10: Self = Self::new(2);
        #[doc = "Detect both edges"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Isel_SPEC;
    pub type Isel = crate::EnumBitfieldStruct<u8, Isel_SPEC>;
    impl Isel {
        #[doc = "Not used as an IRQn input pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an IRQn input pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asel_SPEC;
    pub type Asel = crate::EnumBitfieldStruct<u8, Asel_SPEC>;
    impl Asel {
        #[doc = "Not used as an analog pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an analog pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pb0PfsHa_SPEC;
impl crate::sealed::RegSpec for Pb0PfsHa_SPEC {
    type DataType = u16;
}
#[doc = "Port B0%s Pin Function Select Register"]
pub type Pb0PfsHa = crate::RegValueT<Pb0PfsHa_SPEC>;

impl Pb0PfsHa {
    #[doc = "Port Mode Control"]
    #[inline(always)]
    pub fn pmr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, pb0pfs_ha::Pmr, Pb0PfsHa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,pb0pfs_ha::Pmr, Pb0PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, Pb0PfsHa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, Pb0PfsHa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pb0PfsHa {
    #[inline(always)]
    fn default() -> Pb0PfsHa {
        <crate::RegValueT<Pb0PfsHa_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pb0pfs_ha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmr_SPEC;
    pub type Pmr = crate::EnumBitfieldStruct<u8, Pmr_SPEC>;
    impl Pmr {
        #[doc = "Used as a general I/O pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Used as an I/O port for peripheral functions"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pb0PfsBy_SPEC;
impl crate::sealed::RegSpec for Pb0PfsBy_SPEC {
    type DataType = u8;
}
#[doc = "Port B0%s Pin Function Select Register"]
pub type Pb0PfsBy = crate::RegValueT<Pb0PfsBy_SPEC>;

impl Pb0PfsBy {
    #[doc = "Peripheral Select"]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, Pb0PfsBy_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, Pb0PfsBy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pb0PfsBy {
    #[inline(always)]
    fn default() -> Pb0PfsBy {
        <crate::RegValueT<Pb0PfsBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwprS_SPEC;
impl crate::sealed::RegSpec for PwprS_SPEC {
    type DataType = u8;
}
#[doc = "Write-Protect Register for Secure"]
pub type PwprS = crate::RegValueT<PwprS_SPEC>;

impl PwprS {
    #[doc = "PmnPFS Register Write Enable"]
    #[inline(always)]
    pub fn pfswe(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, pwpr_s::Pfswe, PwprS_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,pwpr_s::Pfswe, PwprS_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "PFSWE Bit Write Disable"]
    #[inline(always)]
    pub fn b0wi(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, pwpr_s::B0Wi, PwprS_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,pwpr_s::B0Wi, PwprS_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PwprS {
    #[inline(always)]
    fn default() -> PwprS {
        <crate::RegValueT<PwprS_SPEC> as RegisterValue<_>>::new(128)
    }
}
pub mod pwpr_s {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfswe_SPEC;
    pub type Pfswe = crate::EnumBitfieldStruct<u8, Pfswe_SPEC>;
    impl Pfswe {
        #[doc = "Disable writes to the PmnPFS register"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable writes to the PmnPFS register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct B0Wi_SPEC;
    pub type B0Wi = crate::EnumBitfieldStruct<u8, B0Wi_SPEC>;
    impl B0Wi {
        #[doc = "Enable writes the PFSWE bit"]
        pub const _0: Self = Self::new(0);
        #[doc = "Disable writes to the PFSWE bit"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psar_SPEC;
impl crate::sealed::RegSpec for Psar_SPEC {
    type DataType = u16;
}
#[doc = "Port Security Attribution register"]
pub type Psar = crate::RegValueT<Psar_SPEC>;

impl Psar {
    #[doc = "Pmn Security Attribution"]
    #[inline(always)]
    pub fn pmnsa(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, psar::Pmnsa, Psar_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,psar::Pmnsa, Psar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Psar {
    #[inline(always)]
    fn default() -> Psar {
        <crate::RegValueT<Psar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod psar {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pmnsa_SPEC;
    pub type Pmnsa = crate::EnumBitfieldStruct<u8, Pmnsa_SPEC>;
    impl Pmnsa {
        #[doc = "Secure"]
        pub const _0: Self = Self::new(0);
        #[doc = "Non Secure"]
        pub const _1: Self = Self::new(1);
    }
}
