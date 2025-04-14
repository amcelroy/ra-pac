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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:21:54 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Capture Engine Unit"]
unsafe impl ::core::marker::Send for super::Ceu {}
unsafe impl ::core::marker::Sync for super::Ceu {}
impl super::Ceu {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn capsr(&self) -> &'static crate::common::Reg<self::Capsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Capsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn capcr(&self) -> &'static crate::common::Reg<self::Capcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Capcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn camcr(&self) -> &'static crate::common::Reg<self::Camcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Camcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cmcyr(&self) -> &'static crate::common::Reg<self::Cmcyr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cmcyr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn camor(&self) -> &'static crate::common::Reg<self::Camor_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Camor_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn capwr(&self) -> &'static crate::common::Reg<self::Capwr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Capwr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn caifr(&self) -> &'static crate::common::Reg<self::Caifr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Caifr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn crcntr(
        &self,
    ) -> &'static crate::common::Reg<self::Crcntr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Crcntr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn crcmpr(
        &self,
    ) -> &'static crate::common::Reg<self::Crcmpr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Crcmpr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cflcr(&self) -> &'static crate::common::Reg<self::Cflcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cflcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfszr(&self) -> &'static crate::common::Reg<self::Cfszr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfszr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cdwdr(&self) -> &'static crate::common::Reg<self::Cdwdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdwdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cdayr(&self) -> &'static crate::common::Reg<self::Cdayr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdayr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cdacr(&self) -> &'static crate::common::Reg<self::Cdacr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdacr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cdbyr(&self) -> &'static crate::common::Reg<self::Cdbyr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdbyr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cdbcr(&self) -> &'static crate::common::Reg<self::Cdbcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdbcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cbdsr(&self) -> &'static crate::common::Reg<self::Cbdsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cbdsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfwcr(&self) -> &'static crate::common::Reg<self::Cfwcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cfwcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[inline(always)]
    pub const fn clfcr(&self) -> &'static crate::common::Reg<self::Clfcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Clfcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cdocr(&self) -> &'static crate::common::Reg<self::Cdocr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdocr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ceier(&self) -> &'static crate::common::Reg<self::Ceier_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ceier_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cetcr(&self) -> &'static crate::common::Reg<self::Cetcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cetcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cstsr(&self) -> &'static crate::common::Reg<self::Cstsr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cstsr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cdssr(&self) -> &'static crate::common::Reg<self::Cdssr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cdssr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cdayr2(
        &self,
    ) -> &'static crate::common::Reg<self::Cdayr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdayr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cdacr2(
        &self,
    ) -> &'static crate::common::Reg<self::Cdacr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdacr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cdbyr2(
        &self,
    ) -> &'static crate::common::Reg<self::Cdbyr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdbyr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(152usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cdbcr2(
        &self,
    ) -> &'static crate::common::Reg<self::Cdbcr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdbcr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(156usize),
            )
        }
    }

    #[inline(always)]
    pub const fn axibusctl2(
        &self,
    ) -> &'static crate::common::Reg<self::Axibusctl2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Axibusctl2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(160usize),
            )
        }
    }

    #[inline(always)]
    pub const fn camor_b(
        &self,
    ) -> &'static crate::common::Reg<self::CamorB_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CamorB_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4112usize),
            )
        }
    }

    #[inline(always)]
    pub const fn capwr_b(
        &self,
    ) -> &'static crate::common::Reg<self::CapwrB_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CapwrB_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4116usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cflcr_b(
        &self,
    ) -> &'static crate::common::Reg<self::CflcrB_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CflcrB_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4144usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfszr_b(
        &self,
    ) -> &'static crate::common::Reg<self::CfszrB_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CfszrB_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4148usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cdwdr_b(
        &self,
    ) -> &'static crate::common::Reg<self::CdwdrB_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CdwdrB_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4152usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cdayr_b(
        &self,
    ) -> &'static crate::common::Reg<self::CdayrB_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CdayrB_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4156usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cdacr_b(
        &self,
    ) -> &'static crate::common::Reg<self::CdacrB_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CdacrB_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4160usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cdbyr_b(
        &self,
    ) -> &'static crate::common::Reg<self::CdbyrB_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CdbyrB_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4164usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cdbcr_b(
        &self,
    ) -> &'static crate::common::Reg<self::CdbcrB_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CdbcrB_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4168usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cbdsr_b(
        &self,
    ) -> &'static crate::common::Reg<self::CbdsrB_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CbdsrB_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4172usize),
            )
        }
    }

    #[inline(always)]
    pub const fn clfcr_b(
        &self,
    ) -> &'static crate::common::Reg<self::ClfcrB_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClfcrB_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4192usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cdocr_b(
        &self,
    ) -> &'static crate::common::Reg<self::CdocrB_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CdocrB_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4196usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cdayr2_b(
        &self,
    ) -> &'static crate::common::Reg<self::Cdayr2B_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdayr2B_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4240usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cdacr2_b(
        &self,
    ) -> &'static crate::common::Reg<self::Cdacr2B_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdacr2B_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4244usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cdbyr2_b(
        &self,
    ) -> &'static crate::common::Reg<self::Cdbyr2B_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdbyr2B_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4248usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cdbcr2_b(
        &self,
    ) -> &'static crate::common::Reg<self::Cdbcr2B_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdbcr2B_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4252usize),
            )
        }
    }

    #[inline(always)]
    pub const fn camor_m(
        &self,
    ) -> &'static crate::common::Reg<self::CamorM_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CamorM_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8208usize),
            )
        }
    }

    #[inline(always)]
    pub const fn capwr_m(
        &self,
    ) -> &'static crate::common::Reg<self::CapwrM_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CapwrM_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8212usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cflcr_m(
        &self,
    ) -> &'static crate::common::Reg<self::CflcrM_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CflcrM_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8240usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfszr_m(
        &self,
    ) -> &'static crate::common::Reg<self::CfszrM_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CfszrM_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8244usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cdwdr_m(
        &self,
    ) -> &'static crate::common::Reg<self::CdwdrM_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CdwdrM_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8248usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cdayr_m(
        &self,
    ) -> &'static crate::common::Reg<self::CdayrM_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CdayrM_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8252usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cdacr_m(
        &self,
    ) -> &'static crate::common::Reg<self::CdacrM_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CdacrM_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8256usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cdbyr_m(
        &self,
    ) -> &'static crate::common::Reg<self::CdbyrM_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CdbyrM_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8260usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cdbcr_m(
        &self,
    ) -> &'static crate::common::Reg<self::CdbcrM_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CdbcrM_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8264usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cbdsr_m(
        &self,
    ) -> &'static crate::common::Reg<self::CbdsrM_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CbdsrM_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8268usize),
            )
        }
    }

    #[inline(always)]
    pub const fn clfcr_m(
        &self,
    ) -> &'static crate::common::Reg<self::ClfcrM_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClfcrM_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8288usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cdocr_m(
        &self,
    ) -> &'static crate::common::Reg<self::CdocrM_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CdocrM_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8292usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cdayr2_m(
        &self,
    ) -> &'static crate::common::Reg<self::Cdayr2M_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdayr2M_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8336usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cdacr2_m(
        &self,
    ) -> &'static crate::common::Reg<self::Cdacr2M_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdacr2M_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8340usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cdbyr2_m(
        &self,
    ) -> &'static crate::common::Reg<self::Cdbyr2M_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdbyr2M_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8344usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cdbcr2_m(
        &self,
    ) -> &'static crate::common::Reg<self::Cdbcr2M_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cdbcr2M_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8348usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capsr_SPEC;
