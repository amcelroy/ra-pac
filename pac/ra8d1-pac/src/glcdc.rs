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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:19:02 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Graphics LCD Controller"]
unsafe impl ::core::marker::Send for super::Glcdc {}
unsafe impl ::core::marker::Sync for super::Glcdc {}
impl super::Glcdc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "Color Palette 0 Plane for Graphics 1 Plane"]
    #[inline(always)]
    pub const fn gr1_clut0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Gr1Clut0_SPEC, crate::common::RW>,
        256,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x0usize))
        }
    }

    #[doc = "Color Palette 1 Plane for Graphics 1 Plane"]
    #[inline(always)]
    pub const fn gr1_clut1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Gr1Clut1_SPEC, crate::common::RW>,
        256,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x400usize))
        }
    }

    #[doc = "Color Palette 0 Plane for Graphics 2 Plane"]
    #[inline(always)]
    pub const fn gr2_clut0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Gr2Clut0_SPEC, crate::common::RW>,
        256,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x800usize))
        }
    }

    #[doc = "Color Palette 1 Plane for Graphics 2 Plane"]
    #[inline(always)]
    pub const fn gr2_clut1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Gr2Clut1_SPEC, crate::common::RW>,
        256,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xc00usize))
        }
    }

    #[doc = "Background Plane Setting Operation Control Register"]
    #[inline(always)]
    pub const fn bg_en(&self) -> &'static crate::common::Reg<self::BgEn_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BgEn_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4096usize),
            )
        }
    }

    #[doc = "Background Plane Setting Free-Running Period Register"]
    #[inline(always)]
    pub const fn bg_peri(
        &self,
    ) -> &'static crate::common::Reg<self::BgPeri_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BgPeri_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4100usize),
            )
        }
    }

    #[doc = "Background Plane Setting Synchronization Position Register"]
    #[inline(always)]
    pub const fn bg_sync(
        &self,
    ) -> &'static crate::common::Reg<self::BgSync_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BgSync_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4104usize),
            )
        }
    }

    #[doc = "Background Plane Setting Full Image Vertical Size Register"]
    #[inline(always)]
    pub const fn bg_vsize(
        &self,
    ) -> &'static crate::common::Reg<self::BgVsize_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BgVsize_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4108usize),
            )
        }
    }

    #[doc = "Background Plane Setting Full Image Horizontal Size Register"]
    #[inline(always)]
    pub const fn bg_hsize(
        &self,
    ) -> &'static crate::common::Reg<self::BgHsize_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BgHsize_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4112usize),
            )
        }
    }

    #[doc = "Background Plane Setting Background Color Register"]
    #[inline(always)]
    pub const fn bg_bgc(&self) -> &'static crate::common::Reg<self::BgBgc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BgBgc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4116usize),
            )
        }
    }

    #[doc = "Background Plane Setting Status Monitor Register"]
    #[inline(always)]
    pub const fn bg_mon(&self) -> &'static crate::common::Reg<self::BgMon_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::BgMon_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(4120usize),
            )
        }
    }

    #[doc = "Graphics %s  Register Update Control Register"]
    #[inline(always)]
    pub const fn gr_ven(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrVen_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1100usize))
        }
    }

    #[doc = "Graphics %s Frame Buffer Read Control Register"]
    #[inline(always)]
    pub const fn gr_flmrd(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrFlmrd_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1104usize))
        }
    }

    #[doc = "Graphics %s Frame Buffer Control Register 1"]
    #[inline(always)]
    pub const fn gr_flm1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrFlm1_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1108usize))
        }
    }

    #[doc = "Graphics %s Frame Buffer Control Register 2"]
    #[inline(always)]
    pub const fn gr_flm2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrFlm2_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x110cusize))
        }
    }

    #[doc = "Graphics %s Frame Buffer Control Register 3"]
    #[inline(always)]
    pub const fn gr_flm3(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrFlm3_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1110usize))
        }
    }

    #[doc = "Graphics %s Frame Buffer Control Register 5"]
    #[inline(always)]
    pub const fn gr_flm5(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrFlm5_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1118usize))
        }
    }

    #[doc = "Graphics %s Frame Buffer Control Register 6"]
    #[inline(always)]
    pub const fn gr_flm6(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrFlm6_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x111cusize))
        }
    }

    #[doc = "Graphics %s Alpha Blending Control Register 1"]
    #[inline(always)]
    pub const fn gr_ab1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrAb1_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1120usize))
        }
    }

    #[doc = "Graphics %s Alpha Blending Control Register 2"]
    #[inline(always)]
    pub const fn gr_ab2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrAb2_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1124usize))
        }
    }

    #[doc = "Graphics %s Alpha Blending Control Register 3"]
    #[inline(always)]
    pub const fn gr_ab3(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrAb3_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1128usize))
        }
    }

    #[doc = "Graphics %s Alpha Blending Control Register 4"]
    #[inline(always)]
    pub const fn gr_ab4(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrAb4_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x112cusize))
        }
    }

    #[doc = "Graphics %s Alpha Blending Control Register 5"]
    #[inline(always)]
    pub const fn gr_ab5(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrAb5_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1130usize))
        }
    }

    #[doc = "Graphics %s Alpha Blending Control Register 6"]
    #[inline(always)]
    pub const fn gr_ab6(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrAb6_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1134usize))
        }
    }

    #[doc = "Graphics %s Alpha Blending Control Register 7"]
    #[inline(always)]
    pub const fn gr_ab7(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrAb7_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1138usize))
        }
    }

    #[doc = "Graphics %s Alpha Blending Control Register 8"]
    #[inline(always)]
    pub const fn gr_ab8(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrAb8_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x113cusize))
        }
    }

    #[doc = "Graphics %s Alpha Blending Control Register 9"]
    #[inline(always)]
    pub const fn gr_ab9(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrAb9_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1140usize))
        }
    }

    #[doc = "Graphics %s Background Color Control Register"]
    #[inline(always)]
    pub const fn gr_base(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrBase_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x114cusize))
        }
    }

    #[doc = "Graphics %s CLUT Table Interrupt Control Register"]
    #[inline(always)]
    pub const fn gr_clutint(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrClutint_SPEC, crate::common::RW>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1150usize))
        }
    }

    #[doc = "Graphics %s Status Monitor Register"]
    #[inline(always)]
    pub const fn gr_mon(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GrMon_SPEC, crate::common::R>,
        2,
        0x100,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1154usize))
        }
    }

    #[doc = "Gamma %s Register Update Control Register"]
    #[inline(always)]
    pub const fn gam_latch(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamLatch_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1300usize))
        }
    }

    #[doc = "Gamma Correction Block Function Switch Register"]
    #[inline(always)]
    pub const fn gam_sw(&self) -> &'static crate::common::Reg<self::GamSw_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GamSw_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4868usize),
            )
        }
    }

    #[doc = "Gamma %s Correction Block Table Setting Register 1"]
    #[inline(always)]
    pub const fn gam_lut1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamLut1_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1308usize))
        }
    }

    #[doc = "Gamma %s Correction Block Table Setting Register 2"]
    #[inline(always)]
    pub const fn gam_lut2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamLut2_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x130cusize))
        }
    }

    #[doc = "Gamma %s Correction Block Table Setting Register 3"]
    #[inline(always)]
    pub const fn gam_lut3(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamLut3_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1310usize))
        }
    }

    #[doc = "Gamma %s Correction Block Table Setting Register 4"]
    #[inline(always)]
    pub const fn gam_lut4(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamLut4_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1314usize))
        }
    }

    #[doc = "Gamma %s Correction Block Table Setting Register 5"]
    #[inline(always)]
    pub const fn gam_lut5(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamLut5_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1318usize))
        }
    }

    #[doc = "Gamma %s Correction Block Table Setting Register 6"]
    #[inline(always)]
    pub const fn gam_lut6(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamLut6_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x131cusize))
        }
    }

    #[doc = "Gamma %s Correction Block Table Setting Register 7"]
    #[inline(always)]
    pub const fn gam_lut7(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamLut7_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1320usize))
        }
    }

    #[doc = "Gamma %s Correction Block Table Setting Register 8"]
    #[inline(always)]
    pub const fn gam_lut8(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamLut8_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1324usize))
        }
    }

    #[doc = "Gamma %s Correction Block Area Setting Register 1"]
    #[inline(always)]
    pub const fn gam_area1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamArea1_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1328usize))
        }
    }

    #[doc = "Gamma %s Correction Block Area Setting Register 2"]
    #[inline(always)]
    pub const fn gam_area2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamArea2_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x132cusize))
        }
    }

    #[doc = "Gamma %s Correction Block Area Setting Register 3"]
    #[inline(always)]
    pub const fn gam_area3(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamArea3_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1330usize))
        }
    }

    #[doc = "Gamma %s Correction Block Area Setting Register 4"]
    #[inline(always)]
    pub const fn gam_area4(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamArea4_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1334usize))
        }
    }

    #[doc = "Gamma %s Correction Block Area Setting Register 5"]
    #[inline(always)]
    pub const fn gam_area5(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::GamArea5_SPEC, crate::common::RW>,
        3,
        0x40,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1338usize))
        }
    }

    #[doc = "Output Control Block Register Update Control Register"]
    #[inline(always)]
    pub const fn out_vlatch(
        &self,
    ) -> &'static crate::common::Reg<self::OutVlatch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OutVlatch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5056usize),
            )
        }
    }

    #[doc = "Output Control Block Output Interface Register"]
    #[inline(always)]
    pub const fn out_set(
        &self,
    ) -> &'static crate::common::Reg<self::OutSet_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OutSet_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5060usize),
            )
        }
    }

    #[doc = "Output Control Block Brightness Correction Register 1"]
    #[inline(always)]
    pub const fn out_bright1(
        &self,
    ) -> &'static crate::common::Reg<self::OutBright1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OutBright1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5064usize),
            )
        }
    }

    #[doc = "Output Control Block Brightness Correction Register 2"]
    #[inline(always)]
    pub const fn out_bright2(
        &self,
    ) -> &'static crate::common::Reg<self::OutBright2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OutBright2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5068usize),
            )
        }
    }

    #[doc = "Output Control Block Contrast Correction Register"]
    #[inline(always)]
    pub const fn out_contrast(
        &self,
    ) -> &'static crate::common::Reg<self::OutContrast_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OutContrast_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5072usize),
            )
        }
    }

    #[doc = "Output Control Block Panel Dither Correction Register"]
    #[inline(always)]
    pub const fn out_pdtha(
        &self,
    ) -> &'static crate::common::Reg<self::OutPdtha_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OutPdtha_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5076usize),
            )
        }
    }

    #[doc = "Output Control Block Output Phase Control Register"]
    #[inline(always)]
    pub const fn out_clkphase(
        &self,
    ) -> &'static crate::common::Reg<self::OutClkphase_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OutClkphase_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5092usize),
            )
        }
    }

    #[doc = "TCON VLATCH Register"]
    #[inline(always)]
    pub const fn tcon_vlatch(
        &self,
    ) -> &'static crate::common::Reg<self::TconVlatch_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TconVlatch_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5120usize),
            )
        }
    }

    #[doc = "TCON Reference Timing Setting Register"]
    #[inline(always)]
    pub const fn tcon_tim(
        &self,
    ) -> &'static crate::common::Reg<self::TconTim_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TconTim_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5124usize),
            )
        }
    }

    #[doc = "TCON Vertical Timing Setting Register %s1"]
    #[inline(always)]
    pub const fn tcon_stv1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::TconStv1_SPEC, crate::common::RW>,
        2,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1408usize))
        }
    }

    #[doc = "TCON Vertical Timing Setting Register %s2"]
    #[inline(always)]
    pub const fn tcon_stv2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::TconStv2_SPEC, crate::common::RW>,
        2,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x140cusize))
        }
    }

    #[doc = "TCON Horizontal Timing Setting Register STH%s1"]
    #[inline(always)]
    pub const fn tcon_sth1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::TconSth1_SPEC, crate::common::RW>,
        2,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x1418usize))
        }
    }

    #[doc = "TCON Horizontal Timing Setting Register STH%s2"]
    #[inline(always)]
    pub const fn tcon_sth2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::TconSth2_SPEC, crate::common::RW>,
        2,
        0x8,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x141cusize))
        }
    }

    #[doc = "TCON Data Enable Polarity Setting Register"]
    #[inline(always)]
    pub const fn tcon_de(
        &self,
    ) -> &'static crate::common::Reg<self::TconDe_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TconDe_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5160usize),
            )
        }
    }

    #[doc = "System Control Block State Detection Control Register"]
    #[inline(always)]
    pub const fn syscnt_dtcten(
        &self,
    ) -> &'static crate::common::Reg<self::SyscntDtcten_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SyscntDtcten_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5184usize),
            )
        }
    }

    #[doc = "System Control Block Interrupt Request Enable Control Register"]
    #[inline(always)]
    pub const fn syscnt_inten(
        &self,
    ) -> &'static crate::common::Reg<self::SyscntInten_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SyscntInten_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5188usize),
            )
        }
    }

    #[doc = "System Control Block Status Clear Register"]
    #[inline(always)]
    pub const fn syscnt_stclr(
        &self,
    ) -> &'static crate::common::Reg<self::SyscntStclr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SyscntStclr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5192usize),
            )
        }
    }

    #[doc = "System Control Block Status Monitor Register"]
    #[inline(always)]
    pub const fn syscnt_stmon(
        &self,
    ) -> &'static crate::common::Reg<self::SyscntStmon_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::SyscntStmon_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(5196usize),
            )
        }
    }

    #[doc = "System Control Block Version and Panel Clock Control Register"]
    #[inline(always)]
    pub const fn syscnt_panel_clk(
        &self,
    ) -> &'static crate::common::Reg<self::SyscntPanelClk_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SyscntPanelClk_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5200usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gr1Clut0_SPEC;