impl crate::sealed::RegSpec for Capsr_SPEC {
    type DataType = u32;
}

pub type Capsr = crate::RegValueT<Capsr_SPEC>;

impl Capsr {
    #[inline(always)]
    pub fn ce(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        capsr::Ce,
        capsr::Ce,
        Capsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            capsr::Ce,
            capsr::Ce,
            Capsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cpkil(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        capsr::Cpkil,
        capsr::Cpkil,
        Capsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            capsr::Cpkil,
            capsr::Cpkil,
            Capsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Capsr {
    #[inline(always)]
    fn default() -> Capsr {
        <crate::RegValueT<Capsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod capsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ce_SPEC;
    pub type Ce = crate::EnumBitfieldStruct<u8, Ce_SPEC>;
    impl Ce {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpkil_SPEC;
    pub type Cpkil = crate::EnumBitfieldStruct<u8, Cpkil_SPEC>;
    impl Cpkil {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capcr_SPEC;
impl crate::sealed::RegSpec for Capcr_SPEC {
    type DataType = u32;
}

pub type Capcr = crate::RegValueT<Capcr_SPEC>;

impl Capcr {
    #[inline(always)]
    pub fn ctncp(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        capcr::Ctncp,
        capcr::Ctncp,
        Capcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            capcr::Ctncp,
            capcr::Ctncp,
            Capcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mtcm(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x3,
        1,
        0,
        capcr::Mtcm,
        capcr::Mtcm,
        Capcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x3,
            1,
            0,
            capcr::Mtcm,
            capcr::Mtcm,
            Capcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fdrp(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Capcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Capcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Capcr {
    #[inline(always)]
    fn default() -> Capcr {
        <crate::RegValueT<Capcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod capcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctncp_SPEC;
    pub type Ctncp = crate::EnumBitfieldStruct<u8, Ctncp_SPEC>;
    impl Ctncp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mtcm_SPEC;
    pub type Mtcm = crate::EnumBitfieldStruct<u8, Mtcm_SPEC>;
    impl Mtcm {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Camcr_SPEC;
impl crate::sealed::RegSpec for Camcr_SPEC {
    type DataType = u32;
}

pub type Camcr = crate::RegValueT<Camcr_SPEC>;

impl Camcr {
    #[inline(always)]
    pub fn hdpol(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        camcr::Hdpol,
        camcr::Hdpol,
        Camcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            camcr::Hdpol,
            camcr::Hdpol,
            Camcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn vdpol(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        camcr::Vdpol,
        camcr::Vdpol,
        Camcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            camcr::Vdpol,
            camcr::Vdpol,
            Camcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn jpg(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        camcr::Jpg,
        camcr::Jpg,
        Camcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            camcr::Jpg,
            camcr::Jpg,
            Camcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dtary(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        camcr::Dtary,
        camcr::Dtary,
        Camcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            camcr::Dtary,
            camcr::Dtary,
            Camcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dtif(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        camcr::Dtif,
        camcr::Dtif,
        Camcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            camcr::Dtif,
            camcr::Dtif,
            Camcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fldpol(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        camcr::Fldpol,
        camcr::Fldpol,
        Camcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            camcr::Fldpol,
            camcr::Fldpol,
            Camcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dsel(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        camcr::Dsel,
        camcr::Dsel,
        Camcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            camcr::Dsel,
            camcr::Dsel,
            Camcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fldsel(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        camcr::Fldsel,
        camcr::Fldsel,
        Camcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            camcr::Fldsel,
            camcr::Fldsel,
            Camcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn hdsel(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        camcr::Hdsel,
        camcr::Hdsel,
        Camcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            camcr::Hdsel,
            camcr::Hdsel,
            Camcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn vdsel(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        camcr::Vdsel,
        camcr::Vdsel,
        Camcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            camcr::Vdsel,
            camcr::Vdsel,
            Camcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Camcr {
    #[inline(always)]
    fn default() -> Camcr {
        <crate::RegValueT<Camcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod camcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hdpol_SPEC;
    pub type Hdpol = crate::EnumBitfieldStruct<u8, Hdpol_SPEC>;
    impl Hdpol {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vdpol_SPEC;
    pub type Vdpol = crate::EnumBitfieldStruct<u8, Vdpol_SPEC>;
    impl Vdpol {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Jpg_SPEC;
    pub type Jpg = crate::EnumBitfieldStruct<u8, Jpg_SPEC>;
    impl Jpg {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtary_SPEC;
    pub type Dtary = crate::EnumBitfieldStruct<u8, Dtary_SPEC>;
    impl Dtary {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtif_SPEC;
    pub type Dtif = crate::EnumBitfieldStruct<u8, Dtif_SPEC>;
    impl Dtif {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fldpol_SPEC;
    pub type Fldpol = crate::EnumBitfieldStruct<u8, Fldpol_SPEC>;
    impl Fldpol {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dsel_SPEC;
    pub type Dsel = crate::EnumBitfieldStruct<u8, Dsel_SPEC>;
    impl Dsel {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fldsel_SPEC;
    pub type Fldsel = crate::EnumBitfieldStruct<u8, Fldsel_SPEC>;
    impl Fldsel {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hdsel_SPEC;
    pub type Hdsel = crate::EnumBitfieldStruct<u8, Hdsel_SPEC>;
    impl Hdsel {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vdsel_SPEC;
    pub type Vdsel = crate::EnumBitfieldStruct<u8, Vdsel_SPEC>;
    impl Vdsel {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmcyr_SPEC;
impl crate::sealed::RegSpec for Cmcyr_SPEC {
    type DataType = u32;
}

pub type Cmcyr = crate::RegValueT<Cmcyr_SPEC>;

impl Cmcyr {
    #[inline(always)]
    pub fn hcyl(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, u16, Cmcyr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3fff,1,0,u16,u16,Cmcyr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vcyl(
        self,
    ) -> crate::common::RegisterField<16, 0x3fff, 1, 0, u16, u16, Cmcyr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3fff,1,0,u16,u16,Cmcyr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cmcyr {
    #[inline(always)]
    fn default() -> Cmcyr {
        <crate::RegValueT<Cmcyr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Camor_SPEC;
impl crate::sealed::RegSpec for Camor_SPEC {
    type DataType = u32;
}

pub type Camor = crate::RegValueT<Camor_SPEC>;

impl Camor {
    #[inline(always)]
    pub fn hofst(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Camor_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16,u16,Camor_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vofst(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, u16, Camor_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xfff,1,0,u16,u16,Camor_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Camor {
    #[inline(always)]
    fn default() -> Camor {
        <crate::RegValueT<Camor_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capwr_SPEC;
impl crate::sealed::RegSpec for Capwr_SPEC {
    type DataType = u32;
}

pub type Capwr = crate::RegValueT<Capwr_SPEC>;

impl Capwr {
    #[inline(always)]
    pub fn hwdth(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Capwr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16,u16,Capwr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vwdth(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, u16, Capwr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xfff,1,0,u16,u16,Capwr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Capwr {
    #[inline(always)]
    fn default() -> Capwr {
        <crate::RegValueT<Capwr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Caifr_SPEC;
impl crate::sealed::RegSpec for Caifr_SPEC {
    type DataType = u32;
}

pub type Caifr = crate::RegValueT<Caifr_SPEC>;

impl Caifr {
    #[inline(always)]
    pub fn fci(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        caifr::Fci,
        caifr::Fci,
        Caifr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            caifr::Fci,
            caifr::Fci,
            Caifr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cim(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        caifr::Cim,
        caifr::Cim,
        Caifr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            caifr::Cim,
            caifr::Cim,
            Caifr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ifs(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        caifr::Ifs,
        caifr::Ifs,
        Caifr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            caifr::Ifs,
            caifr::Ifs,
            Caifr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Caifr {
    #[inline(always)]
    fn default() -> Caifr {
        <crate::RegValueT<Caifr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod caifr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fci_SPEC;
    pub type Fci = crate::EnumBitfieldStruct<u8, Fci_SPEC>;
    impl Fci {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cim_SPEC;
    pub type Cim = crate::EnumBitfieldStruct<u8, Cim_SPEC>;
    impl Cim {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ifs_SPEC;
    pub type Ifs = crate::EnumBitfieldStruct<u8, Ifs_SPEC>;
    impl Ifs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crcntr_SPEC;
impl crate::sealed::RegSpec for Crcntr_SPEC {
    type DataType = u32;
}

pub type Crcntr = crate::RegValueT<Crcntr_SPEC>;

impl Crcntr {
    #[inline(always)]
    pub fn rc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        crcntr::Rc,
        crcntr::Rc,
        Crcntr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            crcntr::Rc,
            crcntr::Rc,
            Crcntr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rs(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        crcntr::Rs,
        crcntr::Rs,
        Crcntr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            crcntr::Rs,
            crcntr::Rs,
            Crcntr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rvs(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        crcntr::Rvs,
        crcntr::Rvs,
        Crcntr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            crcntr::Rvs,
            crcntr::Rvs,
            Crcntr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Crcntr {
    #[inline(always)]
    fn default() -> Crcntr {
        <crate::RegValueT<Crcntr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod crcntr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rc_SPEC;
    pub type Rc = crate::EnumBitfieldStruct<u8, Rc_SPEC>;
    impl Rc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rs_SPEC;
    pub type Rs = crate::EnumBitfieldStruct<u8, Rs_SPEC>;
    impl Rs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rvs_SPEC;
    pub type Rvs = crate::EnumBitfieldStruct<u8, Rvs_SPEC>;
    impl Rvs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crcmpr_SPEC;
impl crate::sealed::RegSpec for Crcmpr_SPEC {
    type DataType = u32;
}

pub type Crcmpr = crate::RegValueT<Crcmpr_SPEC>;

impl Crcmpr {
    #[inline(always)]
    pub fn ra(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        crcmpr::Ra,
        crcmpr::Ra,
        Crcmpr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            crcmpr::Ra,
            crcmpr::Ra,
            Crcmpr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Crcmpr {
    #[inline(always)]
    fn default() -> Crcmpr {
        <crate::RegValueT<Crcmpr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod crcmpr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ra_SPEC;
    pub type Ra = crate::EnumBitfieldStruct<u8, Ra_SPEC>;
    impl Ra {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cflcr_SPEC;
impl crate::sealed::RegSpec for Cflcr_SPEC {
    type DataType = u32;
}

pub type Cflcr = crate::RegValueT<Cflcr_SPEC>;

impl Cflcr {
    #[inline(always)]
    pub fn hfrac(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Cflcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Cflcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn hmant(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, Cflcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,Cflcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vfrac(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, u16, Cflcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xfff,1,0,u16,u16,Cflcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vmant(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, u8, Cflcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0xf,1,0,u8,u8,Cflcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cflcr {
    #[inline(always)]
    fn default() -> Cflcr {
        <crate::RegValueT<Cflcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfszr_SPEC;
impl crate::sealed::RegSpec for Cfszr_SPEC {
    type DataType = u32;
}

pub type Cfszr = crate::RegValueT<Cfszr_SPEC>;

impl Cfszr {
    #[inline(always)]
    pub fn hfclp(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Cfszr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Cfszr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vfclp(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, u16, Cfszr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xfff,1,0,u16,u16,Cfszr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfszr {
    #[inline(always)]
    fn default() -> Cfszr {
        <crate::RegValueT<Cfszr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdwdr_SPEC;
impl crate::sealed::RegSpec for Cdwdr_SPEC {
    type DataType = u32;
}

pub type Cdwdr = crate::RegValueT<Cdwdr_SPEC>;

impl Cdwdr {
    #[inline(always)]
    pub fn chdw(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, Cdwdr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16,u16,Cdwdr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cdwdr {
    #[inline(always)]
    fn default() -> Cdwdr {
        <crate::RegValueT<Cdwdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdayr_SPEC;
impl crate::sealed::RegSpec for Cdayr_SPEC {
    type DataType = u32;
}

pub type Cdayr = crate::RegValueT<Cdayr_SPEC>;

impl Cdayr {
    #[inline(always)]
    pub fn cayr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Cdayr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Cdayr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cdayr {
    #[inline(always)]
    fn default() -> Cdayr {
        <crate::RegValueT<Cdayr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdacr_SPEC;
impl crate::sealed::RegSpec for Cdacr_SPEC {
    type DataType = u32;
}

pub type Cdacr = crate::RegValueT<Cdacr_SPEC>;

impl Cdacr {
    #[inline(always)]
    pub fn cacr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Cdacr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Cdacr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cdacr {
    #[inline(always)]
    fn default() -> Cdacr {
        <crate::RegValueT<Cdacr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdbyr_SPEC;
impl crate::sealed::RegSpec for Cdbyr_SPEC {
    type DataType = u32;
}

pub type Cdbyr = crate::RegValueT<Cdbyr_SPEC>;

impl Cdbyr {
    #[inline(always)]
    pub fn cbyr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Cdbyr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Cdbyr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cdbyr {
    #[inline(always)]
    fn default() -> Cdbyr {
        <crate::RegValueT<Cdbyr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdbcr_SPEC;
impl crate::sealed::RegSpec for Cdbcr_SPEC {
    type DataType = u32;
}

pub type Cdbcr = crate::RegValueT<Cdbcr_SPEC>;

impl Cdbcr {
    #[inline(always)]
    pub fn cbcr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Cdbcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Cdbcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cdbcr {
    #[inline(always)]
    fn default() -> Cdbcr {
        <crate::RegValueT<Cdbcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cbdsr_SPEC;
impl crate::sealed::RegSpec for Cbdsr_SPEC {
    type DataType = u32;
}

pub type Cbdsr = crate::RegValueT<Cbdsr_SPEC>;

impl Cbdsr {
    #[inline(always)]
    pub fn cbvs(
        self,
    ) -> crate::common::RegisterField<0, 0x7fffff, 1, 0, u32, u32, Cbdsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7fffff,1,0,u32,u32,Cbdsr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cbdsr {
    #[inline(always)]
    fn default() -> Cbdsr {
        <crate::RegValueT<Cbdsr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfwcr_SPEC;
impl crate::sealed::RegSpec for Cfwcr_SPEC {
    type DataType = u32;
}

pub type Cfwcr = crate::RegValueT<Cfwcr_SPEC>;

impl Cfwcr {
    #[inline(always)]
    pub fn fwe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cfwcr::Fwe,
        cfwcr::Fwe,
        Cfwcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cfwcr::Fwe,
            cfwcr::Fwe,
            Cfwcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fwv(
        self,
    ) -> crate::common::RegisterField<5, 0x7ffffff, 1, 0, u32, u32, Cfwcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x7ffffff,1,0,u32,u32,Cfwcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfwcr {
    #[inline(always)]
    fn default() -> Cfwcr {
        <crate::RegValueT<Cfwcr_SPEC> as RegisterValue<_>>::new(256)
    }
}
pub mod cfwcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fwe_SPEC;
    pub type Fwe = crate::EnumBitfieldStruct<u8, Fwe_SPEC>;
    impl Fwe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clfcr_SPEC;
impl crate::sealed::RegSpec for Clfcr_SPEC {
    type DataType = u32;
}

pub type Clfcr = crate::RegValueT<Clfcr_SPEC>;

impl Clfcr {
    #[inline(always)]
    pub fn lpf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        clfcr::Lpf,
        clfcr::Lpf,
        Clfcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            clfcr::Lpf,
            clfcr::Lpf,
            Clfcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Clfcr {
    #[inline(always)]
    fn default() -> Clfcr {
        <crate::RegValueT<Clfcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod clfcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lpf_SPEC;
    pub type Lpf = crate::EnumBitfieldStruct<u8, Lpf_SPEC>;
    impl Lpf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdocr_SPEC;
impl crate::sealed::RegSpec for Cdocr_SPEC {
    type DataType = u32;
}

pub type Cdocr = crate::RegValueT<Cdocr_SPEC>;

impl Cdocr {
    #[inline(always)]
    pub fn cobs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cdocr::Cobs,
        cdocr::Cobs,
        Cdocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cdocr::Cobs,
            cdocr::Cobs,
            Cdocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cows(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cdocr::Cows,
        cdocr::Cows,
        Cdocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cdocr::Cows,
            cdocr::Cows,
            Cdocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cols(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cdocr::Cols,
        cdocr::Cols,
        Cdocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cdocr::Cols,
            cdocr::Cols,
            Cdocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cds(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        cdocr::Cds,
        cdocr::Cds,
        Cdocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            cdocr::Cds,
            cdocr::Cds,
            Cdocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cbe(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        cdocr::Cbe,
        cdocr::Cbe,
        Cdocr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            cdocr::Cbe,
            cdocr::Cbe,
            Cdocr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cdocr {
    #[inline(always)]
    fn default() -> Cdocr {
        <crate::RegValueT<Cdocr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cdocr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cobs_SPEC;
    pub type Cobs = crate::EnumBitfieldStruct<u8, Cobs_SPEC>;
    impl Cobs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cows_SPEC;
    pub type Cows = crate::EnumBitfieldStruct<u8, Cows_SPEC>;
    impl Cows {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cols_SPEC;
    pub type Cols = crate::EnumBitfieldStruct<u8, Cols_SPEC>;
    impl Cols {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cds_SPEC;
    pub type Cds = crate::EnumBitfieldStruct<u8, Cds_SPEC>;
    impl Cds {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cbe_SPEC;
    pub type Cbe = crate::EnumBitfieldStruct<u8, Cbe_SPEC>;
    impl Cbe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ceier_SPEC;
impl crate::sealed::RegSpec for Ceier_SPEC {
    type DataType = u32;
}

pub type Ceier = crate::RegValueT<Ceier_SPEC>;

impl Ceier {
    #[inline(always)]
    pub fn cpeie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ceier::Cpeie,
        ceier::Cpeie,
        Ceier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ceier::Cpeie,
            ceier::Cpeie,
            Ceier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cfeie(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ceier::Cfeie,
        ceier::Cfeie,
        Ceier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ceier::Cfeie,
            ceier::Cfeie,
            Ceier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn igrwie(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ceier::Igrwie,
        ceier::Igrwie,
        Ceier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ceier::Igrwie,
            ceier::Igrwie,
            Ceier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn hdie(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        ceier::Hdie,
        ceier::Hdie,
        Ceier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            ceier::Hdie,
            ceier::Hdie,
            Ceier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn vdie(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ceier::Vdie,
        ceier::Vdie,
        Ceier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ceier::Vdie,
            ceier::Vdie,
            Ceier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cpbe1ie(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        ceier::Cpbe1Ie,
        ceier::Cpbe1Ie,
        Ceier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            ceier::Cpbe1Ie,
            ceier::Cpbe1Ie,
            Ceier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cpbe2ie(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        ceier::Cpbe2Ie,
        ceier::Cpbe2Ie,
        Ceier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            ceier::Cpbe2Ie,
            ceier::Cpbe2Ie,
            Ceier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cpbe3ie(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        ceier::Cpbe3Ie,
        ceier::Cpbe3Ie,
        Ceier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            ceier::Cpbe3Ie,
            ceier::Cpbe3Ie,
            Ceier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cpbe4ie(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        ceier::Cpbe4Ie,
        ceier::Cpbe4Ie,
        Ceier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            ceier::Cpbe4Ie,
            ceier::Cpbe4Ie,
            Ceier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cdtofie(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ceier::Cdtofie,
        ceier::Cdtofie,
        Ceier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ceier::Cdtofie,
            ceier::Cdtofie,
            Ceier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ighsie(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        ceier::Ighsie,
        ceier::Ighsie,
        Ceier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            ceier::Ighsie,
            ceier::Ighsie,
            Ceier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn igvsie(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        ceier::Igvsie,
        ceier::Igvsie,
        Ceier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            ceier::Igvsie,
            ceier::Igvsie,
            Ceier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn vbpie(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        ceier::Vbpie,
        ceier::Vbpie,
        Ceier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            ceier::Vbpie,
            ceier::Vbpie,
            Ceier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fwfie(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        ceier::Fwfie,
        ceier::Fwfie,
        Ceier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            ceier::Fwfie,
            ceier::Fwfie,
            Ceier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn nhdie(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        ceier::Nhdie,
        ceier::Nhdie,
        Ceier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            ceier::Nhdie,
            ceier::Nhdie,
            Ceier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn nvdie(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        ceier::Nvdie,
        ceier::Nvdie,
        Ceier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            ceier::Nvdie,
            ceier::Nvdie,
            Ceier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ceier {
    #[inline(always)]
    fn default() -> Ceier {
        <crate::RegValueT<Ceier_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ceier {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpeie_SPEC;
    pub type Cpeie = crate::EnumBitfieldStruct<u8, Cpeie_SPEC>;
    impl Cpeie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cfeie_SPEC;
    pub type Cfeie = crate::EnumBitfieldStruct<u8, Cfeie_SPEC>;
    impl Cfeie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Igrwie_SPEC;
    pub type Igrwie = crate::EnumBitfieldStruct<u8, Igrwie_SPEC>;
    impl Igrwie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hdie_SPEC;
    pub type Hdie = crate::EnumBitfieldStruct<u8, Hdie_SPEC>;
    impl Hdie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vdie_SPEC;
    pub type Vdie = crate::EnumBitfieldStruct<u8, Vdie_SPEC>;
    impl Vdie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpbe1Ie_SPEC;
    pub type Cpbe1Ie = crate::EnumBitfieldStruct<u8, Cpbe1Ie_SPEC>;
    impl Cpbe1Ie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpbe2Ie_SPEC;
    pub type Cpbe2Ie = crate::EnumBitfieldStruct<u8, Cpbe2Ie_SPEC>;
    impl Cpbe2Ie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpbe3Ie_SPEC;
    pub type Cpbe3Ie = crate::EnumBitfieldStruct<u8, Cpbe3Ie_SPEC>;
    impl Cpbe3Ie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpbe4Ie_SPEC;
    pub type Cpbe4Ie = crate::EnumBitfieldStruct<u8, Cpbe4Ie_SPEC>;
    impl Cpbe4Ie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cdtofie_SPEC;
    pub type Cdtofie = crate::EnumBitfieldStruct<u8, Cdtofie_SPEC>;
    impl Cdtofie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ighsie_SPEC;
    pub type Ighsie = crate::EnumBitfieldStruct<u8, Ighsie_SPEC>;
    impl Ighsie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Igvsie_SPEC;
    pub type Igvsie = crate::EnumBitfieldStruct<u8, Igvsie_SPEC>;
    impl Igvsie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbpie_SPEC;
    pub type Vbpie = crate::EnumBitfieldStruct<u8, Vbpie_SPEC>;
    impl Vbpie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fwfie_SPEC;
    pub type Fwfie = crate::EnumBitfieldStruct<u8, Fwfie_SPEC>;
    impl Fwfie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nhdie_SPEC;
    pub type Nhdie = crate::EnumBitfieldStruct<u8, Nhdie_SPEC>;
    impl Nhdie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nvdie_SPEC;
    pub type Nvdie = crate::EnumBitfieldStruct<u8, Nvdie_SPEC>;
    impl Nvdie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cetcr_SPEC;
impl crate::sealed::RegSpec for Cetcr_SPEC {
    type DataType = u32;
}

pub type Cetcr = crate::RegValueT<Cetcr_SPEC>;

impl Cetcr {
    #[inline(always)]
    pub fn cpe(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cetcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cetcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn cfe(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cetcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cetcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn igrw(self) -> crate::common::RegisterFieldBool<4, 1, 0, Cetcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cetcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn hd(self) -> crate::common::RegisterFieldBool<8, 1, 0, Cetcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cetcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn vd(self) -> crate::common::RegisterFieldBool<9, 1, 0, Cetcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Cetcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn cpbe1(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Cetcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Cetcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn cpbe2(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Cetcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Cetcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn cpbe3(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Cetcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Cetcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn cpbe4(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Cetcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Cetcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn cdtof(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Cetcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Cetcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn ighs(self) -> crate::common::RegisterFieldBool<17, 1, 0, Cetcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Cetcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn igvs(self) -> crate::common::RegisterFieldBool<18, 1, 0, Cetcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Cetcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn vbp(self) -> crate::common::RegisterFieldBool<20, 1, 0, Cetcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Cetcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn fwf(self) -> crate::common::RegisterFieldBool<23, 1, 0, Cetcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Cetcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn nhd(self) -> crate::common::RegisterFieldBool<24, 1, 0, Cetcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Cetcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn nvd(self) -> crate::common::RegisterFieldBool<25, 1, 0, Cetcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Cetcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Cetcr {
    #[inline(always)]
    fn default() -> Cetcr {
        <crate::RegValueT<Cetcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cstsr_SPEC;
impl crate::sealed::RegSpec for Cstsr_SPEC {
    type DataType = u32;
}

pub type Cstsr = crate::RegValueT<Cstsr_SPEC>;

impl Cstsr {
    #[inline(always)]
    pub fn cpton(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cstsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cstsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn cpfld(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        cstsr::Cpfld,
        cstsr::Cpfld,
        Cstsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            cstsr::Cpfld,
            cstsr::Cpfld,
            Cstsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn crst(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        cstsr::Crst,
        cstsr::Crst,
        Cstsr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            cstsr::Crst,
            cstsr::Crst,
            Cstsr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cstsr {
    #[inline(always)]
    fn default() -> Cstsr {
        <crate::RegValueT<Cstsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cstsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpfld_SPEC;
    pub type Cpfld = crate::EnumBitfieldStruct<u8, Cpfld_SPEC>;
    impl Cpfld {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Crst_SPEC;
    pub type Crst = crate::EnumBitfieldStruct<u8, Crst_SPEC>;
    impl Crst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdssr_SPEC;
impl crate::sealed::RegSpec for Cdssr_SPEC {
    type DataType = u32;
}

pub type Cdssr = crate::RegValueT<Cdssr_SPEC>;

impl Cdssr {
    #[inline(always)]
    pub fn cdss(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Cdssr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Cdssr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cdssr {
    #[inline(always)]
    fn default() -> Cdssr {
        <crate::RegValueT<Cdssr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdayr2_SPEC;
impl crate::sealed::RegSpec for Cdayr2_SPEC {
    type DataType = u32;
}

pub type Cdayr2 = crate::RegValueT<Cdayr2_SPEC>;

impl Cdayr2 {
    #[inline(always)]
    pub fn cayr2(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Cdayr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Cdayr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cdayr2 {
    #[inline(always)]
    fn default() -> Cdayr2 {
        <crate::RegValueT<Cdayr2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdacr2_SPEC;
impl crate::sealed::RegSpec for Cdacr2_SPEC {
    type DataType = u32;
}

pub type Cdacr2 = crate::RegValueT<Cdacr2_SPEC>;

impl Cdacr2 {
    #[inline(always)]
    pub fn cacr2(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Cdacr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Cdacr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cdacr2 {
    #[inline(always)]
    fn default() -> Cdacr2 {
        <crate::RegValueT<Cdacr2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdbyr2_SPEC;
impl crate::sealed::RegSpec for Cdbyr2_SPEC {
    type DataType = u32;
}

pub type Cdbyr2 = crate::RegValueT<Cdbyr2_SPEC>;

impl Cdbyr2 {
    #[inline(always)]
    pub fn cbyr2(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Cdbyr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Cdbyr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cdbyr2 {
    #[inline(always)]
    fn default() -> Cdbyr2 {
        <crate::RegValueT<Cdbyr2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdbcr2_SPEC;
impl crate::sealed::RegSpec for Cdbcr2_SPEC {
    type DataType = u32;
}

pub type Cdbcr2 = crate::RegValueT<Cdbcr2_SPEC>;

impl Cdbcr2 {
    #[inline(always)]
    pub fn cbcr2(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Cdbcr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Cdbcr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cdbcr2 {
    #[inline(always)]
    fn default() -> Cdbcr2 {
        <crate::RegValueT<Cdbcr2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Axibusctl2_SPEC;
impl crate::sealed::RegSpec for Axibusctl2_SPEC {
    type DataType = u32;
}

pub type Axibusctl2 = crate::RegValueT<Axibusctl2_SPEC>;

impl Axibusctl2 {
    #[inline(always)]
    pub fn awcache(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, Axibusctl2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,Axibusctl2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Axibusctl2 {
    #[inline(always)]
    fn default() -> Axibusctl2 {
        <crate::RegValueT<Axibusctl2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CamorB_SPEC;
impl crate::sealed::RegSpec for CamorB_SPEC {
    type DataType = u32;
}

pub type CamorB = crate::RegValueT<CamorB_SPEC>;

impl CamorB {
    #[inline(always)]
    pub fn hofst(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, CamorB_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16,u16,CamorB_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vofst(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, u16, CamorB_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xfff,1,0,u16,u16,CamorB_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CamorB {
    #[inline(always)]
    fn default() -> CamorB {
        <crate::RegValueT<CamorB_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CapwrB_SPEC;
impl crate::sealed::RegSpec for CapwrB_SPEC {
    type DataType = u32;
}

pub type CapwrB = crate::RegValueT<CapwrB_SPEC>;

impl CapwrB {
    #[inline(always)]
    pub fn hwdth(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, CapwrB_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16,u16,CapwrB_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vwdth(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, u16, CapwrB_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xfff,1,0,u16,u16,CapwrB_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CapwrB {
    #[inline(always)]
    fn default() -> CapwrB {
        <crate::RegValueT<CapwrB_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CflcrB_SPEC;
impl crate::sealed::RegSpec for CflcrB_SPEC {
    type DataType = u32;
}

pub type CflcrB = crate::RegValueT<CflcrB_SPEC>;

impl CflcrB {
    #[inline(always)]
    pub fn hfrac(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, CflcrB_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,CflcrB_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn hmant(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, CflcrB_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,CflcrB_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vfrac(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, u16, CflcrB_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xfff,1,0,u16,u16,CflcrB_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vmant(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, u8, CflcrB_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0xf,1,0,u8,u8,CflcrB_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CflcrB {
    #[inline(always)]
    fn default() -> CflcrB {
        <crate::RegValueT<CflcrB_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CfszrB_SPEC;
impl crate::sealed::RegSpec for CfszrB_SPEC {
    type DataType = u32;
}

pub type CfszrB = crate::RegValueT<CfszrB_SPEC>;

impl CfszrB {
    #[inline(always)]
    pub fn hfclp(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, CfszrB_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,CfszrB_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vfclp(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, u16, CfszrB_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xfff,1,0,u16,u16,CfszrB_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CfszrB {
    #[inline(always)]
    fn default() -> CfszrB {
        <crate::RegValueT<CfszrB_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CdwdrB_SPEC;
impl crate::sealed::RegSpec for CdwdrB_SPEC {
    type DataType = u32;
}

pub type CdwdrB = crate::RegValueT<CdwdrB_SPEC>;

impl CdwdrB {
    #[inline(always)]
    pub fn chdw(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, CdwdrB_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16,u16,CdwdrB_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CdwdrB {
    #[inline(always)]
    fn default() -> CdwdrB {
        <crate::RegValueT<CdwdrB_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CdayrB_SPEC;
impl crate::sealed::RegSpec for CdayrB_SPEC {
    type DataType = u32;
}

pub type CdayrB = crate::RegValueT<CdayrB_SPEC>;

impl CdayrB {
    #[inline(always)]
    pub fn cayr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, CdayrB_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,CdayrB_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CdayrB {
    #[inline(always)]
    fn default() -> CdayrB {
        <crate::RegValueT<CdayrB_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CdacrB_SPEC;
impl crate::sealed::RegSpec for CdacrB_SPEC {
    type DataType = u32;
}

pub type CdacrB = crate::RegValueT<CdacrB_SPEC>;

impl CdacrB {
    #[inline(always)]
    pub fn cacr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, CdacrB_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,CdacrB_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CdacrB {
    #[inline(always)]
    fn default() -> CdacrB {
        <crate::RegValueT<CdacrB_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CdbyrB_SPEC;
impl crate::sealed::RegSpec for CdbyrB_SPEC {
    type DataType = u32;
}

pub type CdbyrB = crate::RegValueT<CdbyrB_SPEC>;

impl CdbyrB {
    #[inline(always)]
    pub fn cbyr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, CdbyrB_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,CdbyrB_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CdbyrB {
    #[inline(always)]
    fn default() -> CdbyrB {
        <crate::RegValueT<CdbyrB_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CdbcrB_SPEC;
impl crate::sealed::RegSpec for CdbcrB_SPEC {
    type DataType = u32;
}

pub type CdbcrB = crate::RegValueT<CdbcrB_SPEC>;

impl CdbcrB {
    #[inline(always)]
    pub fn cbcr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, CdbcrB_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,CdbcrB_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CdbcrB {
    #[inline(always)]
    fn default() -> CdbcrB {
        <crate::RegValueT<CdbcrB_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CbdsrB_SPEC;
impl crate::sealed::RegSpec for CbdsrB_SPEC {
    type DataType = u32;
}

pub type CbdsrB = crate::RegValueT<CbdsrB_SPEC>;

impl CbdsrB {
    #[inline(always)]
    pub fn cbvs(
        self,
    ) -> crate::common::RegisterField<0, 0x7fffff, 1, 0, u32, u32, CbdsrB_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7fffff,1,0,u32,u32,CbdsrB_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CbdsrB {
    #[inline(always)]
    fn default() -> CbdsrB {
        <crate::RegValueT<CbdsrB_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClfcrB_SPEC;
impl crate::sealed::RegSpec for ClfcrB_SPEC {
    type DataType = u32;
}

pub type ClfcrB = crate::RegValueT<ClfcrB_SPEC>;

impl ClfcrB {
    #[inline(always)]
    pub fn lpf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        clfcr_b::Lpf,
        clfcr_b::Lpf,
        ClfcrB_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            clfcr_b::Lpf,
            clfcr_b::Lpf,
            ClfcrB_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ClfcrB {
    #[inline(always)]
    fn default() -> ClfcrB {
        <crate::RegValueT<ClfcrB_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod clfcr_b {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lpf_SPEC;
    pub type Lpf = crate::EnumBitfieldStruct<u8, Lpf_SPEC>;
    impl Lpf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CdocrB_SPEC;
impl crate::sealed::RegSpec for CdocrB_SPEC {
    type DataType = u32;
}

pub type CdocrB = crate::RegValueT<CdocrB_SPEC>;

impl CdocrB {
    #[inline(always)]
    pub fn cobs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cdocr_b::Cobs,
        cdocr_b::Cobs,
        CdocrB_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cdocr_b::Cobs,
            cdocr_b::Cobs,
            CdocrB_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cows(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cdocr_b::Cows,
        cdocr_b::Cows,
        CdocrB_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cdocr_b::Cows,
            cdocr_b::Cows,
            CdocrB_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cols(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cdocr_b::Cols,
        cdocr_b::Cols,
        CdocrB_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cdocr_b::Cols,
            cdocr_b::Cols,
            CdocrB_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cds(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        cdocr_b::Cds,
        cdocr_b::Cds,
        CdocrB_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            cdocr_b::Cds,
            cdocr_b::Cds,
            CdocrB_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cbe(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        cdocr_b::Cbe,
        cdocr_b::Cbe,
        CdocrB_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            cdocr_b::Cbe,
            cdocr_b::Cbe,
            CdocrB_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CdocrB {
    #[inline(always)]
    fn default() -> CdocrB {
        <crate::RegValueT<CdocrB_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cdocr_b {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cobs_SPEC;
    pub type Cobs = crate::EnumBitfieldStruct<u8, Cobs_SPEC>;
    impl Cobs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cows_SPEC;
    pub type Cows = crate::EnumBitfieldStruct<u8, Cows_SPEC>;
    impl Cows {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cols_SPEC;
    pub type Cols = crate::EnumBitfieldStruct<u8, Cols_SPEC>;
    impl Cols {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cds_SPEC;
    pub type Cds = crate::EnumBitfieldStruct<u8, Cds_SPEC>;
    impl Cds {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cbe_SPEC;
    pub type Cbe = crate::EnumBitfieldStruct<u8, Cbe_SPEC>;
    impl Cbe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdayr2B_SPEC;
impl crate::sealed::RegSpec for Cdayr2B_SPEC {
    type DataType = u32;
}

pub type Cdayr2B = crate::RegValueT<Cdayr2B_SPEC>;

impl Cdayr2B {
    #[inline(always)]
    pub fn cayr2(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Cdayr2B_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Cdayr2B_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cdayr2B {
    #[inline(always)]
    fn default() -> Cdayr2B {
        <crate::RegValueT<Cdayr2B_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdacr2B_SPEC;
impl crate::sealed::RegSpec for Cdacr2B_SPEC {
    type DataType = u32;
}

pub type Cdacr2B = crate::RegValueT<Cdacr2B_SPEC>;

impl Cdacr2B {
    #[inline(always)]
    pub fn cacr2(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Cdacr2B_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Cdacr2B_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cdacr2B {
    #[inline(always)]
    fn default() -> Cdacr2B {
        <crate::RegValueT<Cdacr2B_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdbyr2B_SPEC;
impl crate::sealed::RegSpec for Cdbyr2B_SPEC {
    type DataType = u32;
}

pub type Cdbyr2B = crate::RegValueT<Cdbyr2B_SPEC>;

impl Cdbyr2B {
    #[inline(always)]
    pub fn cbyr2(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Cdbyr2B_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Cdbyr2B_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cdbyr2B {
    #[inline(always)]
    fn default() -> Cdbyr2B {
        <crate::RegValueT<Cdbyr2B_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdbcr2B_SPEC;
impl crate::sealed::RegSpec for Cdbcr2B_SPEC {
    type DataType = u32;
}

pub type Cdbcr2B = crate::RegValueT<Cdbcr2B_SPEC>;

impl Cdbcr2B {
    #[inline(always)]
    pub fn cbcr2(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Cdbcr2B_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Cdbcr2B_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cdbcr2B {
    #[inline(always)]
    fn default() -> Cdbcr2B {
        <crate::RegValueT<Cdbcr2B_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CamorM_SPEC;
impl crate::sealed::RegSpec for CamorM_SPEC {
    type DataType = u32;
}

pub type CamorM = crate::RegValueT<CamorM_SPEC>;

impl CamorM {
    #[inline(always)]
    pub fn hofst(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, CamorM_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16,u16,CamorM_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vofst(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, u16, CamorM_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xfff,1,0,u16,u16,CamorM_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CamorM {
    #[inline(always)]
    fn default() -> CamorM {
        <crate::RegValueT<CamorM_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CapwrM_SPEC;
impl crate::sealed::RegSpec for CapwrM_SPEC {
    type DataType = u32;
}

pub type CapwrM = crate::RegValueT<CapwrM_SPEC>;

impl CapwrM {
    #[inline(always)]
    pub fn hwdth(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, CapwrM_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16,u16,CapwrM_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vwdth(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, u16, CapwrM_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xfff,1,0,u16,u16,CapwrM_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CapwrM {
    #[inline(always)]
    fn default() -> CapwrM {
        <crate::RegValueT<CapwrM_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CflcrM_SPEC;
impl crate::sealed::RegSpec for CflcrM_SPEC {
    type DataType = u32;
}

pub type CflcrM = crate::RegValueT<CflcrM_SPEC>;

impl CflcrM {
    #[inline(always)]
    pub fn hfrac(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, CflcrM_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,CflcrM_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn hmant(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, CflcrM_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,CflcrM_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vfrac(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, u16, CflcrM_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xfff,1,0,u16,u16,CflcrM_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vmant(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, u8, CflcrM_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0xf,1,0,u8,u8,CflcrM_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CflcrM {
    #[inline(always)]
    fn default() -> CflcrM {
        <crate::RegValueT<CflcrM_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CfszrM_SPEC;
impl crate::sealed::RegSpec for CfszrM_SPEC {
    type DataType = u32;
}

pub type CfszrM = crate::RegValueT<CfszrM_SPEC>;

impl CfszrM {
    #[inline(always)]
    pub fn hfclp(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, CfszrM_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,CfszrM_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vfclp(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, u16, CfszrM_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xfff,1,0,u16,u16,CfszrM_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CfszrM {
    #[inline(always)]
    fn default() -> CfszrM {
        <crate::RegValueT<CfszrM_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CdwdrM_SPEC;
impl crate::sealed::RegSpec for CdwdrM_SPEC {
    type DataType = u32;
}

pub type CdwdrM = crate::RegValueT<CdwdrM_SPEC>;

impl CdwdrM {
    #[inline(always)]
    pub fn chdw(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, CdwdrM_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16,u16,CdwdrM_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CdwdrM {
    #[inline(always)]
    fn default() -> CdwdrM {
        <crate::RegValueT<CdwdrM_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CdayrM_SPEC;
impl crate::sealed::RegSpec for CdayrM_SPEC {
    type DataType = u32;
}

pub type CdayrM = crate::RegValueT<CdayrM_SPEC>;

impl CdayrM {
    #[inline(always)]
    pub fn cayr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, CdayrM_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,CdayrM_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CdayrM {
    #[inline(always)]
    fn default() -> CdayrM {
        <crate::RegValueT<CdayrM_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CdacrM_SPEC;
impl crate::sealed::RegSpec for CdacrM_SPEC {
    type DataType = u32;
}

pub type CdacrM = crate::RegValueT<CdacrM_SPEC>;

impl CdacrM {
    #[inline(always)]
    pub fn cacr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, CdacrM_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,CdacrM_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CdacrM {
    #[inline(always)]
    fn default() -> CdacrM {
        <crate::RegValueT<CdacrM_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CdbyrM_SPEC;
impl crate::sealed::RegSpec for CdbyrM_SPEC {
    type DataType = u32;
}

pub type CdbyrM = crate::RegValueT<CdbyrM_SPEC>;

impl CdbyrM {
    #[inline(always)]
    pub fn cbyr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, CdbyrM_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,CdbyrM_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CdbyrM {
    #[inline(always)]
    fn default() -> CdbyrM {
        <crate::RegValueT<CdbyrM_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CdbcrM_SPEC;
impl crate::sealed::RegSpec for CdbcrM_SPEC {
    type DataType = u32;
}

pub type CdbcrM = crate::RegValueT<CdbcrM_SPEC>;

impl CdbcrM {
    #[inline(always)]
    pub fn cbcr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, CdbcrM_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,CdbcrM_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CdbcrM {
    #[inline(always)]
    fn default() -> CdbcrM {
        <crate::RegValueT<CdbcrM_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CbdsrM_SPEC;
impl crate::sealed::RegSpec for CbdsrM_SPEC {
    type DataType = u32;
}

pub type CbdsrM = crate::RegValueT<CbdsrM_SPEC>;

impl CbdsrM {
    #[inline(always)]
    pub fn cbvs(
        self,
    ) -> crate::common::RegisterField<0, 0x7fffff, 1, 0, u32, u32, CbdsrM_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7fffff,1,0,u32,u32,CbdsrM_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CbdsrM {
    #[inline(always)]
    fn default() -> CbdsrM {
        <crate::RegValueT<CbdsrM_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClfcrM_SPEC;
impl crate::sealed::RegSpec for ClfcrM_SPEC {
    type DataType = u32;
}

pub type ClfcrM = crate::RegValueT<ClfcrM_SPEC>;

impl ClfcrM {
    #[inline(always)]
    pub fn lpf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        clfcr_m::Lpf,
        clfcr_m::Lpf,
        ClfcrM_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            clfcr_m::Lpf,
            clfcr_m::Lpf,
            ClfcrM_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ClfcrM {
    #[inline(always)]
    fn default() -> ClfcrM {
        <crate::RegValueT<ClfcrM_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod clfcr_m {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lpf_SPEC;
    pub type Lpf = crate::EnumBitfieldStruct<u8, Lpf_SPEC>;
    impl Lpf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CdocrM_SPEC;
impl crate::sealed::RegSpec for CdocrM_SPEC {
    type DataType = u32;
}

pub type CdocrM = crate::RegValueT<CdocrM_SPEC>;

impl CdocrM {
    #[inline(always)]
    pub fn cobs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cdocr_m::Cobs,
        cdocr_m::Cobs,
        CdocrM_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cdocr_m::Cobs,
            cdocr_m::Cobs,
            CdocrM_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cows(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cdocr_m::Cows,
        cdocr_m::Cows,
        CdocrM_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cdocr_m::Cows,
            cdocr_m::Cows,
            CdocrM_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cols(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cdocr_m::Cols,
        cdocr_m::Cols,
        CdocrM_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cdocr_m::Cols,
            cdocr_m::Cols,
            CdocrM_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cds(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        cdocr_m::Cds,
        cdocr_m::Cds,
        CdocrM_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            cdocr_m::Cds,
            cdocr_m::Cds,
            CdocrM_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cbe(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        cdocr_m::Cbe,
        cdocr_m::Cbe,
        CdocrM_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            cdocr_m::Cbe,
            cdocr_m::Cbe,
            CdocrM_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for CdocrM {
    #[inline(always)]
    fn default() -> CdocrM {
        <crate::RegValueT<CdocrM_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cdocr_m {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cobs_SPEC;
    pub type Cobs = crate::EnumBitfieldStruct<u8, Cobs_SPEC>;
    impl Cobs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cows_SPEC;
    pub type Cows = crate::EnumBitfieldStruct<u8, Cows_SPEC>;
    impl Cows {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cols_SPEC;
    pub type Cols = crate::EnumBitfieldStruct<u8, Cols_SPEC>;
    impl Cols {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cds_SPEC;
    pub type Cds = crate::EnumBitfieldStruct<u8, Cds_SPEC>;
    impl Cds {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cbe_SPEC;
    pub type Cbe = crate::EnumBitfieldStruct<u8, Cbe_SPEC>;
    impl Cbe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdayr2M_SPEC;
impl crate::sealed::RegSpec for Cdayr2M_SPEC {
    type DataType = u32;
}

pub type Cdayr2M = crate::RegValueT<Cdayr2M_SPEC>;

impl Cdayr2M {
    #[inline(always)]
    pub fn cayr2(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Cdayr2M_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Cdayr2M_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cdayr2M {
    #[inline(always)]
    fn default() -> Cdayr2M {
        <crate::RegValueT<Cdayr2M_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdacr2M_SPEC;
impl crate::sealed::RegSpec for Cdacr2M_SPEC {
    type DataType = u32;
}

pub type Cdacr2M = crate::RegValueT<Cdacr2M_SPEC>;

impl Cdacr2M {
    #[inline(always)]
    pub fn cacr2(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Cdacr2M_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Cdacr2M_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cdacr2M {
    #[inline(always)]
    fn default() -> Cdacr2M {
        <crate::RegValueT<Cdacr2M_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdbyr2M_SPEC;
impl crate::sealed::RegSpec for Cdbyr2M_SPEC {
    type DataType = u32;
}

pub type Cdbyr2M = crate::RegValueT<Cdbyr2M_SPEC>;

impl Cdbyr2M {
    #[inline(always)]
    pub fn cbyr2(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Cdbyr2M_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Cdbyr2M_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cdbyr2M {
    #[inline(always)]
    fn default() -> Cdbyr2M {
        <crate::RegValueT<Cdbyr2M_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdbcr2M_SPEC;
impl crate::sealed::RegSpec for Cdbcr2M_SPEC {
    type DataType = u32;
}

pub type Cdbcr2M = crate::RegValueT<Cdbcr2M_SPEC>;

impl Cdbcr2M {
    #[inline(always)]
    pub fn cbcr2(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Cdbcr2M_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Cdbcr2M_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cdbcr2M {
    #[inline(always)]
    fn default() -> Cdbcr2M {
        <crate::RegValueT<Cdbcr2M_SPEC> as RegisterValue<_>>::new(0)
    }
}