impl crate::sealed::RegSpec for Gr1Clut0_SPEC {
    type DataType = u32;
}
#[doc = "Color Palette 0 Plane for Graphics 1 Plane"]
pub type Gr1Clut0 = crate::RegValueT<Gr1Clut0_SPEC>;

impl Gr1Clut0 {
    #[doc = "B Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    pub fn b(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Gr1Clut0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Gr1Clut0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "G Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    pub fn g(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Gr1Clut0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Gr1Clut0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "R Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    pub fn r(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Gr1Clut0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Gr1Clut0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Alpha Blending Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    pub fn a(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Gr1Clut0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Gr1Clut0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gr1Clut0 {
    #[inline(always)]
    fn default() -> Gr1Clut0 {
        <crate::RegValueT<Gr1Clut0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gr1Clut1_SPEC;
impl crate::sealed::RegSpec for Gr1Clut1_SPEC {
    type DataType = u32;
}
#[doc = "Color Palette 1 Plane for Graphics 1 Plane"]
pub type Gr1Clut1 = crate::RegValueT<Gr1Clut1_SPEC>;

impl Gr1Clut1 {
    #[doc = "B Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    pub fn b(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Gr1Clut1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Gr1Clut1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "G Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    pub fn g(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Gr1Clut1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Gr1Clut1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "R Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    pub fn r(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Gr1Clut1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Gr1Clut1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Alpha Blending Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    pub fn a(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Gr1Clut1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Gr1Clut1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gr1Clut1 {
    #[inline(always)]
    fn default() -> Gr1Clut1 {
        <crate::RegValueT<Gr1Clut1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gr2Clut0_SPEC;
impl crate::sealed::RegSpec for Gr2Clut0_SPEC {
    type DataType = u32;
}
#[doc = "Color Palette 0 Plane for Graphics 2 Plane"]
pub type Gr2Clut0 = crate::RegValueT<Gr2Clut0_SPEC>;

impl Gr2Clut0 {
    #[doc = "B Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    pub fn b(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Gr2Clut0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Gr2Clut0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "G Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    pub fn g(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Gr2Clut0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Gr2Clut0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "R Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    pub fn r(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Gr2Clut0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Gr2Clut0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Alpha Blending Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    pub fn a(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Gr2Clut0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Gr2Clut0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gr2Clut0 {
    #[inline(always)]
    fn default() -> Gr2Clut0 {
        <crate::RegValueT<Gr2Clut0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gr2Clut1_SPEC;
impl crate::sealed::RegSpec for Gr2Clut1_SPEC {
    type DataType = u32;
}
#[doc = "Color Palette 1 Plane for Graphics 2 Plane"]
pub type Gr2Clut1 = crate::RegValueT<Gr2Clut1_SPEC>;

impl Gr2Clut1 {
    #[doc = "B Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    pub fn b(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Gr2Clut1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Gr2Clut1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "G Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    pub fn g(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Gr2Clut1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Gr2Clut1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "R Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    pub fn r(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Gr2Clut1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Gr2Clut1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Alpha Blending Value of Color Palette n Plane for Graphics m Plane"]
    #[inline(always)]
    pub fn a(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Gr2Clut1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Gr2Clut1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gr2Clut1 {
    #[inline(always)]
    fn default() -> Gr2Clut1 {
        <crate::RegValueT<Gr2Clut1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BgEn_SPEC;
impl crate::sealed::RegSpec for BgEn_SPEC {
    type DataType = u32;
}
#[doc = "Background Plane Setting Operation Control Register"]
pub type BgEn = crate::RegValueT<BgEn_SPEC>;

impl BgEn {
    #[doc = "Background plane generation module operation enable"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, bg_en::En, BgEn_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,bg_en::En, BgEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Control of LCDC internal register value reflection to internal operations"]
    #[inline(always)]
    pub fn ven(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, bg_en::Ven, BgEn_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1,1,0,bg_en::Ven, BgEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Entire module SW reset control"]
    #[inline(always)]
    pub fn swrst(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, bg_en::Swrst, BgEn_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,bg_en::Swrst, BgEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 000000000000000. The write value should be 000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<17, 0x7fff, 1, 0, u16, BgEn_SPEC, crate::common::RW> {
        crate::common::RegisterField::<17,0x7fff,1,0,u16, BgEn_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BgEn {
    #[inline(always)]
    fn default() -> BgEn {
        <crate::RegValueT<BgEn_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bg_en {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "Enables operation."]
        pub const _1: Self = Self::new(1);
        #[doc = "Disables operation."]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ven_SPEC;
    pub type Ven = crate::EnumBitfieldStruct<u8, Ven_SPEC>;
    impl Ven {
        #[doc = "Enables"]
        pub const _1: Self = Self::new(1);
        #[doc = "Disables(Cleared to 0 by an internal source)"]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Swrst_SPEC;
    pub type Swrst = crate::EnumBitfieldStruct<u8, Swrst_SPEC>;
    impl Swrst {
        #[doc = "Releases the entire module from the SW reset state."]
        pub const _1: Self = Self::new(1);
        #[doc = "Places the entire module in the SW reset state."]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BgPeri_SPEC;
impl crate::sealed::RegSpec for BgPeri_SPEC {
    type DataType = u32;
}
#[doc = "Background Plane Setting Free-Running Period Register"]
pub type BgPeri = crate::RegValueT<BgPeri_SPEC>;

impl BgPeri {
    #[doc = "Background plane horizontal synchronization signal period on the basis of pixel clock (PXCLK)."]
    #[inline(always)]
    pub fn fh(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, bg_peri::Fh, BgPeri_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,bg_peri::Fh, BgPeri_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Background plane vertical synchronization signal period on the basis of line."]
    #[inline(always)]
    pub fn fv(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, bg_peri::Fv, BgPeri_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,bg_peri::Fv, BgPeri_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, BgPeri_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8, BgPeri_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BgPeri {
    #[inline(always)]
    fn default() -> BgPeri {
        <crate::RegValueT<BgPeri_SPEC> as RegisterValue<_>>::new(1507351)
    }
}
pub mod bg_peri {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fh_SPEC;
    pub type Fh = crate::EnumBitfieldStruct<u8, Fh_SPEC>;
    impl Fh {
        #[doc = "FH lines. The valid range is 0x017 to 0x3FF."]
        pub const FH: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fv_SPEC;
    pub type Fv = crate::EnumBitfieldStruct<u8, Fv_SPEC>;
    impl Fv {
        #[doc = "FV lines.The valid range is 0x013 to 0x3FF."]
        pub const FV: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BgSync_SPEC;
impl crate::sealed::RegSpec for BgSync_SPEC {
    type DataType = u32;
}
#[doc = "Background Plane Setting Synchronization Position Register"]
pub type BgSync = crate::RegValueT<BgSync_SPEC>;

impl BgSync {
    #[doc = "Background plane horizontal synchronization signal assertion position on the basis of pixel clock (PXCLK)."]
    #[inline(always)]
    pub fn hp(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, bg_sync::Hp, BgSync_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,bg_sync::Hp, BgSync_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Background plane vertical synchronization signal assertion position on the basis of line."]
    #[inline(always)]
    pub fn vp(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, bg_sync::Vp, BgSync_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xf,1,0,bg_sync::Vp, BgSync_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 000000000000. The write value should be 000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<20, 0xfff, 1, 0, u16, BgSync_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xfff,1,0,u16, BgSync_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BgSync {
    #[inline(always)]
    fn default() -> BgSync {
        <crate::RegValueT<BgSync_SPEC> as RegisterValue<_>>::new(65537)
    }
}
pub mod bg_sync {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hp_SPEC;
    pub type Hp = crate::EnumBitfieldStruct<u8, Hp_SPEC>;
    impl Hp {
        #[doc = "Setting prohibited"]
        pub const _0_X_0: Self = Self::new(0);
        #[doc = "(HP)th line (pixels)"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vp_SPEC;
    pub type Vp = crate::EnumBitfieldStruct<u8, Vp_SPEC>;
    impl Vp {
        #[doc = "Setting prohibited"]
        pub const _0_X_0: Self = Self::new(0);
        #[doc = "(VP)th line"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BgVsize_SPEC;
impl crate::sealed::RegSpec for BgVsize_SPEC {
    type DataType = u32;
}
#[doc = "Background Plane Setting Full Image Vertical Size Register"]
pub type BgVsize = crate::RegValueT<BgVsize_SPEC>;

impl BgVsize {
    #[doc = "Background plane vertical valid pixel width on the basis of line"]
    #[inline(always)]
    pub fn vw(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, bg_vsize::Vw, BgVsize_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,bg_vsize::Vw, BgVsize_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Background plane vertical valid pixel start position on the basis of line"]
    #[inline(always)]
    pub fn vp(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, bg_vsize::Vp, BgVsize_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,bg_vsize::Vp, BgVsize_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, BgVsize_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8, BgVsize_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BgVsize {
    #[inline(always)]
    fn default() -> BgVsize {
        <crate::RegValueT<BgVsize_SPEC> as RegisterValue<_>>::new(458768)
    }
}
pub mod bg_vsize {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vw_SPEC;
    pub type Vw = crate::EnumBitfieldStruct<u8, Vw_SPEC>;
    impl Vw {
        #[doc = "VW lines. The valid range is 0x010 to 0x3F0."]
        pub const VW: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vp_SPEC;
    pub type Vp = crate::EnumBitfieldStruct<u8, Vp_SPEC>;
    impl Vp {
        #[doc = "VP lines. The valid range is 0x003 to 0x3EF."]
        pub const VP: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BgHsize_SPEC;
impl crate::sealed::RegSpec for BgHsize_SPEC {
    type DataType = u32;
}
#[doc = "Background Plane Setting Full Image Horizontal Size Register"]
pub type BgHsize = crate::RegValueT<BgHsize_SPEC>;

impl BgHsize {
    #[doc = "Background plane horizontall valid pixel width on the basis of pixel clock (PXCLK)Note: When serial RGB is selected as the output format for the output control block, add two to the horizontal enable signal width and set the resulting value to this field."]
    #[inline(always)]
    pub fn hw(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, bg_hsize::Hw, BgHsize_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,bg_hsize::Hw, BgHsize_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Background plane horizontal valid pixel start position on the basis of pixel clock (PXCLK)."]
    #[inline(always)]
    pub fn hp(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, bg_hsize::Hp, BgHsize_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,bg_hsize::Hp, BgHsize_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, BgHsize_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8, BgHsize_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BgHsize {
    #[inline(always)]
    fn default() -> BgHsize {
        <crate::RegValueT<BgHsize_SPEC> as RegisterValue<_>>::new(393232)
    }
}
pub mod bg_hsize {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hw_SPEC;
    pub type Hw = crate::EnumBitfieldStruct<u8, Hw_SPEC>;
    impl Hw {
        #[doc = "HW cycles. The valid range is 0x010 to 0x3F8."]
        pub const HW: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hp_SPEC;
    pub type Hp = crate::EnumBitfieldStruct<u8, Hp_SPEC>;
    impl Hp {
        #[doc = "HP cycle(pixel). The valid range is 0x006 to 0x3EE."]
        pub const HP: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BgBgc_SPEC;
impl crate::sealed::RegSpec for BgBgc_SPEC {
    type DataType = u32;
}
#[doc = "Background Plane Setting Background Color Register"]
pub type BgBgc = crate::RegValueT<BgBgc_SPEC>;

impl BgBgc {
    #[doc = "B value for background plane valid pixel areaUnsigned; 8-bit integer"]
    #[inline(always)]
    pub fn b(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, BgBgc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, BgBgc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "G value for background plane valid pixel areaUnsigned; 8-bit integer"]
    #[inline(always)]
    pub fn g(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, BgBgc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, BgBgc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "R value for background plane valid pixel area.Unsigned; 8-bit integer."]
    #[inline(always)]
    pub fn r(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, BgBgc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, BgBgc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, BgBgc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, BgBgc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BgBgc {
    #[inline(always)]
    fn default() -> BgBgc {
        <crate::RegValueT<BgBgc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BgMon_SPEC;
impl crate::sealed::RegSpec for BgMon_SPEC {
    type DataType = u32;
}
#[doc = "Background Plane Setting Status Monitor Register"]
pub type BgMon = crate::RegValueT<BgMon_SPEC>;

impl BgMon {
    #[doc = "Background plane generation module operation state monitor."]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, bg_mon::En, BgMon_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1,1,0,bg_mon::En, BgMon_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Entire module internal operation reflection control signal monitor.The signal  state for controlling reflection of the register values to the internal operations upon assertion of the vertical synchronization signal."]
    #[inline(always)]
    pub fn ven(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, bg_mon::Ven, BgMon_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x1,1,0,bg_mon::Ven, BgMon_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Entire module SW reset state monitor."]
    #[inline(always)]
    pub fn swrst(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, bg_mon::Swrst, BgMon_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x1,1,0,bg_mon::Swrst, BgMon_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "These bits are read as 000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<17, 0x7fff, 1, 0, u16, BgMon_SPEC, crate::common::R> {
        crate::common::RegisterField::<17,0x7fff,1,0,u16, BgMon_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for BgMon {
    #[inline(always)]
    fn default() -> BgMon {
        <crate::RegValueT<BgMon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bg_mon {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "Operation is in progress."]
        pub const _1: Self = Self::new(1);
        #[doc = "Operation is stopped."]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ven_SPEC;
    pub type Ven = crate::EnumBitfieldStruct<u8, Ven_SPEC>;
    impl Ven {
        #[doc = "The signal for controlling reflection of the register values to the internal operations upon assertion of the vertical synchronization signal is asserted."]
        pub const _1: Self = Self::new(1);
        #[doc = "The signal for controlling reflection of the register values to the internal operations upon assertion of the vertical synchronization signal is negated."]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Swrst_SPEC;
    pub type Swrst = crate::EnumBitfieldStruct<u8, Swrst_SPEC>;
    impl Swrst {
        #[doc = "The entire module is released from the SW reset state."]
        pub const _1: Self = Self::new(1);
        #[doc = "The entire module is in the SW reset state."]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrVen_SPEC;
impl crate::sealed::RegSpec for GrVen_SPEC {
    type DataType = u32;
}
#[doc = "Graphics %s  Register Update Control Register"]
pub type GrVen = crate::RegValueT<GrVen_SPEC>;

impl GrVen {
    #[doc = "Control of graphics n module register value reflection to internal operations.Reflection of the register values to the internal operation at the assertion of the vertical synchronization signal (VS)."]
    #[inline(always)]
    pub fn pven(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, gr_ven::Pven, GrVen_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,gr_ven::Pven, GrVen_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 0000000000000000000000000000000. The write value should be 0000000000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7fffffff, 1, 0, u32, GrVen_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7fffffff,1,0,u32, GrVen_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrVen {
    #[inline(always)]
    fn default() -> GrVen {
        <crate::RegValueT<GrVen_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gr_ven {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pven_SPEC;
    pub type Pven = crate::EnumBitfieldStruct<u8, Pven_SPEC>;
    impl Pven {
        #[doc = "Enables reflection of the register values to the internal operation at the assertion of the vertical synchronization signal (VS)."]
        pub const _1: Self = Self::new(1);
        #[doc = "Disables reflection of the register values to the internal operation at the assertion of the vertical synchronization signal (VS)."]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrFlmrd_SPEC;
impl crate::sealed::RegSpec for GrFlmrd_SPEC {
    type DataType = u32;
}
#[doc = "Graphics %s Frame Buffer Read Control Register"]
pub type GrFlmrd = crate::RegValueT<GrFlmrd_SPEC>;

impl GrFlmrd {
    #[doc = "Graphics data (frame buffer data) read enable."]
    #[inline(always)]
    pub fn renb(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, gr_flmrd::Renb, GrFlmrd_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,gr_flmrd::Renb, GrFlmrd_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, GrFlmrd_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, GrFlmrd_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrFlmrd {
    #[inline(always)]
    fn default() -> GrFlmrd {
        <crate::RegValueT<GrFlmrd_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gr_flmrd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Renb_SPEC;
    pub type Renb = crate::EnumBitfieldStruct<u8, Renb_SPEC>;
    impl Renb {
        #[doc = "Enables reading."]
        pub const _1: Self = Self::new(1);
        #[doc = "Disables reading."]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrFlm1_SPEC;
impl crate::sealed::RegSpec for GrFlm1_SPEC {
    type DataType = u32;
}
#[doc = "Graphics %s Frame Buffer Control Register 1"]
pub type GrFlm1 = crate::RegValueT<GrFlm1_SPEC>;

impl GrFlm1 {
    #[doc = "Burst transfer control for graphics data (frame buffer data)access"]
    #[inline(always)]
    pub fn bstmd(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, gr_flm1::Bstmd, GrFlm1_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3,1,0,gr_flm1::Bstmd, GrFlm1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "These bits are read as 000000000000000000000000000000. The write value should be 000000000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3fffffff, 1, 0, u32, GrFlm1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3fffffff,1,0,u32, GrFlm1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrFlm1 {
    #[inline(always)]
    fn default() -> GrFlm1 {
        <crate::RegValueT<GrFlm1_SPEC> as RegisterValue<_>>::new(3)
    }
}
pub mod gr_flm1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bstmd_SPEC;
    pub type Bstmd = crate::EnumBitfieldStruct<u8, Bstmd_SPEC>;
    impl Bstmd {
        #[doc = "16-beat increment burst transfer (64-byte boundary)"]
        pub const _11: Self = Self::new(3);
        #[doc = "Setting prohibited."]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrFlm2_SPEC;
impl crate::sealed::RegSpec for GrFlm2_SPEC {
    type DataType = u32;
}
#[doc = "Graphics %s Frame Buffer Control Register 2"]
pub type GrFlm2 = crate::RegValueT<GrFlm2_SPEC>;

impl GrFlm2 {
    #[doc = "Base address for accessing graphics data (frame buffer data)Set the head address in the frame buffer where graphics data is to be stored. GRn_FLM2.BASE\\[5:0\\] should be fixed to 0 during 64-byte burst transfer."]
    #[inline(always)]
    pub fn base(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, GrFlm2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, GrFlm2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrFlm2 {
    #[inline(always)]
    fn default() -> GrFlm2 {
        <crate::RegValueT<GrFlm2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrFlm3_SPEC;
impl crate::sealed::RegSpec for GrFlm3_SPEC {
    type DataType = u32;
}
#[doc = "Graphics %s Frame Buffer Control Register 3"]
pub type GrFlm3 = crate::RegValueT<GrFlm3_SPEC>;

impl GrFlm3 {
    #[doc = "Macro line offset address for accessing graphics data(frame buffer data)Signed; 16-bit integer"]
    #[inline(always)]
    pub fn lnoff(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, GrFlm3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, GrFlm3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrFlm3 {
    #[inline(always)]
    fn default() -> GrFlm3 {
        <crate::RegValueT<GrFlm3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrFlm5_SPEC;
impl crate::sealed::RegSpec for GrFlm5_SPEC {
    type DataType = u32;
}
#[doc = "Graphics %s Frame Buffer Control Register 5"]
pub type GrFlm5 = crate::RegValueT<GrFlm5_SPEC>;

impl GrFlm5 {
    #[doc = "Number of data transfer times per line for accessing graphics data (frame buffer data), where one transfer is defined as 16-beat burst access (64-byte boundary)"]
    #[inline(always)]
    pub fn datanum(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        gr_flm5::Datanum,
        GrFlm5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            gr_flm5::Datanum,
            GrFlm5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Number of lines per frame for accessing graphics data (frame buffer data)."]
    #[inline(always)]
    pub fn lnnum(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, gr_flm5::Lnnum, GrFlm5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            16,
            0x7ff,
            1,
            0,
            gr_flm5::Lnnum,
            GrFlm5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for GrFlm5 {
    #[inline(always)]
    fn default() -> GrFlm5 {
        <crate::RegValueT<GrFlm5_SPEC> as RegisterValue<_>>::new(983040)
    }
}
pub mod gr_flm5 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Datanum_SPEC;
    pub type Datanum = crate::EnumBitfieldStruct<u8, Datanum_SPEC>;
    impl Datanum {
        #[doc = "DATAUM+1 times."]
        pub const DATAUM: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lnnum_SPEC;
    pub type Lnnum = crate::EnumBitfieldStruct<u8, Lnnum_SPEC>;
    impl Lnnum {
        #[doc = "LNNUM lines.  The valid range is 0x00F to 0x3FF."]
        pub const LNNUM: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrFlm6_SPEC;
impl crate::sealed::RegSpec for GrFlm6_SPEC {
    type DataType = u32;
}
#[doc = "Graphics %s Frame Buffer Control Register 6"]
pub type GrFlm6 = crate::RegValueT<GrFlm6_SPEC>;

impl GrFlm6 {
    #[doc = "Data format for accessing graphics data (frame buffer data)."]
    #[inline(always)]
    pub fn format(
        self,
    ) -> crate::common::RegisterField<28, 0x7, 1, 0, gr_flm6::Format, GrFlm6_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x7,1,0,gr_flm6::Format, GrFlm6_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, GrFlm6_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, GrFlm6_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for GrFlm6 {
    #[inline(always)]
    fn default() -> GrFlm6 {
        <crate::RegValueT<GrFlm6_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gr_flm6 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Format_SPEC;
    pub type Format = crate::EnumBitfieldStruct<u8, Format_SPEC>;
    impl Format {
        #[doc = "CLUT11bit/pix)"]
        pub const _111: Self = Self::new(7);
        #[doc = "CLUT4 (4 bits/pix)"]
        pub const _110: Self = Self::new(6);
        #[doc = "CLUT8 (8 bits/pix)"]
        pub const _101: Self = Self::new(5);
        #[doc = "ARGB8888 (32 bits/pix)"]
        pub const _100: Self = Self::new(4);
        #[doc = "ARGB4444 (16 bits/pix)"]
        pub const _011: Self = Self::new(3);
        #[doc = "ARGB1555 (16 bits/pix, 1 bit of A is LUT data)"]
        pub const _010: Self = Self::new(2);
        #[doc = "RGB888 (32 bits/pix, 8 bits on the MSB side are invalid)"]
        pub const _001: Self = Self::new(1);
        #[doc = "RGB565 (16 bits/pix)"]
        pub const _000: Self = Self::new(0);
        #[doc = "Setting prohibited."]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrAb1_SPEC;
impl crate::sealed::RegSpec for GrAb1_SPEC {
    type DataType = u32;
}
#[doc = "Graphics %s Alpha Blending Control Register 1"]
pub type GrAb1 = crate::RegValueT<GrAb1_SPEC>;

impl GrAb1 {
    #[doc = "Graphics display plane control."]
    #[inline(always)]
    pub fn dispsel(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, gr_ab1::Dispsel, GrAb1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,gr_ab1::Dispsel, GrAb1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Graphics image area border display control."]
    #[inline(always)]
    pub fn grcdispon(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, gr_ab1::Grcdispon, GrAb1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,gr_ab1::Grcdispon, GrAb1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Image area border display control for rectangular area alpha blending."]
    #[inline(always)]
    pub fn arcdispon(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, gr_ab1::Arcdispon, GrAb1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,gr_ab1::Arcdispon, GrAb1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Rectangular area alpha blending control."]
    #[inline(always)]
    pub fn arcon(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, gr_ab1::Arcon, GrAb1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,gr_ab1::Arcon, GrAb1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 0000000000000000000. The write value should be 0000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<13, 0x7ffff, 1, 0, u32, GrAb1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x7ffff,1,0,u32, GrAb1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrAb1 {
    #[inline(always)]
    fn default() -> GrAb1 {
        <crate::RegValueT<GrAb1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gr_ab1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dispsel_SPEC;
    pub type Dispsel = crate::EnumBitfieldStruct<u8, Dispsel_SPEC>;
    impl Dispsel {
        #[doc = "Blended display of lower-layer graphics (input image from the previous stage) and current graphics (graphics data read from the AHB bus)"]
        pub const _11: Self = Self::new(3);
        #[doc = "Current graphics display"]
        pub const _10: Self = Self::new(2);
        #[doc = "Lower-layer graphics display"]
        pub const _01: Self = Self::new(1);
        #[doc = "Background color display (value set by the GRn_BASE register)."]
        pub const _00: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Grcdispon_SPEC;
    pub type Grcdispon = crate::EnumBitfieldStruct<u8, Grcdispon_SPEC>;
    impl Grcdispon {
        #[doc = "Display on"]
        pub const _1: Self = Self::new(1);
        #[doc = "Display off"]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arcdispon_SPEC;
    pub type Arcdispon = crate::EnumBitfieldStruct<u8, Arcdispon_SPEC>;
    impl Arcdispon {
        #[doc = "Display on"]
        pub const _1: Self = Self::new(1);
        #[doc = "Display off"]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arcon_SPEC;
    pub type Arcon = crate::EnumBitfieldStruct<u8, Arcon_SPEC>;
    impl Arcon {
        #[doc = "On"]
        pub const _1: Self = Self::new(1);
        #[doc = "Off"]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrAb2_SPEC;
impl crate::sealed::RegSpec for GrAb2_SPEC {
    type DataType = u32;
}
#[doc = "Graphics %s Alpha Blending Control Register 2"]
pub type GrAb2 = crate::RegValueT<GrAb2_SPEC>;

impl GrAb2 {
    #[doc = "Vertical width of graphics image area."]
    #[inline(always)]
    pub fn grcvw(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, gr_ab2::Grcvw, GrAb2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,gr_ab2::Grcvw, GrAb2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Vertical start position of graphics image area."]
    #[inline(always)]
    pub fn grcvs(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, gr_ab2::Grcvs, GrAb2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,gr_ab2::Grcvs, GrAb2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, GrAb2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8, GrAb2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrAb2 {
    #[inline(always)]
    fn default() -> GrAb2 {
        <crate::RegValueT<GrAb2_SPEC> as RegisterValue<_>>::new(393232)
    }
}
pub mod gr_ab2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Grcvw_SPEC;
    pub type Grcvw = crate::EnumBitfieldStruct<u8, Grcvw_SPEC>;
    impl Grcvw {
        #[doc = "GRCVW lines.  The valid range is 0x010 to 0x3F0."]
        pub const GRCVW: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Grcvs_SPEC;
    pub type Grcvs = crate::EnumBitfieldStruct<u8, Grcvs_SPEC>;
    impl Grcvs {
        #[doc = "GRCVS lines. The valid range is 0x002 to 0x3EE."]
        pub const GRCVS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrAb3_SPEC;
impl crate::sealed::RegSpec for GrAb3_SPEC {
    type DataType = u32;
}
#[doc = "Graphics %s Alpha Blending Control Register 3"]
pub type GrAb3 = crate::RegValueT<GrAb3_SPEC>;

impl GrAb3 {
    #[doc = "Horizontal width of graphics image area."]
    #[inline(always)]
    pub fn grchw(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, gr_ab3::Grchw, GrAb3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,gr_ab3::Grchw, GrAb3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Horizontal start position of graphics image area."]
    #[inline(always)]
    pub fn grchs(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, gr_ab3::Grchs, GrAb3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,gr_ab3::Grchs, GrAb3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, GrAb3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8, GrAb3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrAb3 {
    #[inline(always)]
    fn default() -> GrAb3 {
        <crate::RegValueT<GrAb3_SPEC> as RegisterValue<_>>::new(327696)
    }
}
pub mod gr_ab3 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Grchw_SPEC;
    pub type Grchw = crate::EnumBitfieldStruct<u8, Grchw_SPEC>;
    impl Grchw {
        #[doc = "GRCHW pixels. The valid range is 0x010 to 0x3F0."]
        pub const GRCHW: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Grchs_SPEC;
    pub type Grchs = crate::EnumBitfieldStruct<u8, Grchs_SPEC>;
    impl Grchs {
        #[doc = "GRCHS lines. The valid range is 0x005 to 0x3ED."]
        pub const GRCHS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrAb4_SPEC;
impl crate::sealed::RegSpec for GrAb4_SPEC {
    type DataType = u32;
}
#[doc = "Graphics %s Alpha Blending Control Register 4"]
pub type GrAb4 = crate::RegValueT<GrAb4_SPEC>;

impl GrAb4 {
    #[doc = "Vertical width of rectangular area alpha blending image area."]
    #[inline(always)]
    pub fn arcvw(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, gr_ab4::Arcvw, GrAb4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,gr_ab4::Arcvw, GrAb4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Vertical start position of rectangular area alpha blending image area"]
    #[inline(always)]
    pub fn arcvs(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, gr_ab4::Arcvs, GrAb4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,gr_ab4::Arcvs, GrAb4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, GrAb4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8, GrAb4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrAb4 {
    #[inline(always)]
    fn default() -> GrAb4 {
        <crate::RegValueT<GrAb4_SPEC> as RegisterValue<_>>::new(393232)
    }
}
pub mod gr_ab4 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arcvw_SPEC;
    pub type Arcvw = crate::EnumBitfieldStruct<u8, Arcvw_SPEC>;
    impl Arcvw {
        #[doc = "ARCVW linels. The valid range is 0x001 to 0x3F0."]
        pub const ARCVW: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arcvs_SPEC;
    pub type Arcvs = crate::EnumBitfieldStruct<u8, Arcvs_SPEC>;
    impl Arcvs {
        #[doc = "ARCVS linels. The valid range is 0x002 to 0x3EE."]
        pub const ARCVS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrAb5_SPEC;
impl crate::sealed::RegSpec for GrAb5_SPEC {
    type DataType = u32;
}
#[doc = "Graphics %s Alpha Blending Control Register 5"]
pub type GrAb5 = crate::RegValueT<GrAb5_SPEC>;

impl GrAb5 {
    #[doc = "Horizontal width of rectangular area alpha blending image area."]
    #[inline(always)]
    pub fn archw(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, gr_ab5::Archw, GrAb5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,gr_ab5::Archw, GrAb5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Horizontal start position of rectangular area alpha blending image area."]
    #[inline(always)]
    pub fn archs(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, gr_ab5::Archs, GrAb5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,gr_ab5::Archs, GrAb5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, GrAb5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8, GrAb5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrAb5 {
    #[inline(always)]
    fn default() -> GrAb5 {
        <crate::RegValueT<GrAb5_SPEC> as RegisterValue<_>>::new(327696)
    }
}
pub mod gr_ab5 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Archw_SPEC;
    pub type Archw = crate::EnumBitfieldStruct<u8, Archw_SPEC>;
    impl Archw {
        #[doc = "ARCHW pixels. The valid range is 0x001 to 0x3F0."]
        pub const ARCHW: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Archs_SPEC;
    pub type Archs = crate::EnumBitfieldStruct<u8, Archs_SPEC>;
    impl Archs {
        #[doc = "ARCHS pixel. The valid range is 0x005 to 0x3ED."]
        pub const ARCHS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrAb6_SPEC;
impl crate::sealed::RegSpec for GrAb6_SPEC {
    type DataType = u32;
}
#[doc = "Graphics %s Alpha Blending Control Register 6"]
pub type GrAb6 = crate::RegValueT<GrAb6_SPEC>;

impl GrAb6 {
    #[doc = "Frame rate for alpha blending in rectangular area."]
    #[inline(always)]
    pub fn arcrate(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, gr_ab6::Arcrate, GrAb6_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,gr_ab6::Arcrate, GrAb6_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Alpha coefficient for alpha blending in rectangular area (-255 to 255).\\[8\\]: Sign (0: addition, 1: subtraction)\\[7:0\\]: Variation (absolute value)"]
    #[inline(always)]
    pub fn arccoef(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, GrAb6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1ff,1,0,u16, GrAb6_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<25, 0x7f, 1, 0, u8, GrAb6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<25,0x7f,1,0,u8, GrAb6_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrAb6 {
    #[inline(always)]
    fn default() -> GrAb6 {
        <crate::RegValueT<GrAb6_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gr_ab6 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arcrate_SPEC;
    pub type Arcrate = crate::EnumBitfieldStruct<u8, Arcrate_SPEC>;
    impl Arcrate {
        #[doc = "ARCRATE+1 frames"]
        pub const ARCRATE: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrAb7_SPEC;
impl crate::sealed::RegSpec for GrAb7_SPEC {
    type DataType = u32;
}
#[doc = "Graphics %s Alpha Blending Control Register 7"]
pub type GrAb7 = crate::RegValueT<GrAb7_SPEC>;

impl GrAb7 {
    #[doc = "RGB-index chroma-key processing control."]
    #[inline(always)]
    pub fn ckon(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, gr_ab7::Ckon, GrAb7_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,gr_ab7::Ckon, GrAb7_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Initial alpha value for alpha blending in rectangular area."]
    #[inline(always)]
    pub fn arcdef(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, GrAb7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, GrAb7_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, GrAb7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, GrAb7_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrAb7 {
    #[inline(always)]
    fn default() -> GrAb7 {
        <crate::RegValueT<GrAb7_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gr_ab7 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ckon_SPEC;
    pub type Ckon = crate::EnumBitfieldStruct<u8, Ckon_SPEC>;
    impl Ckon {
        #[doc = "Enables chroma-key processing"]
        pub const _1: Self = Self::new(1);
        #[doc = "Disables chroma-key processing"]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrAb8_SPEC;
impl crate::sealed::RegSpec for GrAb8_SPEC {
    type DataType = u32;
}
#[doc = "Graphics %s Alpha Blending Control Register 8"]
pub type GrAb8 = crate::RegValueT<GrAb8_SPEC>;

impl GrAb8 {
    #[doc = "R signal for RGB-index chroma-key processingUnsigned; 8 bits."]
    #[inline(always)]
    pub fn ckkr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, GrAb8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, GrAb8_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "B signal for RGB-index chroma-key processingUnsigned; 8 bits."]
    #[inline(always)]
    pub fn ckkb(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, GrAb8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, GrAb8_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "G signal for RGB-index chroma-key processingUnsigned; 8 bits."]
    #[inline(always)]
    pub fn ckkg(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, GrAb8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, GrAb8_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, GrAb8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, GrAb8_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrAb8 {
    #[inline(always)]
    fn default() -> GrAb8 {
        <crate::RegValueT<GrAb8_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrAb9_SPEC;
impl crate::sealed::RegSpec for GrAb9_SPEC {
    type DataType = u32;
}
#[doc = "Graphics %s Alpha Blending Control Register 9"]
pub type GrAb9 = crate::RegValueT<GrAb9_SPEC>;

impl GrAb9 {
    #[doc = "R value after RGB-index chroma-key processing replacementUnsigned; 8 bits."]
    #[inline(always)]
    pub fn ckr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, GrAb9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, GrAb9_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "B value after RGB-index chroma-key processing replacementUnsigned; 8 bits."]
    #[inline(always)]
    pub fn ckb(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, GrAb9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, GrAb9_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "G value after RGB-index chroma-key processing replacementUnsigned; 8 bits."]
    #[inline(always)]
    pub fn ckg(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, GrAb9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, GrAb9_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "A value after RGB-index chroma-key processing replacement."]
    #[inline(always)]
    pub fn cka(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, GrAb9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, GrAb9_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrAb9 {
    #[inline(always)]
    fn default() -> GrAb9 {
        <crate::RegValueT<GrAb9_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrBase_SPEC;
impl crate::sealed::RegSpec for GrBase_SPEC {
    type DataType = u32;
}
#[doc = "Graphics %s Background Color Control Register"]
pub type GrBase = crate::RegValueT<GrBase_SPEC>;

impl GrBase {
    #[doc = "Background color R valueUnsigned; 8 bits"]
    #[inline(always)]
    pub fn r(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, GrBase_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, GrBase_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Background color B valueUnsigned; 8 bits"]
    #[inline(always)]
    pub fn b(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, GrBase_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, GrBase_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Background color G valueUnsigned; 8 bits"]
    #[inline(always)]
    pub fn g(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, GrBase_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, GrBase_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, GrBase_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, GrBase_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrBase {
    #[inline(always)]
    fn default() -> GrBase {
        <crate::RegValueT<GrBase_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrClutint_SPEC;
impl crate::sealed::RegSpec for GrClutint_SPEC {
    type DataType = u32;
}
#[doc = "Graphics %s CLUT Table Interrupt Control Register"]
pub type GrClutint = crate::RegValueT<GrClutint_SPEC>;

impl GrClutint {
    #[doc = "Number of detection lines"]
    #[inline(always)]
    pub fn line(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ff,
        1,
        0,
        gr_clutint::Line,
        GrClutint_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7ff,
            1,
            0,
            gr_clutint::Line,
            GrClutint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CLUT table control"]
    #[inline(always)]
    pub fn sel(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        gr_clutint::Sel,
        GrClutint_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            gr_clutint::Sel,
            GrClutint_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 000000000000000. The write value should be 000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<17, 0x7fff, 1, 0, u16, GrClutint_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x7fff,1,0,u16, GrClutint_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GrClutint {
    #[inline(always)]
    fn default() -> GrClutint {
        <crate::RegValueT<GrClutint_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gr_clutint {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Line_SPEC;
    pub type Line = crate::EnumBitfieldStruct<u8, Line_SPEC>;
    impl Line {
        #[doc = "LINE+1 lines. The valid range is 0x000 to 0x400."]
        pub const LINE: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sel_SPEC;
    pub type Sel = crate::EnumBitfieldStruct<u8, Sel_SPEC>;
    impl Sel {
        #[doc = "Uses CLUT1 plane for internal operations."]
        pub const _1: Self = Self::new(1);
        #[doc = "Uses CLUT0 plane for internal operations."]
        pub const _0: Self = Self::new(0);
        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrMon_SPEC;
impl crate::sealed::RegSpec for GrMon_SPEC {
    type DataType = u32;
}
#[doc = "Graphics %s Status Monitor Register"]
pub type GrMon = crate::RegValueT<GrMon_SPEC>;

impl GrMon {
    #[doc = "Status monitor for alpha blending in rectangular area"]
    #[inline(always)]
    pub fn arcst(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, gr_mon::Arcst, GrMon_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,gr_mon::Arcst, GrMon_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Status monitor for underflow"]
    #[inline(always)]
    pub fn undflst(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, gr_mon::Undflst, GrMon_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x1,1,0,gr_mon::Undflst, GrMon_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "These bits are read as 000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<17, 0x7fff, 1, 0, u16, GrMon_SPEC, crate::common::R> {
        crate::common::RegisterField::<17,0x7fff,1,0,u16, GrMon_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for GrMon {
    #[inline(always)]
    fn default() -> GrMon {
        <crate::RegValueT<GrMon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gr_mon {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Arcst_SPEC;
    pub type Arcst = crate::EnumBitfieldStruct<u8, Arcst_SPEC>;
    impl Arcst {
        #[doc = "Fade-in/fade-out is in progress."]
        pub const _1: Self = Self::new(1);
        #[doc = "Fade-in/fade-out is not in progress."]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Undflst_SPEC;
    pub type Undflst = crate::EnumBitfieldStruct<u8, Undflst_SPEC>;
    impl Undflst {
        #[doc = "An underflow occurs in internal operations."]
        pub const _1: Self = Self::new(1);
        #[doc = "No underflow occurs in internal operations."]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamLatch_SPEC;
impl crate::sealed::RegSpec for GamLatch_SPEC {
    type DataType = u32;
}
#[doc = "Gamma %s Register Update Control Register"]
pub type GamLatch = crate::RegValueT<GamLatch_SPEC>;

impl GamLatch {
    #[doc = "Control of gamma correction x module register value reflection to internal operations.The register values to be reflected to the internal operations at the assertion of the vertical synchronization signal (VS)."]
    #[inline(always)]
    pub fn ven(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, gam_latch::Ven, GamLatch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,gam_latch::Ven, GamLatch_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 0000000000000000000000000000000. The write value should be 0000000000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7fffffff, 1, 0, u32, GamLatch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7fffffff,1,0,u32, GamLatch_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamLatch {
    #[inline(always)]
    fn default() -> GamLatch {
        <crate::RegValueT<GamLatch_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gam_latch {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ven_SPEC;
    pub type Ven = crate::EnumBitfieldStruct<u8, Ven_SPEC>;
    impl Ven {
        #[doc = "Enables the register values to be reflected to the internal operations at the assertion of the vertical synchronization signal (VS)."]
        pub const _1: Self = Self::new(1);
        #[doc = "Disables the register values to be reflected to the internal operations at the assertion of the vertical synchronization signal (VS)."]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamSw_SPEC;
impl crate::sealed::RegSpec for GamSw_SPEC {
    type DataType = u32;
}
#[doc = "Gamma Correction Block Function Switch Register"]
pub type GamSw = crate::RegValueT<GamSw_SPEC>;

impl GamSw {
    #[doc = "Gamma correction on/off control"]
    #[inline(always)]
    pub fn gamon(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, gam_sw::Gamon, GamSw_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,gam_sw::Gamon, GamSw_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 0000000000000000000000000000000. The write value should be 0000000000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7fffffff, 1, 0, u32, GamSw_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7fffffff,1,0,u32, GamSw_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamSw {
    #[inline(always)]
    fn default() -> GamSw {
        <crate::RegValueT<GamSw_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gam_sw {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gamon_SPEC;
    pub type Gamon = crate::EnumBitfieldStruct<u8, Gamon_SPEC>;
    impl Gamon {
        #[doc = "Turns on gamma correction."]
        pub const _1: Self = Self::new(1);
        #[doc = "Turns off gamma correction."]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamLut1_SPEC;
impl crate::sealed::RegSpec for GamLut1_SPEC {
    type DataType = u32;
}
#[doc = "Gamma %s Correction Block Table Setting Register 1"]
pub type GamLut1 = crate::RegValueT<GamLut1_SPEC>;

impl GamLut1 {
    #[doc = "Gain value of area 1Unsigned 11-bit fixed point"]
    #[inline(always)]
    pub fn gain01(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ff,
        1,
        0,
        gam_lut1::Gain01,
        GamLut1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7ff,
            1,
            0,
            gam_lut1::Gain01,
            GamLut1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Gain value of area 0.Unsigned 11-bit fixed point."]
    #[inline(always)]
    pub fn gain00(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7ff,
        1,
        0,
        gam_lut1::Gain00,
        GamLut1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7ff,
            1,
            0,
            gam_lut1::Gain00,
            GamLut1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, GamLut1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8, GamLut1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamLut1 {
    #[inline(always)]
    fn default() -> GamLut1 {
        <crate::RegValueT<GamLut1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gam_lut1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain01_SPEC;
    pub type Gain01 = crate::EnumBitfieldStruct<u8, Gain01_SPEC>;
    impl Gain01 {
        #[doc = "GAIN01/1024"]
        pub const GAIN_01: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain00_SPEC;
    pub type Gain00 = crate::EnumBitfieldStruct<u8, Gain00_SPEC>;
    impl Gain00 {
        #[doc = "GAIN00/1024"]
        pub const GAIN_00: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamLut2_SPEC;
impl crate::sealed::RegSpec for GamLut2_SPEC {
    type DataType = u32;
}
#[doc = "Gamma %s Correction Block Table Setting Register 2"]
pub type GamLut2 = crate::RegValueT<GamLut2_SPEC>;

impl GamLut2 {
    #[doc = "Gain value of area 3Unsigned 11-bit fixed point"]
    #[inline(always)]
    pub fn gain03(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ff,
        1,
        0,
        gam_lut2::Gain03,
        GamLut2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7ff,
            1,
            0,
            gam_lut2::Gain03,
            GamLut2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Gain value of area 2Unsigned 11-bit fixed point"]
    #[inline(always)]
    pub fn gain02(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7ff,
        1,
        0,
        gam_lut2::Gain02,
        GamLut2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7ff,
            1,
            0,
            gam_lut2::Gain02,
            GamLut2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, GamLut2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8, GamLut2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamLut2 {
    #[inline(always)]
    fn default() -> GamLut2 {
        <crate::RegValueT<GamLut2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gam_lut2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain03_SPEC;
    pub type Gain03 = crate::EnumBitfieldStruct<u8, Gain03_SPEC>;
    impl Gain03 {
        #[doc = "GAIN03/1024"]
        pub const GAIN_03: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain02_SPEC;
    pub type Gain02 = crate::EnumBitfieldStruct<u8, Gain02_SPEC>;
    impl Gain02 {
        #[doc = "GAIN02/1024"]
        pub const GAIN_02: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamLut3_SPEC;
impl crate::sealed::RegSpec for GamLut3_SPEC {
    type DataType = u32;
}
#[doc = "Gamma %s Correction Block Table Setting Register 3"]
pub type GamLut3 = crate::RegValueT<GamLut3_SPEC>;

impl GamLut3 {
    #[doc = "Gain value of area 5Unsigned 11-bit fixed point"]
    #[inline(always)]
    pub fn gain05(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ff,
        1,
        0,
        gam_lut3::Gain05,
        GamLut3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7ff,
            1,
            0,
            gam_lut3::Gain05,
            GamLut3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Gain value of area 4Unsigned 11-bit fixed point"]
    #[inline(always)]
    pub fn gain04(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7ff,
        1,
        0,
        gam_lut3::Gain04,
        GamLut3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7ff,
            1,
            0,
            gam_lut3::Gain04,
            GamLut3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, GamLut3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8, GamLut3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamLut3 {
    #[inline(always)]
    fn default() -> GamLut3 {
        <crate::RegValueT<GamLut3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gam_lut3 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain05_SPEC;
    pub type Gain05 = crate::EnumBitfieldStruct<u8, Gain05_SPEC>;
    impl Gain05 {
        #[doc = "GAIN05/1024"]
        pub const GAIN_05: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain04_SPEC;
    pub type Gain04 = crate::EnumBitfieldStruct<u8, Gain04_SPEC>;
    impl Gain04 {
        #[doc = "GAIN04/1024"]
        pub const GAIN_04: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamLut4_SPEC;
impl crate::sealed::RegSpec for GamLut4_SPEC {
    type DataType = u32;
}
#[doc = "Gamma %s Correction Block Table Setting Register 4"]
pub type GamLut4 = crate::RegValueT<GamLut4_SPEC>;

impl GamLut4 {
    #[doc = "Gain value of area 7Unsigned 11-bit fixed point"]
    #[inline(always)]
    pub fn gain07(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ff,
        1,
        0,
        gam_lut4::Gain07,
        GamLut4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7ff,
            1,
            0,
            gam_lut4::Gain07,
            GamLut4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Gain value of area 6Unsigned 11-bit fixed point"]
    #[inline(always)]
    pub fn gain06(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7ff,
        1,
        0,
        gam_lut4::Gain06,
        GamLut4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7ff,
            1,
            0,
            gam_lut4::Gain06,
            GamLut4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, GamLut4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8, GamLut4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamLut4 {
    #[inline(always)]
    fn default() -> GamLut4 {
        <crate::RegValueT<GamLut4_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gam_lut4 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain07_SPEC;
    pub type Gain07 = crate::EnumBitfieldStruct<u8, Gain07_SPEC>;
    impl Gain07 {
        #[doc = "GAIN07/1024"]
        pub const GAIN_07: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain06_SPEC;
    pub type Gain06 = crate::EnumBitfieldStruct<u8, Gain06_SPEC>;
    impl Gain06 {
        #[doc = "GAIN06/1024"]
        pub const GAIN_06: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamLut5_SPEC;
impl crate::sealed::RegSpec for GamLut5_SPEC {
    type DataType = u32;
}
#[doc = "Gamma %s Correction Block Table Setting Register 5"]
pub type GamLut5 = crate::RegValueT<GamLut5_SPEC>;

impl GamLut5 {
    #[doc = "Gain value of area 9Unsigned 11-bit fixed point"]
    #[inline(always)]
    pub fn gain09(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ff,
        1,
        0,
        gam_lut5::Gain09,
        GamLut5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7ff,
            1,
            0,
            gam_lut5::Gain09,
            GamLut5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Gain value of area 8Unsigned 11-bit fixed point"]
    #[inline(always)]
    pub fn gain08(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7ff,
        1,
        0,
        gam_lut5::Gain08,
        GamLut5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7ff,
            1,
            0,
            gam_lut5::Gain08,
            GamLut5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, GamLut5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8, GamLut5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamLut5 {
    #[inline(always)]
    fn default() -> GamLut5 {
        <crate::RegValueT<GamLut5_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gam_lut5 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain09_SPEC;
    pub type Gain09 = crate::EnumBitfieldStruct<u8, Gain09_SPEC>;
    impl Gain09 {
        #[doc = "GAIN09/1024"]
        pub const GAIN_09: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain08_SPEC;
    pub type Gain08 = crate::EnumBitfieldStruct<u8, Gain08_SPEC>;
    impl Gain08 {
        #[doc = "GAIN08/1024"]
        pub const GAIN_08: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamLut6_SPEC;
impl crate::sealed::RegSpec for GamLut6_SPEC {
    type DataType = u32;
}
#[doc = "Gamma %s Correction Block Table Setting Register 6"]
pub type GamLut6 = crate::RegValueT<GamLut6_SPEC>;

impl GamLut6 {
    #[doc = "Gain value of area 11Unsigned 11-bit fixed point"]
    #[inline(always)]
    pub fn gain11(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ff,
        1,
        0,
        gam_lut6::Gain11,
        GamLut6_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7ff,
            1,
            0,
            gam_lut6::Gain11,
            GamLut6_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Gain value of area 10Unsigned 11-bit fixed point"]
    #[inline(always)]
    pub fn gain10(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7ff,
        1,
        0,
        gam_lut6::Gain10,
        GamLut6_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7ff,
            1,
            0,
            gam_lut6::Gain10,
            GamLut6_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, GamLut6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8, GamLut6_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamLut6 {
    #[inline(always)]
    fn default() -> GamLut6 {
        <crate::RegValueT<GamLut6_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gam_lut6 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain11_SPEC;
    pub type Gain11 = crate::EnumBitfieldStruct<u8, Gain11_SPEC>;
    impl Gain11 {
        #[doc = "GAIN11/1024"]
        pub const GAIN_11: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain10_SPEC;
    pub type Gain10 = crate::EnumBitfieldStruct<u8, Gain10_SPEC>;
    impl Gain10 {
        #[doc = "GAIN10/1024"]
        pub const GAIN_10: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamLut7_SPEC;
impl crate::sealed::RegSpec for GamLut7_SPEC {
    type DataType = u32;
}
#[doc = "Gamma %s Correction Block Table Setting Register 7"]
pub type GamLut7 = crate::RegValueT<GamLut7_SPEC>;

impl GamLut7 {
    #[doc = "Gain value of area 13Unsigned 11-bit fixed point"]
    #[inline(always)]
    pub fn gain13(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ff,
        1,
        0,
        gam_lut7::Gain13,
        GamLut7_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7ff,
            1,
            0,
            gam_lut7::Gain13,
            GamLut7_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Gain value of area 12Unsigned 11-bit fixed point"]
    #[inline(always)]
    pub fn gain12(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7ff,
        1,
        0,
        gam_lut7::Gain12,
        GamLut7_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7ff,
            1,
            0,
            gam_lut7::Gain12,
            GamLut7_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, GamLut7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8, GamLut7_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamLut7 {
    #[inline(always)]
    fn default() -> GamLut7 {
        <crate::RegValueT<GamLut7_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gam_lut7 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain13_SPEC;
    pub type Gain13 = crate::EnumBitfieldStruct<u8, Gain13_SPEC>;
    impl Gain13 {
        #[doc = "GAIN13/1024"]
        pub const GAIN_13: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain12_SPEC;
    pub type Gain12 = crate::EnumBitfieldStruct<u8, Gain12_SPEC>;
    impl Gain12 {
        #[doc = "GAIN12/1024"]
        pub const GAIN_12: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamLut8_SPEC;
impl crate::sealed::RegSpec for GamLut8_SPEC {
    type DataType = u32;
}
#[doc = "Gamma %s Correction Block Table Setting Register 8"]
pub type GamLut8 = crate::RegValueT<GamLut8_SPEC>;

impl GamLut8 {
    #[doc = "Gain value of area 15Unsigned 11-bit fixed point"]
    #[inline(always)]
    pub fn gain15(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ff,
        1,
        0,
        gam_lut8::Gain15,
        GamLut8_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7ff,
            1,
            0,
            gam_lut8::Gain15,
            GamLut8_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Gain value of area 14Unsigned 11-bit fixed point"]
    #[inline(always)]
    pub fn gain14(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7ff,
        1,
        0,
        gam_lut8::Gain14,
        GamLut8_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7ff,
            1,
            0,
            gam_lut8::Gain14,
            GamLut8_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, GamLut8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8, GamLut8_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamLut8 {
    #[inline(always)]
    fn default() -> GamLut8 {
        <crate::RegValueT<GamLut8_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gam_lut8 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain15_SPEC;
    pub type Gain15 = crate::EnumBitfieldStruct<u8, Gain15_SPEC>;
    impl Gain15 {
        #[doc = "GAIN15/1024"]
        pub const GAIN_15: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gain14_SPEC;
    pub type Gain14 = crate::EnumBitfieldStruct<u8, Gain14_SPEC>;
    impl Gain14 {
        #[doc = "GAIN14/1024"]
        pub const GAIN_14: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamArea1_SPEC;
impl crate::sealed::RegSpec for GamArea1_SPEC {
    type DataType = u32;
}
#[doc = "Gamma %s Correction Block Area Setting Register 1"]
pub type GamArea1 = crate::RegValueT<GamArea1_SPEC>;

impl GamArea1 {
    #[doc = "Start threshold of area 3Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th03(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, GamArea1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, GamArea1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Start threshold of area 2Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th02(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, GamArea1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3ff,1,0,u16, GamArea1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Start threshold of area 1Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th01(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, GamArea1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3ff,1,0,u16, GamArea1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, GamArea1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8, GamArea1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamArea1 {
    #[inline(always)]
    fn default() -> GamArea1 {
        <crate::RegValueT<GamArea1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamArea2_SPEC;
impl crate::sealed::RegSpec for GamArea2_SPEC {
    type DataType = u32;
}
#[doc = "Gamma %s Correction Block Area Setting Register 2"]
pub type GamArea2 = crate::RegValueT<GamArea2_SPEC>;

impl GamArea2 {
    #[doc = "Start threshold of area 6Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th06(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, GamArea2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, GamArea2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Start threshold of area 5Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th05(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, GamArea2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3ff,1,0,u16, GamArea2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Start threshold of area 4Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th04(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, GamArea2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3ff,1,0,u16, GamArea2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, GamArea2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8, GamArea2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamArea2 {
    #[inline(always)]
    fn default() -> GamArea2 {
        <crate::RegValueT<GamArea2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamArea3_SPEC;
impl crate::sealed::RegSpec for GamArea3_SPEC {
    type DataType = u32;
}
#[doc = "Gamma %s Correction Block Area Setting Register 3"]
pub type GamArea3 = crate::RegValueT<GamArea3_SPEC>;

impl GamArea3 {
    #[doc = "Start threshold of area 9Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th09(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, GamArea3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, GamArea3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Start threshold of area 8Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th08(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, GamArea3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3ff,1,0,u16, GamArea3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Start threshold of area 7Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th07(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, GamArea3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3ff,1,0,u16, GamArea3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, GamArea3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8, GamArea3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamArea3 {
    #[inline(always)]
    fn default() -> GamArea3 {
        <crate::RegValueT<GamArea3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamArea4_SPEC;
impl crate::sealed::RegSpec for GamArea4_SPEC {
    type DataType = u32;
}
#[doc = "Gamma %s Correction Block Area Setting Register 4"]
pub type GamArea4 = crate::RegValueT<GamArea4_SPEC>;

impl GamArea4 {
    #[doc = "Start threshold of area 12Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th12(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, GamArea4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, GamArea4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Start threshold of area 11Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th11(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, GamArea4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3ff,1,0,u16, GamArea4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Start threshold of area 10Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th10(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, GamArea4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3ff,1,0,u16, GamArea4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, GamArea4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8, GamArea4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamArea4 {
    #[inline(always)]
    fn default() -> GamArea4 {
        <crate::RegValueT<GamArea4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GamArea5_SPEC;
impl crate::sealed::RegSpec for GamArea5_SPEC {
    type DataType = u32;
}
#[doc = "Gamma %s Correction Block Area Setting Register 5"]
pub type GamArea5 = crate::RegValueT<GamArea5_SPEC>;

impl GamArea5 {
    #[doc = "Start threshold of area 15Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th15(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, GamArea5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, GamArea5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Start threshold of area 14Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th14(
        self,
    ) -> crate::common::RegisterField<10, 0x3ff, 1, 0, u16, GamArea5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3ff,1,0,u16, GamArea5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Start threshold of area 13Unsigned 10-bit integer"]
    #[inline(always)]
    pub fn th13(
        self,
    ) -> crate::common::RegisterField<20, 0x3ff, 1, 0, u16, GamArea5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3ff,1,0,u16, GamArea5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, GamArea5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8, GamArea5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GamArea5 {
    #[inline(always)]
    fn default() -> GamArea5 {
        <crate::RegValueT<GamArea5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutVlatch_SPEC;
impl crate::sealed::RegSpec for OutVlatch_SPEC {
    type DataType = u32;
}
#[doc = "Output Control Block Register Update Control Register"]
pub type OutVlatch = crate::RegValueT<OutVlatch_SPEC>;

impl OutVlatch {
    #[doc = "Control of output control module register value reflection to internal operations.The register values to be reflected to the internal operations at the assertion of the vertical synchronization signal (VS)."]
    #[inline(always)]
    pub fn ven(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        out_vlatch::Ven,
        OutVlatch_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            out_vlatch::Ven,
            OutVlatch_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 0000000000000000000000000000000. The write value should be 0000000000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7fffffff, 1, 0, u32, OutVlatch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7fffffff,1,0,u32, OutVlatch_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for OutVlatch {
    #[inline(always)]
    fn default() -> OutVlatch {
        <crate::RegValueT<OutVlatch_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod out_vlatch {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ven_SPEC;
    pub type Ven = crate::EnumBitfieldStruct<u8, Ven_SPEC>;
    impl Ven {
        #[doc = "Enables the register values to be reflected to the internal operations at the assertion of the vertical synchronization signal (VS)."]
        pub const _1: Self = Self::new(1);
        #[doc = "Disables the register values to be reflected to the internal operations at the assertion of the vertical synchronization signal (VS)."]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutSet_SPEC;
impl crate::sealed::RegSpec for OutSet_SPEC {
    type DataType = u32;
}
#[doc = "Output Control Block Output Interface Register"]
pub type OutSet = crate::RegValueT<OutSet_SPEC>;

impl OutSet {
    #[doc = "Data delay in serial RGB format (based on OUTCLK)"]
    #[inline(always)]
    pub fn phase(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, out_set::Phase, OutSet_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,out_set::Phase, OutSet_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Invalid data position control in serial RGB format"]
    #[inline(always)]
    pub fn dirsel(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, out_set::Dirsel, OutSet_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,out_set::Dirsel, OutSet_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock frequency division control"]
    #[inline(always)]
    pub fn frqsel(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, out_set::Frqsel, OutSet_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,out_set::Frqsel, OutSet_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Output format select"]
    #[inline(always)]
    pub fn format(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, out_set::Format, OutSet_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,out_set::Format, OutSet_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pixel order control"]
    #[inline(always)]
    pub fn swapon(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, out_set::Swapon, OutSet_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,out_set::Swapon, OutSet_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bit endian change control"]
    #[inline(always)]
    pub fn endianon(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        out_set::Endianon,
        OutSet_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            out_set::Endianon,
            OutSet_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<29, 0x7, 1, 0, u8, OutSet_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x7,1,0,u8, OutSet_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for OutSet {
    #[inline(always)]
    fn default() -> OutSet {
        <crate::RegValueT<OutSet_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod out_set {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Phase_SPEC;
    pub type Phase = crate::EnumBitfieldStruct<u8, Phase_SPEC>;
    impl Phase {
        #[doc = "3 cycles"]
        pub const _11: Self = Self::new(3);
        #[doc = "2 cycles"]
        pub const _10: Self = Self::new(2);
        #[doc = "1 cycle"]
        pub const _01: Self = Self::new(1);
        #[doc = "0 cycle"]
        pub const _00: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dirsel_SPEC;
    pub type Dirsel = crate::EnumBitfieldStruct<u8, Dirsel_SPEC>;
    impl Dirsel {
        #[doc = "Invalid data is output prior to valid (RGB) data."]
        pub const _1: Self = Self::new(1);
        #[doc = "Invalid data is output following valid (RGB) data."]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Frqsel_SPEC;
    pub type Frqsel = crate::EnumBitfieldStruct<u8, Frqsel_SPEC>;
    impl Frqsel {
        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
        #[doc = "Quarter frequency (serial RGB)"]
        pub const _10: Self = Self::new(2);
        #[doc = "Setting prohibited"]
        pub const _01: Self = Self::new(1);
        #[doc = "No frequency division, parallel RGB"]
        pub const _00: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Format_SPEC;
    pub type Format = crate::EnumBitfieldStruct<u8, Format_SPEC>;
    impl Format {
        #[doc = "Serial RGB; select RGB888 as dither output format."]
        pub const _11: Self = Self::new(3);
        #[doc = "RGB565; select RGB565 as dither output format."]
        pub const _10: Self = Self::new(2);
        #[doc = "RGB666; select RGB666 as dither output format."]
        pub const _01: Self = Self::new(1);
        #[doc = "RGB888; select RGB888 as dither output format."]
        pub const _00: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Swapon_SPEC;
    pub type Swapon = crate::EnumBitfieldStruct<u8, Swapon_SPEC>;
    impl Swapon {
        #[doc = "In the order of BGR"]
        pub const _1: Self = Self::new(1);
        #[doc = "In the order of RGB"]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Endianon_SPEC;
    pub type Endianon = crate::EnumBitfieldStruct<u8, Endianon_SPEC>;
    impl Endianon {
        #[doc = "Ascending order (big endian)"]
        pub const _1: Self = Self::new(1);
        #[doc = "Descending order (little endian)"]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutBright1_SPEC;
impl crate::sealed::RegSpec for OutBright1_SPEC {
    type DataType = u32;
}
#[doc = "Output Control Block Brightness Correction Register 1"]
pub type OutBright1 = crate::RegValueT<OutBright1_SPEC>;

impl OutBright1 {
    #[doc = "Brightness (DC) adjustment of G signalUnsigned; 10 bits; +512 with offset; integer"]
    #[inline(always)]
    pub fn brtg(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, OutBright1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, OutBright1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 0000000000000000000000. The write value should be 0000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<10, 0x3fffff, 1, 0, u32, OutBright1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3fffff,1,0,u32, OutBright1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for OutBright1 {
    #[inline(always)]
    fn default() -> OutBright1 {
        <crate::RegValueT<OutBright1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutBright2_SPEC;
impl crate::sealed::RegSpec for OutBright2_SPEC {
    type DataType = u32;
}
#[doc = "Output Control Block Brightness Correction Register 2"]
pub type OutBright2 = crate::RegValueT<OutBright2_SPEC>;

impl OutBright2 {
    #[doc = "Brightness (DC) adjustment of R signalUnsigned; 10 bits; +512 with offset; integer"]
    #[inline(always)]
    pub fn brtr(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, OutBright2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, OutBright2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Brightness (DC) adjustment of B signalUnsigned; 10 bits; +512 with offset; integer"]
    #[inline(always)]
    pub fn brtb(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, OutBright2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3ff,1,0,u16, OutBright2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<26, 0x3f, 1, 0, u8, OutBright2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x3f,1,0,u8, OutBright2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for OutBright2 {
    #[inline(always)]
    fn default() -> OutBright2 {
        <crate::RegValueT<OutBright2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutContrast_SPEC;
impl crate::sealed::RegSpec for OutContrast_SPEC {
    type DataType = u32;
}
#[doc = "Output Control Block Contrast Correction Register"]
pub type OutContrast = crate::RegValueT<OutContrast_SPEC>;

impl OutContrast {
    #[doc = "Contrast (GAIN) adjustment of R signalUnsigned; 8 bits fixed point"]
    #[inline(always)]
    pub fn contr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        out_contrast::Contr,
        OutContrast_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            out_contrast::Contr,
            OutContrast_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Contrast (GAIN) adjustment of B signalUnsigned; 8 bits fixed point"]
    #[inline(always)]
    pub fn contb(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        out_contrast::Contb,
        OutContrast_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            out_contrast::Contb,
            OutContrast_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Contrast (GAIN) adjustment of G signalUnsigned; 8 bits fixed point."]
    #[inline(always)]
    pub fn contg(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        out_contrast::Contg,
        OutContrast_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            out_contrast::Contg,
            OutContrast_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 00000000. The write value should be 00000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, OutContrast_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, OutContrast_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for OutContrast {
    #[inline(always)]
    fn default() -> OutContrast {
        <crate::RegValueT<OutContrast_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod out_contrast {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Contr_SPEC;
    pub type Contr = crate::EnumBitfieldStruct<u8, Contr_SPEC>;
    impl Contr {
        #[doc = "CONTR/128"]
        pub const CONTR: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Contb_SPEC;
    pub type Contb = crate::EnumBitfieldStruct<u8, Contb_SPEC>;
    impl Contb {
        #[doc = "CONTB/128"]
        pub const CONTB: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Contg_SPEC;
    pub type Contg = crate::EnumBitfieldStruct<u8, Contg_SPEC>;
    impl Contg {
        #[doc = "CONTG/128"]
        pub const CONTG: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutPdtha_SPEC;
impl crate::sealed::RegSpec for OutPdtha_SPEC {
    type DataType = u32;
}
#[doc = "Output Control Block Panel Dither Correction Register"]
pub type OutPdtha = crate::RegValueT<OutPdtha_SPEC>;

impl OutPdtha {
    #[doc = "Pattern value (D) of 2 x 2 pattern ditherUnsigned 2-bit integer"]
    #[inline(always)]
    pub fn pd(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, OutPdtha_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, OutPdtha_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pattern value (C) of 2 x 2 pattern ditherUnsigned 2-bit integer"]
    #[inline(always)]
    pub fn pc(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, OutPdtha_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, OutPdtha_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pattern value (B) of 2 x 2 pattern ditherUnsigned 2-bit integer"]
    #[inline(always)]
    pub fn pb(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, OutPdtha_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, OutPdtha_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pattern value (A) of 2 x 2 pattern ditherUnsigned 2-bit integer"]
    #[inline(always)]
    pub fn pa(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, OutPdtha_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8, OutPdtha_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Output format select"]
    #[inline(always)]
    pub fn form(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3,
        1,
        0,
        out_pdtha::Form,
        OutPdtha_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            out_pdtha::Form,
            OutPdtha_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Operation mode"]
    #[inline(always)]
    pub fn sel(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, out_pdtha::Sel, OutPdtha_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            20,
            0x3,
            1,
            0,
            out_pdtha::Sel,
            OutPdtha_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 0000000000. The write value should be 0000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<22, 0x3ff, 1, 0, u16, OutPdtha_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0x3ff,1,0,u16, OutPdtha_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for OutPdtha {
    #[inline(always)]
    fn default() -> OutPdtha {
        <crate::RegValueT<OutPdtha_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod out_pdtha {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Form_SPEC;
    pub type Form = crate::EnumBitfieldStruct<u8, Form_SPEC>;
    impl Form {
        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
        #[doc = "RGB565; select RGB565 as output interface format."]
        pub const _10: Self = Self::new(2);
        #[doc = "RGB666; select RGB666 as output interface format."]
        pub const _01: Self = Self::new(1);
        #[doc = "RGB888; select RGB888 or serial RGB as output interface format."]
        pub const _00: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sel_SPEC;
    pub type Sel = crate::EnumBitfieldStruct<u8, Sel_SPEC>;
    impl Sel {
        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
        #[doc = "2x2 pattern dither"]
        pub const _10: Self = Self::new(2);
        #[doc = "Round-off"]
        pub const _01: Self = Self::new(1);
        #[doc = "Truncate"]
        pub const _00: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutClkphase_SPEC;
impl crate::sealed::RegSpec for OutClkphase_SPEC {
    type DataType = u32;
}
#[doc = "Output Control Block Output Phase Control Register"]
pub type OutClkphase = crate::RegValueT<OutClkphase_SPEC>;

impl OutClkphase {
    #[doc = "LCD_TCON3 Output Phase Control"]
    #[inline(always)]
    pub fn tcon3edge(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        out_clkphase::Tcon3Edge,
        OutClkphase_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            out_clkphase::Tcon3Edge,
            OutClkphase_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "LCD_TCON2 Output Phase Control"]
    #[inline(always)]
    pub fn tcon2edge(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        out_clkphase::Tcon2Edge,
        OutClkphase_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            out_clkphase::Tcon2Edge,
            OutClkphase_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "LCD_TCON1 Output Phase Control"]
    #[inline(always)]
    pub fn tcon1edge(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        out_clkphase::Tcon1Edge,
        OutClkphase_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            out_clkphase::Tcon1Edge,
            OutClkphase_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "LCD_TCON0 Output Phase Control"]
    #[inline(always)]
    pub fn tcon0edge(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        out_clkphase::Tcon0Edge,
        OutClkphase_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            out_clkphase::Tcon0Edge,
            OutClkphase_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "LCD_DATA Output Phase Control"]
    #[inline(always)]
    pub fn lcdedge(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        out_clkphase::Lcdedge,
        OutClkphase_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            out_clkphase::Lcdedge,
            OutClkphase_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Correction control"]
    #[inline(always)]
    pub fn frontgam(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        out_clkphase::Frontgam,
        OutClkphase_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            out_clkphase::Frontgam,
            OutClkphase_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, OutClkphase_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16, OutClkphase_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for OutClkphase {
    #[inline(always)]
    fn default() -> OutClkphase {
        <crate::RegValueT<OutClkphase_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod out_clkphase {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcon3Edge_SPEC;
    pub type Tcon3Edge = crate::EnumBitfieldStruct<u8, Tcon3Edge_SPEC>;
    impl Tcon3Edge {
        #[doc = "In synchronization with the falling edge of LCD_CLK."]
        pub const _1: Self = Self::new(1);
        #[doc = "In synchronization with the rising edge of LCD_CLK."]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcon2Edge_SPEC;
    pub type Tcon2Edge = crate::EnumBitfieldStruct<u8, Tcon2Edge_SPEC>;
    impl Tcon2Edge {
        #[doc = "In synchronization with the falling edge of LCD_CLK."]
        pub const _1: Self = Self::new(1);
        #[doc = "In synchronization with the rising edge of LCD_CLK."]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcon1Edge_SPEC;
    pub type Tcon1Edge = crate::EnumBitfieldStruct<u8, Tcon1Edge_SPEC>;
    impl Tcon1Edge {
        #[doc = "In synchronization with the falling edge of LCD_CLK."]
        pub const _1: Self = Self::new(1);
        #[doc = "In synchronization with the rising edge of LCD_CLK."]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcon0Edge_SPEC;
    pub type Tcon0Edge = crate::EnumBitfieldStruct<u8, Tcon0Edge_SPEC>;
    impl Tcon0Edge {
        #[doc = "In synchronization with the falling edge of LCD_CLK."]
        pub const _1: Self = Self::new(1);
        #[doc = "In synchronization with the rising edge of LCD_CLK."]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lcdedge_SPEC;
    pub type Lcdedge = crate::EnumBitfieldStruct<u8, Lcdedge_SPEC>;
    impl Lcdedge {
        #[doc = "In synchronization with the rising edge of LCD_CLK."]
        pub const _0: Self = Self::new(0);
        #[doc = "In synchronization with the falling edge of LCD_CLK"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Frontgam_SPEC;
    pub type Frontgam = crate::EnumBitfieldStruct<u8, Frontgam_SPEC>;
    impl Frontgam {
        #[doc = "Gamma correction is followed by brightness/contrast correction."]
        pub const _1: Self = Self::new(1);
        #[doc = "Brightness/contrast correction is followed by gamma correction."]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TconVlatch_SPEC;
impl crate::sealed::RegSpec for TconVlatch_SPEC {
    type DataType = u32;
}
#[doc = "TCON VLATCH Register"]
pub type TconVlatch = crate::RegValueT<TconVlatch_SPEC>;

impl TconVlatch {
    #[doc = ""]
    #[inline(always)]
    pub fn ven(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, TconVlatch_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,TconVlatch_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 0000000000000000. The write value should be 0000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, TconVlatch_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16, TconVlatch_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TconVlatch {
    #[inline(always)]
    fn default() -> TconVlatch {
        <crate::RegValueT<TconVlatch_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TconTim_SPEC;
impl crate::sealed::RegSpec for TconTim_SPEC {
    type DataType = u32;
}
#[doc = "TCON Reference Timing Setting Register"]
pub type TconTim = crate::RegValueT<TconTim_SPEC>;

impl TconTim {
    #[doc = "Horizontal synchronization signal generation reference timingSets the offset from the assertion of the internal horizontal synchronization signal in terms of pixels."]
    #[inline(always)]
    pub fn offset(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7ff,
        1,
        0,
        tcon_tim::Offset,
        TconTim_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7ff,
            1,
            0,
            tcon_tim::Offset,
            TconTim_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Vertical synchronization signal generation change timing Sets the delay from the assertion of the internal horizontal synchronization signal in terms of pixels."]
    #[inline(always)]
    pub fn half(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7ff,
        1,
        0,
        tcon_tim::Half,
        TconTim_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7ff,
            1,
            0,
            tcon_tim::Half,
            TconTim_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, TconTim_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8, TconTim_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TconTim {
    #[inline(always)]
    fn default() -> TconTim {
        <crate::RegValueT<TconTim_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tcon_tim {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Offset_SPEC;
    pub type Offset = crate::EnumBitfieldStruct<u8, Offset_SPEC>;
    impl Offset {
        #[doc = "OFFSET+1 pixels. The valid range is 0x000 to 0x3FF."]
        pub const OFFSET: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Half_SPEC;
    pub type Half = crate::EnumBitfieldStruct<u8, Half_SPEC>;
    impl Half {
        #[doc = "HALF pixels. The valid range is 0x000 to 0x3FF."]
        pub const HALF: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TconStv1_SPEC;
impl crate::sealed::RegSpec for TconStv1_SPEC {
    type DataType = u32;
}
#[doc = "TCON Vertical Timing Setting Register %s1"]
pub type TconStv1 = crate::RegValueT<TconStv1_SPEC>;

impl TconStv1 {
    #[doc = "STVx1 second change timingSets the signal assertion width."]
    #[inline(always)]
    pub fn vw(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, tcon_stv1::Vw, TconStv1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x7ff,
            1,
            0,
            tcon_stv1::Vw,
            TconStv1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "STVx1 first change timing"]
    #[inline(always)]
    pub fn vs(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7ff,
        1,
        0,
        tcon_stv1::Vs,
        TconStv1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7ff,
            1,
            0,
            tcon_stv1::Vs,
            TconStv1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, TconStv1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8, TconStv1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TconStv1 {
    #[inline(always)]
    fn default() -> TconStv1 {
        <crate::RegValueT<TconStv1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tcon_stv1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vw_SPEC;
    pub type Vw = crate::EnumBitfieldStruct<u8, Vw_SPEC>;
    impl Vw {
        #[doc = "VW pixels. The valid range is 0x000 to 0x3FF."]
        pub const VW: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vs_SPEC;
    pub type Vs = crate::EnumBitfieldStruct<u8, Vs_SPEC>;
    impl Vs {
        #[doc = "VS pixels. The valid range is 0x000 to 0x3FF."]
        pub const VS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TconStv2_SPEC;
impl crate::sealed::RegSpec for TconStv2_SPEC {
    type DataType = u32;
}
#[doc = "TCON Vertical Timing Setting Register %s2"]
pub type TconStv2 = crate::RegValueT<TconStv2_SPEC>;

impl TconStv2 {
    #[doc = "Output signal select control for VSOUT (controlled by TCON_STVA2 register)/VEOUT (controlled by the TCON_STVB2 register) pin"]
    #[inline(always)]
    pub fn sel(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, tcon_stv2::Sel, TconStv2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,tcon_stv2::Sel, TconStv2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "STVx signal polarity inversion control"]
    #[inline(always)]
    pub fn inv(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, tcon_stv2::Inv, TconStv2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,tcon_stv2::Inv, TconStv2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 000000000000000000000000000. The write value should be 000000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7ffffff, 1, 0, u32, TconStv2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x7ffffff,1,0,u32, TconStv2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TconStv2 {
    #[inline(always)]
    fn default() -> TconStv2 {
        <crate::RegValueT<TconStv2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tcon_stv2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sel_SPEC;
    pub type Sel = crate::EnumBitfieldStruct<u8, Sel_SPEC>;
    impl Sel {
        #[doc = "DE"]
        pub const _111: Self = Self::new(7);
        #[doc = "Setting prohibited"]
        pub const _110: Self = Self::new(6);
        #[doc = "Setting prohibited"]
        pub const _101: Self = Self::new(5);
        #[doc = "Setting prohibited"]
        pub const _100: Self = Self::new(4);
        #[doc = "STHB"]
        pub const _011: Self = Self::new(3);
        #[doc = "STHA"]
        pub const _010: Self = Self::new(2);
        #[doc = "STVB"]
        pub const _001: Self = Self::new(1);
        #[doc = "STVA"]
        pub const _000: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Inv_SPEC;
    pub type Inv = crate::EnumBitfieldStruct<u8, Inv_SPEC>;
    impl Inv {
        #[doc = "Inverted"]
        pub const _1: Self = Self::new(1);
        #[doc = "Not inverted"]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TconSth1_SPEC;
impl crate::sealed::RegSpec for TconSth1_SPEC {
    type DataType = u32;
}
#[doc = "TCON Horizontal Timing Setting Register STH%s1"]
pub type TconSth1 = crate::RegValueT<TconSth1_SPEC>;

impl TconSth1 {
    #[doc = "STHx1 second change timing.Sets the signal assertion width."]
    #[inline(always)]
    pub fn hw(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, tcon_sth1::Hw, TconSth1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x7ff,
            1,
            0,
            tcon_sth1::Hw,
            TconSth1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "STHx1 first change timing"]
    #[inline(always)]
    pub fn hs(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7ff,
        1,
        0,
        tcon_sth1::Hs,
        TconSth1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7ff,
            1,
            0,
            tcon_sth1::Hs,
            TconSth1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, TconSth1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8, TconSth1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TconSth1 {
    #[inline(always)]
    fn default() -> TconSth1 {
        <crate::RegValueT<TconSth1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tcon_sth1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hw_SPEC;
    pub type Hw = crate::EnumBitfieldStruct<u8, Hw_SPEC>;
    impl Hw {
        #[doc = "HW pixels. The valid range is 0x000 to 0x3FF."]
        pub const HW: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hs_SPEC;
    pub type Hs = crate::EnumBitfieldStruct<u8, Hs_SPEC>;
    impl Hs {
        #[doc = "HS lines. The valid range is 0x000 to 0x3FF."]
        pub const HS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TconSth2_SPEC;
impl crate::sealed::RegSpec for TconSth2_SPEC {
    type DataType = u32;
}
#[doc = "TCON Horizontal Timing Setting Register STH%s2"]
pub type TconSth2 = crate::RegValueT<TconSth2_SPEC>;

impl TconSth2 {
    #[doc = "Output signal select control for LCD_TCON2 (controlled by TCON_STHA2 register)/LCD_TCON3 (controlled by the TCON_STHB2 register) pin."]
    #[inline(always)]
    pub fn sel(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, tcon_sth2::Sel, TconSth2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,tcon_sth2::Sel, TconSth2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "STVx signal polarity inversion control."]
    #[inline(always)]
    pub fn inv(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, tcon_sth2::Inv, TconSth2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,tcon_sth2::Inv, TconSth2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "STHx signal generation reference timing control."]
    #[inline(always)]
    pub fn hssel(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        tcon_sth2::Hssel,
        TconSth2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            tcon_sth2::Hssel,
            TconSth2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 00000000000000000000000. The write value should be 00000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<9, 0x7fffff, 1, 0, u32, TconSth2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x7fffff,1,0,u32, TconSth2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TconSth2 {
    #[inline(always)]
    fn default() -> TconSth2 {
        <crate::RegValueT<TconSth2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tcon_sth2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sel_SPEC;
    pub type Sel = crate::EnumBitfieldStruct<u8, Sel_SPEC>;
    impl Sel {
        #[doc = "DE"]
        pub const _111: Self = Self::new(7);
        #[doc = "Setting prohibited"]
        pub const _110: Self = Self::new(6);
        #[doc = "Setting prohibited"]
        pub const _101: Self = Self::new(5);
        #[doc = "Setting prohibited"]
        pub const _100: Self = Self::new(4);
        #[doc = "STHB"]
        pub const _011: Self = Self::new(3);
        #[doc = "STHA"]
        pub const _010: Self = Self::new(2);
        #[doc = "STVB"]
        pub const _001: Self = Self::new(1);
        #[doc = "STVA"]
        pub const _000: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Inv_SPEC;
    pub type Inv = crate::EnumBitfieldStruct<u8, Inv_SPEC>;
    impl Inv {
        #[doc = "Inverted"]
        pub const _1: Self = Self::new(1);
        #[doc = "Not inverted"]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hssel_SPEC;
    pub type Hssel = crate::EnumBitfieldStruct<u8, Hssel_SPEC>;
    impl Hssel {
        #[doc = "Reference timing is the offset set with the TCON_TIM.OFFSET\\[10:0\\] (horizontal synchronization generation reference timing) field"]
        pub const _1: Self = Self::new(1);
        #[doc = "Reference timing is the input horizontal synchronization signal (HSIN)"]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TconDe_SPEC;
impl crate::sealed::RegSpec for TconDe_SPEC {
    type DataType = u32;
}
#[doc = "TCON Data Enable Polarity Setting Register"]
pub type TconDe = crate::RegValueT<TconDe_SPEC>;

impl TconDe {
    #[doc = "DE signal polarity inversion control."]
    #[inline(always)]
    pub fn inv(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, tcon_de::Inv, TconDe_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,tcon_de::Inv, TconDe_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 0000000000000000000000000000000. The write value should be 0000000000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7fffffff, 1, 0, u32, TconDe_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7fffffff,1,0,u32, TconDe_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TconDe {
    #[inline(always)]
    fn default() -> TconDe {
        <crate::RegValueT<TconDe_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tcon_de {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Inv_SPEC;
    pub type Inv = crate::EnumBitfieldStruct<u8, Inv_SPEC>;
    impl Inv {
        #[doc = "Inverted"]
        pub const _1: Self = Self::new(1);
        #[doc = "Not inverted"]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SyscntDtcten_SPEC;
impl crate::sealed::RegSpec for SyscntDtcten_SPEC {
    type DataType = u32;
}
#[doc = "System Control Block State Detection Control Register"]
pub type SyscntDtcten = crate::RegValueT<SyscntDtcten_SPEC>;

impl SyscntDtcten {
    #[doc = "Specified line detection control"]
    #[inline(always)]
    pub fn vposdtc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        syscnt_dtcten::Vposdtc,
        SyscntDtcten_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            syscnt_dtcten::Vposdtc,
            SyscntDtcten_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Graphics 1 underflow detection control"]
    #[inline(always)]
    pub fn l1undfdtc(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        syscnt_dtcten::L1Undfdtc,
        SyscntDtcten_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            syscnt_dtcten::L1Undfdtc,
            SyscntDtcten_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Graphics 2 underflow detection control"]
    #[inline(always)]
    pub fn l2undfdtc(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        syscnt_dtcten::L2Undfdtc,
        SyscntDtcten_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            syscnt_dtcten::L2Undfdtc,
            SyscntDtcten_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 00000000000000000000000000000. The write value should be 00000000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1fffffff, 1, 0, u32, SyscntDtcten_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1fffffff,1,0,u32, SyscntDtcten_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SyscntDtcten {
    #[inline(always)]
    fn default() -> SyscntDtcten {
        <crate::RegValueT<SyscntDtcten_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod syscnt_dtcten {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vposdtc_SPEC;
    pub type Vposdtc = crate::EnumBitfieldStruct<u8, Vposdtc_SPEC>;
    impl Vposdtc {
        #[doc = "Enables detection."]
        pub const _1: Self = Self::new(1);
        #[doc = "Disables detection."]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L1Undfdtc_SPEC;
    pub type L1Undfdtc = crate::EnumBitfieldStruct<u8, L1Undfdtc_SPEC>;
    impl L1Undfdtc {
        #[doc = "Enables detection."]
        pub const _1: Self = Self::new(1);
        #[doc = "Disables detection."]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L2Undfdtc_SPEC;
    pub type L2Undfdtc = crate::EnumBitfieldStruct<u8, L2Undfdtc_SPEC>;
    impl L2Undfdtc {
        #[doc = "Enables detection."]
        pub const _1: Self = Self::new(1);
        #[doc = "Disables detection."]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SyscntInten_SPEC;
impl crate::sealed::RegSpec for SyscntInten_SPEC {
    type DataType = u32;
}
#[doc = "System Control Block Interrupt Request Enable Control Register"]
pub type SyscntInten = crate::RegValueT<SyscntInten_SPEC>;

impl SyscntInten {
    #[doc = "Interrupt request signal GLCDC_VPOS enable control."]
    #[inline(always)]
    pub fn vposinten(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        syscnt_inten::Vposinten,
        SyscntInten_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            syscnt_inten::Vposinten,
            SyscntInten_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Interrupt request signal GLCDC_L1UNDF enable control."]
    #[inline(always)]
    pub fn l1undfinten(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        syscnt_inten::L1Undfinten,
        SyscntInten_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            syscnt_inten::L1Undfinten,
            SyscntInten_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Interrupt request signal GLCDC_L2UNDF enable control."]
    #[inline(always)]
    pub fn l2undfinten(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        syscnt_inten::L2Undfinten,
        SyscntInten_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            syscnt_inten::L2Undfinten,
            SyscntInten_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 00000000000000000000000000000. The write value should be 00000000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1fffffff, 1, 0, u32, SyscntInten_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1fffffff,1,0,u32, SyscntInten_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SyscntInten {
    #[inline(always)]
    fn default() -> SyscntInten {
        <crate::RegValueT<SyscntInten_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod syscnt_inten {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vposinten_SPEC;
    pub type Vposinten = crate::EnumBitfieldStruct<u8, Vposinten_SPEC>;
    impl Vposinten {
        #[doc = "Enables GLCDC_VPOS output"]
        pub const _1: Self = Self::new(1);
        #[doc = "Disables GLCDC_VPOS output"]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L1Undfinten_SPEC;
    pub type L1Undfinten = crate::EnumBitfieldStruct<u8, L1Undfinten_SPEC>;
    impl L1Undfinten {
        #[doc = "Enables GLCDC_L1UNDF output"]
        pub const _1: Self = Self::new(1);
        #[doc = "Disables GLCDC_L1UNDF output"]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L2Undfinten_SPEC;
    pub type L2Undfinten = crate::EnumBitfieldStruct<u8, L2Undfinten_SPEC>;
    impl L2Undfinten {
        #[doc = "Enables GLCDC_L2UNDF output"]
        pub const _1: Self = Self::new(1);
        #[doc = "Disables GLCDC_L2UNDF output"]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SyscntStclr_SPEC;
impl crate::sealed::RegSpec for SyscntStclr_SPEC {
    type DataType = u32;
}
#[doc = "System Control Block Status Clear Register"]
pub type SyscntStclr = crate::RegValueT<SyscntStclr_SPEC>;

impl SyscntStclr {
    #[doc = "Graphics 2 specified line detection flag clear field"]
    #[inline(always)]
    pub fn vposclr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        syscnt_stclr::Vposclr,
        SyscntStclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            syscnt_stclr::Vposclr,
            SyscntStclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Graphics 1 underflow detection flag clear field"]
    #[inline(always)]
    pub fn l1undfclr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        syscnt_stclr::L1Undfclr,
        SyscntStclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            syscnt_stclr::L1Undfclr,
            SyscntStclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Graphics 2 underflow detection flag clear field"]
    #[inline(always)]
    pub fn l2undfclr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        syscnt_stclr::L2Undfclr,
        SyscntStclr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            syscnt_stclr::L2Undfclr,
            SyscntStclr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 00000000000000000000000000000. The write value should be 00000000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1fffffff, 1, 0, u32, SyscntStclr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1fffffff,1,0,u32, SyscntStclr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SyscntStclr {
    #[inline(always)]
    fn default() -> SyscntStclr {
        <crate::RegValueT<SyscntStclr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod syscnt_stclr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vposclr_SPEC;
    pub type Vposclr = crate::EnumBitfieldStruct<u8, Vposclr_SPEC>;
    impl Vposclr {
        #[doc = "Clears the specified line detection flag."]
        pub const _1: Self = Self::new(1);
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L1Undfclr_SPEC;
    pub type L1Undfclr = crate::EnumBitfieldStruct<u8, L1Undfclr_SPEC>;
    impl L1Undfclr {
        #[doc = "Clears the graphics 1 underflow detection flag."]
        pub const _1: Self = Self::new(1);
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L2Undfclr_SPEC;
    pub type L2Undfclr = crate::EnumBitfieldStruct<u8, L2Undfclr_SPEC>;
    impl L2Undfclr {
        #[doc = "Clears the graphics 2 underflow detection flag."]
        pub const _1: Self = Self::new(1);
        #[doc = "No operation"]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SyscntStmon_SPEC;
impl crate::sealed::RegSpec for SyscntStmon_SPEC {
    type DataType = u32;
}
#[doc = "System Control Block Status Monitor Register"]
pub type SyscntStmon = crate::RegValueT<SyscntStmon_SPEC>;

impl SyscntStmon {
    #[doc = "Graphics 2 specified line detection flag"]
    #[inline(always)]
    pub fn vpos(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        syscnt_stmon::Vpos,
        SyscntStmon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            syscnt_stmon::Vpos,
            SyscntStmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Graphics 1 underflow detection flag"]
    #[inline(always)]
    pub fn l1undf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        syscnt_stmon::L1Undf,
        SyscntStmon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            syscnt_stmon::L1Undf,
            SyscntStmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Graphics 2 underflow detection flag"]
    #[inline(always)]
    pub fn l2undf(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        syscnt_stmon::L2Undf,
        SyscntStmon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            syscnt_stmon::L2Undf,
            SyscntStmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 00000000000000000000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1fffffff, 1, 0, u32, SyscntStmon_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<3,0x1fffffff,1,0,u32, SyscntStmon_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for SyscntStmon {
    #[inline(always)]
    fn default() -> SyscntStmon {
        <crate::RegValueT<SyscntStmon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod syscnt_stmon {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vpos_SPEC;
    pub type Vpos = crate::EnumBitfieldStruct<u8, Vpos_SPEC>;
    impl Vpos {
        #[doc = "A specified line notification has been detected in graphics 2."]
        pub const _1: Self = Self::new(1);
        #[doc = "No specified line notification has been detected in graphics 2."]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L1Undf_SPEC;
    pub type L1Undf = crate::EnumBitfieldStruct<u8, L1Undf_SPEC>;
    impl L1Undf {
        #[doc = "An underflow has been detected in graphics 1."]
        pub const _1: Self = Self::new(1);
        #[doc = "No underflow has been detected in graphics 1."]
        pub const _0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct L2Undf_SPEC;
    pub type L2Undf = crate::EnumBitfieldStruct<u8, L2Undf_SPEC>;
    impl L2Undf {
        #[doc = "An underflow has been detected in graphics 2."]
        pub const _1: Self = Self::new(1);
        #[doc = "No underflow has been detected in graphics 2."]
        pub const _0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SyscntPanelClk_SPEC;
impl crate::sealed::RegSpec for SyscntPanelClk_SPEC {
    type DataType = u32;
}
#[doc = "System Control Block Version and Panel Clock Control Register"]
pub type SyscntPanelClk = crate::RegValueT<SyscntPanelClk_SPEC>;

impl SyscntPanelClk {
    #[doc = "Clock division ratio setting controlRefer toTable 2.7.1 for details about setting value.Note: Settings that are not listed in table 2.7.1 are prohibited."]
    #[inline(always)]
    pub fn dcdr(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, SyscntPanelClk_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8, SyscntPanelClk_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Panel clock output enable controlNote: Before changing the PIXSEL,CLKSEL or DCDR bit, this bit must be set to 0."]
    #[inline(always)]
    pub fn clken(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        syscnt_panel_clk::Clken,
        SyscntPanelClk_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            syscnt_panel_clk::Clken,
            SyscntPanelClk_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Panel clock supply source select"]
    #[inline(always)]
    pub fn clksel(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        syscnt_panel_clk::Clksel,
        SyscntPanelClk_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            syscnt_panel_clk::Clksel,
            SyscntPanelClk_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Pixel clock select control.Must be set to the same value as OUT_SET.FRQSEL\\[1\\]."]
    #[inline(always)]
    pub fn pixsel(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        syscnt_panel_clk::Pixsel,
        SyscntPanelClk_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            syscnt_panel_clk::Pixsel,
            SyscntPanelClk_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<13, 0x7, 1, 0, u8, SyscntPanelClk_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x7,1,0,u8, SyscntPanelClk_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Version informationVersion information of the GLCD"]
    #[inline(always)]
    pub fn ver(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, SyscntPanelClk_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16, SyscntPanelClk_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for SyscntPanelClk {
    #[inline(always)]
    fn default() -> SyscntPanelClk {
        <crate::RegValueT<SyscntPanelClk_SPEC> as RegisterValue<_>>::new(17825792)
    }
}
pub mod syscnt_panel_clk {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clken_SPEC;
    pub type Clken = crate::EnumBitfieldStruct<u8, Clken_SPEC>;
    impl Clken {
        #[doc = "Disable panel clock output"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable panel clock output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clksel_SPEC;
    pub type Clksel = crate::EnumBitfieldStruct<u8, Clksel_SPEC>;
    impl Clksel {
        #[doc = "External clock select"]
        pub const _0: Self = Self::new(0);
        #[doc = "PLL output select"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pixsel_SPEC;
    pub type Pixsel = crate::EnumBitfieldStruct<u8, Pixsel_SPEC>;
    impl Pixsel {
        #[doc = "No frequency division, parallel RGB"]
        pub const _0: Self = Self::new(0);
        #[doc = "Quarter frequency,serial RGB"]
        pub const _1: Self = Self::new(1);
    }
}
