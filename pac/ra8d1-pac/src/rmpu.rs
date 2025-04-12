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
#[doc = r"Renesas Memory Protection Unit"]
unsafe impl ::core::marker::Send for super::Rmpu {}
unsafe impl ::core::marker::Sync for super::Rmpu {}
impl super::Rmpu {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "MMPU Operation After Detection Register"]
    #[inline(always)]
    pub const fn mmpuoad(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuoad_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuoad_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "MMPU Operation After Detection Protect Register"]
    #[inline(always)]
    pub const fn mmpuoadpt(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuoadpt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuoadpt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "MMPU Enable Register for DMAC"]
    #[inline(always)]
    pub const fn mmpuendmac(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuendmac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuendmac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[doc = "MMPU Enable Protect Register for DMAC"]
    #[inline(always)]
    pub const fn mmpuenptdmac(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuenptdmac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuenptdmac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(260usize),
            )
        }
    }

    #[doc = "MMPU Regions Protect Register for DMAC"]
    #[inline(always)]
    pub const fn mmpurptdmac(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpurptdmac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpurptdmac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(264usize),
            )
        }
    }

    #[doc = "MMPU Regions Protect register for DMAC Secure"]
    #[inline(always)]
    pub const fn mmpurptdmac_sec(
        &self,
    ) -> &'static crate::common::Reg<self::MmpurptdmacSec_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::MmpurptdmacSec_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(268usize),
            )
        }
    }

    #[doc = "MMPU Access Control Register for DMAC %s"]
    #[inline(always)]
    pub const fn mmpuacdmac(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuacdmac_SPEC, crate::common::RW>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x200usize))
        }
    }

    #[doc = "MMPU Start Address Register for DMAC %s"]
    #[inline(always)]
    pub const fn mmpusdmac(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpusdmac_SPEC, crate::common::RW>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x204usize))
        }
    }

    #[doc = "MMPU End Address Register for DMAC %s"]
    #[inline(always)]
    pub const fn mmpuedmac(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuedmac_SPEC, crate::common::RW>,
        8,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x208usize))
        }
    }

    #[doc = "MMPU Enable Register for EDMAC"]
    #[inline(always)]
    pub const fn mmpuenedmac(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuenedmac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuenedmac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1280usize),
            )
        }
    }

    #[doc = "MMPU Enable Protect Register for EDMAC"]
    #[inline(always)]
    pub const fn mmpuenptedmac(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuenptedmac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuenptedmac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1284usize),
            )
        }
    }

    #[doc = "MMPU Regions Protect Register for EDMAC"]
    #[inline(always)]
    pub const fn mmpurptedmac(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpurptedmac_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpurptedmac_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1288usize),
            )
        }
    }

    #[doc = "MMPU Access Control Register for EDMAC %s"]
    #[inline(always)]
    pub const fn mmpuacedmac(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuacedmac_SPEC, crate::common::RW>,
        4,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x600usize))
        }
    }

    #[doc = "MMPU Start Address Register for EDMAC %s"]
    #[inline(always)]
    pub const fn mmpusedmac(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpusedmac_SPEC, crate::common::RW>,
        4,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x604usize))
        }
    }

    #[doc = "MMPU End Address Register for EDMAC %s"]
    #[inline(always)]
    pub const fn mmpueedmac(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpueedmac_SPEC, crate::common::RW>,
        4,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x608usize))
        }
    }

    #[doc = "MMPU Enable Register for GLCDC"]
    #[inline(always)]
    pub const fn mmpuenglcdc(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuenglcdc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuenglcdc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1792usize),
            )
        }
    }

    #[doc = "MMPU Enable Protect Register for GLCDC"]
    #[inline(always)]
    pub const fn mmpuenptglcdc(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuenptglcdc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuenptglcdc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1796usize),
            )
        }
    }

    #[doc = "MMPU Regions Protect Register for GLCDC"]
    #[inline(always)]
    pub const fn mmpurptglcdc(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpurptglcdc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpurptglcdc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1800usize),
            )
        }
    }

    #[doc = "MMPU Access Control Register for GLCDC %s"]
    #[inline(always)]
    pub const fn mmpuacglcdc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuacglcdc_SPEC, crate::common::RW>,
        2,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x800usize))
        }
    }

    #[doc = "MMPU Start Address Register for GLCDC %s"]
    #[inline(always)]
    pub const fn mmpusglcdc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpusglcdc_SPEC, crate::common::RW>,
        2,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x804usize))
        }
    }

    #[doc = "Graphics Region %s End Address Register"]
    #[inline(always)]
    pub const fn mmpueglcdc(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpueglcdc_SPEC, crate::common::RW>,
        2,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x808usize))
        }
    }

    #[doc = "MMPU Enable Register for DRW"]
    #[inline(always)]
    pub const fn mmpuendrw(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuendrw_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuendrw_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2304usize),
            )
        }
    }

    #[doc = "MMPU Enable Protect Register for DRW"]
    #[inline(always)]
    pub const fn mmpuenpdrw(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuenpdrw_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuenpdrw_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2308usize),
            )
        }
    }

    #[doc = "MMPU Regions Protect Register for DRW"]
    #[inline(always)]
    pub const fn mmpurptdrw(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpurptdrw_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpurptdrw_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2312usize),
            )
        }
    }

    #[doc = "MMPU Access Control Register for DRW %s"]
    #[inline(always)]
    pub const fn mmpuacdrw(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuacdrw_SPEC, crate::common::RW>,
        3,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xa00usize))
        }
    }

    #[doc = "MMPU Start Address Register for DRW %s"]
    #[inline(always)]
    pub const fn mmpusdrw(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpusdrw_SPEC, crate::common::RW>,
        3,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xa04usize))
        }
    }

    #[doc = "Graphics Region %s End Address Register"]
    #[inline(always)]
    pub const fn mmpuedrw(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuedrw_SPEC, crate::common::RW>,
        2,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xa08usize))
        }
    }

    #[doc = "MMPU Enable Register for MIPI"]
    #[inline(always)]
    pub const fn mmpuenmipi(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuenmipi_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuenmipi_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2816usize),
            )
        }
    }

    #[doc = "MMPU Enable Protect Register for MIPI"]
    #[inline(always)]
    pub const fn mmpuenptmipi(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuenptmipi_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuenptmipi_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2820usize),
            )
        }
    }

    #[doc = "MMPU Regions Protect Register for MIPI"]
    #[inline(always)]
    pub const fn mmpurptmipi(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpurptmipi_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpurptmipi_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2824usize),
            )
        }
    }

    #[doc = "MMPU Access Control Register for MIPI"]
    #[inline(always)]
    pub const fn mmpuacmipi(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuacmipi_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuacmipi_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3072usize),
            )
        }
    }

    #[doc = "MMPU Start Address Register for MIPI"]
    #[inline(always)]
    pub const fn mmpusmipi(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpusmipi_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpusmipi_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3076usize),
            )
        }
    }

    #[doc = "MMPU End Address Register for MIPI"]
    #[inline(always)]
    pub const fn mmpuemipi(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuemipi_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuemipi_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3080usize),
            )
        }
    }

    #[doc = "MMPU Enable Register for CEU"]
    #[inline(always)]
    pub const fn mmpuenceu(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuenceu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuenceu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3328usize),
            )
        }
    }

    #[doc = "MMPU Enable Protect Register for CEU"]
    #[inline(always)]
    pub const fn mmpuenptceu(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpuenptceu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpuenptceu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3332usize),
            )
        }
    }

    #[doc = "MMPU Regions Protect Register for CEU"]
    #[inline(always)]
    pub const fn mmpurptceu(
        &self,
    ) -> &'static crate::common::Reg<self::Mmpurptceu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mmpurptceu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3336usize),
            )
        }
    }

    #[doc = "MMPU Access Control Register for CEU %s"]
    #[inline(always)]
    pub const fn mmpuacceu(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpuacceu_SPEC, crate::common::RW>,
        2,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xe00usize))
        }
    }

    #[doc = "MMPU Start Address Register for CEU %s"]
    #[inline(always)]
    pub const fn mmpusceu(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpusceu_SPEC, crate::common::RW>,
        2,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xe04usize))
        }
    }

    #[doc = "MMPU End Address Register for CEU %s"]
    #[inline(always)]
    pub const fn mmpueceu(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mmpueceu_SPEC, crate::common::RW>,
        2,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xe08usize))
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuoad_SPEC;
impl crate::sealed::RegSpec for Mmpuoad_SPEC {
    type DataType = u16;
}
#[doc = "MMPU Operation After Detection Register"]
pub type Mmpuoad = crate::RegValueT<Mmpuoad_SPEC>;

impl Mmpuoad {
    #[doc = "Operation after detection"]
    #[inline(always)]
    pub fn oad(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, mmpuoad::Oad, Mmpuoad_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,mmpuoad::Oad, Mmpuoad_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, Mmpuoad_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8, Mmpuoad_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Mmpuoad_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Mmpuoad_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuoad {
    #[inline(always)]
    fn default() -> Mmpuoad {
        <crate::RegValueT<Mmpuoad_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuoad {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oad_SPEC;
    pub type Oad = crate::EnumBitfieldStruct<u8, Oad_SPEC>;
    impl Oad {
        #[doc = "NMI"]
        pub const _0: Self = Self::new(0);
        #[doc = "Reset"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuoadpt_SPEC;
impl crate::sealed::RegSpec for Mmpuoadpt_SPEC {
    type DataType = u16;
}
#[doc = "MMPU Operation After Detection Protect Register"]
pub type Mmpuoadpt = crate::RegValueT<Mmpuoadpt_SPEC>;

impl Mmpuoadpt {
    #[doc = "Protection of register"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuoadpt::Protect,
        Mmpuoadpt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuoadpt::Protect,
            Mmpuoadpt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, Mmpuoadpt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8, Mmpuoadpt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Key code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Mmpuoadpt_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Mmpuoadpt_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuoadpt {
    #[inline(always)]
    fn default() -> Mmpuoadpt {
        <crate::RegValueT<Mmpuoadpt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuoadpt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "MMPUOAD register writing is possible."]
        pub const _0: Self = Self::new(0);
        #[doc = "MMPUOAD register writing is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuendmac_SPEC;
impl crate::sealed::RegSpec for Mmpuendmac_SPEC {
    type DataType = u16;
}
#[doc = "MMPU Enable Register for DMAC"]
pub type Mmpuendmac = crate::RegValueT<Mmpuendmac_SPEC>;

impl Mmpuendmac {
    #[doc = "Bus master MPU of DMAC Enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuendmac::Enable,
        Mmpuendmac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuendmac::Enable,
            Mmpuendmac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, Mmpuendmac_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8, Mmpuendmac_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Mmpuendmac_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Mmpuendmac_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuendmac {
    #[inline(always)]
    fn default() -> Mmpuendmac {
        <crate::RegValueT<Mmpuendmac_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuendmac {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "Bus master MPU of DMAC is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Bus master MPU of DMAC is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuenptdmac_SPEC;
impl crate::sealed::RegSpec for Mmpuenptdmac_SPEC {
    type DataType = u16;
}
#[doc = "MMPU Enable Protect Register for DMAC"]
pub type Mmpuenptdmac = crate::RegValueT<Mmpuenptdmac_SPEC>;

impl Mmpuenptdmac {
    #[doc = "Protection of register"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuenptdmac::Protect,
        Mmpuenptdmac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuenptdmac::Protect,
            Mmpuenptdmac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, Mmpuenptdmac_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8, Mmpuenptdmac_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Mmpuenptdmac_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Mmpuenptdmac_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuenptdmac {
    #[inline(always)]
    fn default() -> Mmpuenptdmac {
        <crate::RegValueT<Mmpuenptdmac_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuenptdmac {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "MMPUENDMAC register write is possible"]
        pub const _0: Self = Self::new(0);
        #[doc = "MMPUENDMAC register write is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpurptdmac_SPEC;
impl crate::sealed::RegSpec for Mmpurptdmac_SPEC {
    type DataType = u16;
}
#[doc = "MMPU Regions Protect Register for DMAC"]
pub type Mmpurptdmac = crate::RegValueT<Mmpurptdmac_SPEC>;

impl Mmpurptdmac {
    #[doc = "Protection of register"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpurptdmac::Protect,
        Mmpurptdmac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpurptdmac::Protect,
            Mmpurptdmac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, Mmpurptdmac_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8, Mmpurptdmac_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Mmpurptdmac_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Mmpurptdmac_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpurptdmac {
    #[inline(always)]
    fn default() -> Mmpurptdmac {
        <crate::RegValueT<Mmpurptdmac_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpurptdmac {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "Bus master MPU register for DMAC write is possible"]
        pub const _0: Self = Self::new(0);
        #[doc = "Bus master MPU register for DMAC write is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MmpurptdmacSec_SPEC;
impl crate::sealed::RegSpec for MmpurptdmacSec_SPEC {
    type DataType = u16;
}
#[doc = "MMPU Regions Protect register for DMAC Secure"]
pub type MmpurptdmacSec = crate::RegValueT<MmpurptdmacSec_SPEC>;

impl MmpurptdmacSec {
    #[doc = "Protection of register"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpurptdmac_sec::Protect,
        MmpurptdmacSec_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpurptdmac_sec::Protect,
            MmpurptdmacSec_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, MmpurptdmacSec_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7f,1,0,u8, MmpurptdmacSec_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, MmpurptdmacSec_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8, MmpurptdmacSec_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for MmpurptdmacSec {
    #[inline(always)]
    fn default() -> MmpurptdmacSec {
        <crate::RegValueT<MmpurptdmacSec_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpurptdmac_sec {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "Bus master MPU register for DMAC Secure write is possible"]
        pub const _0: Self = Self::new(0);
        #[doc = "Bus master MPU register for DMAC Secure write is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuacdmac_SPEC;
impl crate::sealed::RegSpec for Mmpuacdmac_SPEC {
    type DataType = u16;
}
#[doc = "MMPU Access Control Register for DMAC %s"]
pub type Mmpuacdmac = crate::RegValueT<Mmpuacdmac_SPEC>;

impl Mmpuacdmac {
    #[doc = "Region Enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuacdmac::Enable,
        Mmpuacdmac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuacdmac::Enable,
            Mmpuacdmac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Read protection"]
    #[inline(always)]
    pub fn rp(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mmpuacdmac::Rp,
        Mmpuacdmac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mmpuacdmac::Rp,
            Mmpuacdmac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn wp(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mmpuacdmac::Wp,
        Mmpuacdmac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mmpuacdmac::Wp,
            Mmpuacdmac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Privilege protection"]
    #[inline(always)]
    pub fn pp(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        mmpuacdmac::Pp,
        Mmpuacdmac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            mmpuacdmac::Pp,
            Mmpuacdmac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 000000000000. The write value should be 000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xfff, 1, 0, u16, Mmpuacdmac_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xfff,1,0,u16, Mmpuacdmac_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuacdmac {
    #[inline(always)]
    fn default() -> Mmpuacdmac {
        <crate::RegValueT<Mmpuacdmac_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuacdmac {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "DMAC region n unit is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "DMAC region n unit is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rp_SPEC;
    pub type Rp = crate::EnumBitfieldStruct<u8, Rp_SPEC>;
    impl Rp {
        #[doc = "Read permission"]
        pub const _0: Self = Self::new(0);
        #[doc = "Read protection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wp_SPEC;
    pub type Wp = crate::EnumBitfieldStruct<u8, Wp_SPEC>;
    impl Wp {
        #[doc = "Write permission"]
        pub const _0: Self = Self::new(0);
        #[doc = "Write protection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pp_SPEC;
    pub type Pp = crate::EnumBitfieldStruct<u8, Pp_SPEC>;
    impl Pp {
        #[doc = "Unprivileged access permission"]
        pub const _0: Self = Self::new(0);
        #[doc = "Unprivileged access protection"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpusdmac_SPEC;
impl crate::sealed::RegSpec for Mmpusdmac_SPEC {
    type DataType = u32;
}
#[doc = "MMPU Start Address Register for DMAC %s"]
pub type Mmpusdmac = crate::RegValueT<Mmpusdmac_SPEC>;

impl Mmpusdmac {
    #[doc = "Region Stat Address : Address where the region starts, for use in region determination.NOTE: The low-order 5 bits are fixed to 0."]
    #[inline(always)]
    pub fn mmpus(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Mmpusdmac_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Mmpusdmac_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpusdmac {
    #[inline(always)]
    fn default() -> Mmpusdmac {
        <crate::RegValueT<Mmpusdmac_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuedmac_SPEC;
impl crate::sealed::RegSpec for Mmpuedmac_SPEC {
    type DataType = u32;
}
#[doc = "MMPU End Address Register for DMAC %s"]
pub type Mmpuedmac = crate::RegValueT<Mmpuedmac_SPEC>;

impl Mmpuedmac {
    #[doc = "Region End Address : Address where the region end, for use in region determination.NOTE: The low-order 5 bits are fixed to 1."]
    #[inline(always)]
    pub fn mmpue(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Mmpuedmac_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Mmpuedmac_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuedmac {
    #[inline(always)]
    fn default() -> Mmpuedmac {
        <crate::RegValueT<Mmpuedmac_SPEC> as RegisterValue<_>>::new(31)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuenedmac_SPEC;
impl crate::sealed::RegSpec for Mmpuenedmac_SPEC {
    type DataType = u16;
}
#[doc = "MMPU Enable Register for EDMAC"]
pub type Mmpuenedmac = crate::RegValueT<Mmpuenedmac_SPEC>;

impl Mmpuenedmac {
    #[doc = "Bus master MPU of EDMAC Enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuenedmac::Enable,
        Mmpuenedmac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuenedmac::Enable,
            Mmpuenedmac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, Mmpuenedmac_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8, Mmpuenedmac_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Mmpuenedmac_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Mmpuenedmac_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuenedmac {
    #[inline(always)]
    fn default() -> Mmpuenedmac {
        <crate::RegValueT<Mmpuenedmac_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuenedmac {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "Bus master MPU of EDMAC is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Bus master MPU of EDMAC is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuenptedmac_SPEC;
impl crate::sealed::RegSpec for Mmpuenptedmac_SPEC {
    type DataType = u16;
}
#[doc = "MMPU Enable Protect Register for EDMAC"]
pub type Mmpuenptedmac = crate::RegValueT<Mmpuenptedmac_SPEC>;

impl Mmpuenptedmac {
    #[doc = "Protection of register"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuenptedmac::Protect,
        Mmpuenptedmac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuenptedmac::Protect,
            Mmpuenptedmac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, Mmpuenptedmac_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7f,1,0,u8, Mmpuenptedmac_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Mmpuenptedmac_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Mmpuenptedmac_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuenptedmac {
    #[inline(always)]
    fn default() -> Mmpuenptedmac {
        <crate::RegValueT<Mmpuenptedmac_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuenptedmac {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "MMPUENEDMAC register write is possible"]
        pub const _0: Self = Self::new(0);
        #[doc = "MMPUENEDMAC register write is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpurptedmac_SPEC;
impl crate::sealed::RegSpec for Mmpurptedmac_SPEC {
    type DataType = u16;
}
#[doc = "MMPU Regions Protect Register for EDMAC"]
pub type Mmpurptedmac = crate::RegValueT<Mmpurptedmac_SPEC>;

impl Mmpurptedmac {
    #[doc = "Protection of register"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpurptedmac::Protect,
        Mmpurptedmac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpurptedmac::Protect,
            Mmpurptedmac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, Mmpurptedmac_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8, Mmpurptedmac_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Mmpurptedmac_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Mmpurptedmac_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpurptedmac {
    #[inline(always)]
    fn default() -> Mmpurptedmac {
        <crate::RegValueT<Mmpurptedmac_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpurptedmac {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "Bus master MPU register for EDMAC write is possible"]
        pub const _0: Self = Self::new(0);
        #[doc = "Bus master MPU register for EDMAC write is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuacedmac_SPEC;
impl crate::sealed::RegSpec for Mmpuacedmac_SPEC {
    type DataType = u16;
}
#[doc = "MMPU Access Control Register for EDMAC %s"]
pub type Mmpuacedmac = crate::RegValueT<Mmpuacedmac_SPEC>;

impl Mmpuacedmac {
    #[doc = "Region Enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuacedmac::Enable,
        Mmpuacedmac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuacedmac::Enable,
            Mmpuacedmac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Read protection"]
    #[inline(always)]
    pub fn rp(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mmpuacedmac::Rp,
        Mmpuacedmac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mmpuacedmac::Rp,
            Mmpuacedmac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn wp(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mmpuacedmac::Wp,
        Mmpuacedmac_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mmpuacedmac::Wp,
            Mmpuacedmac_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 000000000000. The write value should be 000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xfff, 1, 0, u16, Mmpuacedmac_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xfff,1,0,u16, Mmpuacedmac_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuacedmac {
    #[inline(always)]
    fn default() -> Mmpuacedmac {
        <crate::RegValueT<Mmpuacedmac_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuacedmac {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "EDMAC region n unit is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "EDMAC region n unit is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rp_SPEC;
    pub type Rp = crate::EnumBitfieldStruct<u8, Rp_SPEC>;
    impl Rp {
        #[doc = "Read permission"]
        pub const _0: Self = Self::new(0);
        #[doc = "Read protection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wp_SPEC;
    pub type Wp = crate::EnumBitfieldStruct<u8, Wp_SPEC>;
    impl Wp {
        #[doc = "Write permission"]
        pub const _0: Self = Self::new(0);
        #[doc = "Write protection"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpusedmac_SPEC;
impl crate::sealed::RegSpec for Mmpusedmac_SPEC {
    type DataType = u32;
}
#[doc = "MMPU Start Address Register for EDMAC %s"]
pub type Mmpusedmac = crate::RegValueT<Mmpusedmac_SPEC>;

impl Mmpusedmac {
    #[doc = "Region Stat Address : Address where the region starts, for use in region determination.NOTE: The low-order 5 bits are fixed to 0."]
    #[inline(always)]
    pub fn mmpus(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Mmpusedmac_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Mmpusedmac_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpusedmac {
    #[inline(always)]
    fn default() -> Mmpusedmac {
        <crate::RegValueT<Mmpusedmac_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpueedmac_SPEC;
impl crate::sealed::RegSpec for Mmpueedmac_SPEC {
    type DataType = u32;
}
#[doc = "MMPU End Address Register for EDMAC %s"]
pub type Mmpueedmac = crate::RegValueT<Mmpueedmac_SPEC>;

impl Mmpueedmac {
    #[doc = "Region End Address : Address where the region end, for use in region determination.NOTE: The low-order 5 bits are fixed to 1."]
    #[inline(always)]
    pub fn mmpue(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Mmpueedmac_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Mmpueedmac_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpueedmac {
    #[inline(always)]
    fn default() -> Mmpueedmac {
        <crate::RegValueT<Mmpueedmac_SPEC> as RegisterValue<_>>::new(31)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuenglcdc_SPEC;
impl crate::sealed::RegSpec for Mmpuenglcdc_SPEC {
    type DataType = u16;
}
#[doc = "MMPU Enable Register for GLCDC"]
pub type Mmpuenglcdc = crate::RegValueT<Mmpuenglcdc_SPEC>;

impl Mmpuenglcdc {
    #[doc = "Bus master MPU of GLCDC Enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuenglcdc::Enable,
        Mmpuenglcdc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuenglcdc::Enable,
            Mmpuenglcdc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, Mmpuenglcdc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8, Mmpuenglcdc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Mmpuenglcdc_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Mmpuenglcdc_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuenglcdc {
    #[inline(always)]
    fn default() -> Mmpuenglcdc {
        <crate::RegValueT<Mmpuenglcdc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuenglcdc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "Bus master MPU of GLCDC is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Bus master MPU of GLCDC is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuenptglcdc_SPEC;
impl crate::sealed::RegSpec for Mmpuenptglcdc_SPEC {
    type DataType = u16;
}
#[doc = "MMPU Enable Protect Register for GLCDC"]
pub type Mmpuenptglcdc = crate::RegValueT<Mmpuenptglcdc_SPEC>;

impl Mmpuenptglcdc {
    #[doc = "Protection of register"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuenptglcdc::Protect,
        Mmpuenptglcdc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuenptglcdc::Protect,
            Mmpuenptglcdc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, Mmpuenptglcdc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7f,1,0,u8, Mmpuenptglcdc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Mmpuenptglcdc_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Mmpuenptglcdc_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuenptglcdc {
    #[inline(always)]
    fn default() -> Mmpuenptglcdc {
        <crate::RegValueT<Mmpuenptglcdc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuenptglcdc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "MMPUENGLCDC register write is possible"]
        pub const _0: Self = Self::new(0);
        #[doc = "MMPUENGLCDC register write is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpurptglcdc_SPEC;
impl crate::sealed::RegSpec for Mmpurptglcdc_SPEC {
    type DataType = u16;
}
#[doc = "MMPU Regions Protect Register for GLCDC"]
pub type Mmpurptglcdc = crate::RegValueT<Mmpurptglcdc_SPEC>;

impl Mmpurptglcdc {
    #[doc = "Protection of register"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpurptglcdc::Protect,
        Mmpurptglcdc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpurptglcdc::Protect,
            Mmpurptglcdc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, Mmpurptglcdc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8, Mmpurptglcdc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Mmpurptglcdc_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Mmpurptglcdc_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpurptglcdc {
    #[inline(always)]
    fn default() -> Mmpurptglcdc {
        <crate::RegValueT<Mmpurptglcdc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpurptglcdc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "Bus master MPU register for GLCDC write is possible"]
        pub const _0: Self = Self::new(0);
        #[doc = "Bus master MPU register for GLCDC write is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuacglcdc_SPEC;
impl crate::sealed::RegSpec for Mmpuacglcdc_SPEC {
    type DataType = u16;
}
#[doc = "MMPU Access Control Register for GLCDC %s"]
pub type Mmpuacglcdc = crate::RegValueT<Mmpuacglcdc_SPEC>;

impl Mmpuacglcdc {
    #[doc = "Region Enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuacglcdc::Enable,
        Mmpuacglcdc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuacglcdc::Enable,
            Mmpuacglcdc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Read protection"]
    #[inline(always)]
    pub fn rp(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mmpuacglcdc::Rp,
        Mmpuacglcdc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mmpuacglcdc::Rp,
            Mmpuacglcdc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn wp(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mmpuacglcdc::Wp,
        Mmpuacglcdc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mmpuacglcdc::Wp,
            Mmpuacglcdc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 000000000000. The write value should be 000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xfff, 1, 0, u16, Mmpuacglcdc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xfff,1,0,u16, Mmpuacglcdc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuacglcdc {
    #[inline(always)]
    fn default() -> Mmpuacglcdc {
        <crate::RegValueT<Mmpuacglcdc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuacglcdc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "GLCDC region n unit is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "GLCDC region n unit is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rp_SPEC;
    pub type Rp = crate::EnumBitfieldStruct<u8, Rp_SPEC>;
    impl Rp {
        #[doc = "Read permission"]
        pub const _0: Self = Self::new(0);
        #[doc = "Read protection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wp_SPEC;
    pub type Wp = crate::EnumBitfieldStruct<u8, Wp_SPEC>;
    impl Wp {
        #[doc = "Write permission"]
        pub const _0: Self = Self::new(0);
        #[doc = "Write protection"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpusglcdc_SPEC;
impl crate::sealed::RegSpec for Mmpusglcdc_SPEC {
    type DataType = u32;
}
#[doc = "MMPU Start Address Register for GLCDC %s"]
pub type Mmpusglcdc = crate::RegValueT<Mmpusglcdc_SPEC>;

impl Mmpusglcdc {
    #[doc = "Region Stat Address : Address where the region starts, for use in region determination.NOTE: The low-order 10 bits are fixed to 0."]
    #[inline(always)]
    pub fn mmpus(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Mmpusglcdc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Mmpusglcdc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpusglcdc {
    #[inline(always)]
    fn default() -> Mmpusglcdc {
        <crate::RegValueT<Mmpusglcdc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpueglcdc_SPEC;
impl crate::sealed::RegSpec for Mmpueglcdc_SPEC {
    type DataType = u32;
}
#[doc = "Graphics Region %s End Address Register"]
pub type Mmpueglcdc = crate::RegValueT<Mmpueglcdc_SPEC>;

impl Mmpueglcdc {
    #[doc = "Region End Address : Address where the region end, for use in region determination.NOTE: The low-order 10 bits are fixed to 1."]
    #[inline(always)]
    pub fn mmpue(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Mmpueglcdc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Mmpueglcdc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpueglcdc {
    #[inline(always)]
    fn default() -> Mmpueglcdc {
        <crate::RegValueT<Mmpueglcdc_SPEC> as RegisterValue<_>>::new(1023)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuendrw_SPEC;
impl crate::sealed::RegSpec for Mmpuendrw_SPEC {
    type DataType = u16;
}
#[doc = "MMPU Enable Register for DRW"]
pub type Mmpuendrw = crate::RegValueT<Mmpuendrw_SPEC>;

impl Mmpuendrw {
    #[doc = "Bus master MPU of DRW Enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuendrw::Enable,
        Mmpuendrw_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuendrw::Enable,
            Mmpuendrw_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, Mmpuendrw_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8, Mmpuendrw_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Mmpuendrw_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Mmpuendrw_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuendrw {
    #[inline(always)]
    fn default() -> Mmpuendrw {
        <crate::RegValueT<Mmpuendrw_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuendrw {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "Bus master MPU of DRW is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Bus master MPU of DRW is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuenpdrw_SPEC;
impl crate::sealed::RegSpec for Mmpuenpdrw_SPEC {
    type DataType = u16;
}
#[doc = "MMPU Enable Protect Register for DRW"]
pub type Mmpuenpdrw = crate::RegValueT<Mmpuenpdrw_SPEC>;

impl Mmpuenpdrw {
    #[doc = "Protection of register"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuenpdrw::Protect,
        Mmpuenpdrw_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuenpdrw::Protect,
            Mmpuenpdrw_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, Mmpuenpdrw_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8, Mmpuenpdrw_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Mmpuenpdrw_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Mmpuenpdrw_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuenpdrw {
    #[inline(always)]
    fn default() -> Mmpuenpdrw {
        <crate::RegValueT<Mmpuenpdrw_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuenpdrw {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "MMPUENDRW register write is possible"]
        pub const _0: Self = Self::new(0);
        #[doc = "MMPUENDRW register write is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpurptdrw_SPEC;
impl crate::sealed::RegSpec for Mmpurptdrw_SPEC {
    type DataType = u16;
}
#[doc = "MMPU Regions Protect Register for DRW"]
pub type Mmpurptdrw = crate::RegValueT<Mmpurptdrw_SPEC>;

impl Mmpurptdrw {
    #[doc = "Protection of register"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpurptdrw::Protect,
        Mmpurptdrw_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpurptdrw::Protect,
            Mmpurptdrw_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, Mmpurptdrw_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8, Mmpurptdrw_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Mmpurptdrw_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Mmpurptdrw_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpurptdrw {
    #[inline(always)]
    fn default() -> Mmpurptdrw {
        <crate::RegValueT<Mmpurptdrw_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpurptdrw {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "Bus master MPU register for DRW write is possible"]
        pub const _0: Self = Self::new(0);
        #[doc = "Bus master MPU register for DRW write is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuacdrw_SPEC;
impl crate::sealed::RegSpec for Mmpuacdrw_SPEC {
    type DataType = u16;
}
#[doc = "MMPU Access Control Register for DRW %s"]
pub type Mmpuacdrw = crate::RegValueT<Mmpuacdrw_SPEC>;

impl Mmpuacdrw {
    #[doc = "Region Enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuacdrw::Enable,
        Mmpuacdrw_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuacdrw::Enable,
            Mmpuacdrw_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Read protection"]
    #[inline(always)]
    pub fn rp(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, mmpuacdrw::Rp, Mmpuacdrw_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,mmpuacdrw::Rp, Mmpuacdrw_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn wp(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, mmpuacdrw::Wp, Mmpuacdrw_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,mmpuacdrw::Wp, Mmpuacdrw_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 000000000000. The write value should be 000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xfff, 1, 0, u16, Mmpuacdrw_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xfff,1,0,u16, Mmpuacdrw_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuacdrw {
    #[inline(always)]
    fn default() -> Mmpuacdrw {
        <crate::RegValueT<Mmpuacdrw_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuacdrw {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "DRW region n unit is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "DRW region n unit is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rp_SPEC;
    pub type Rp = crate::EnumBitfieldStruct<u8, Rp_SPEC>;
    impl Rp {
        #[doc = "Read permission"]
        pub const _0: Self = Self::new(0);
        #[doc = "Read protection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wp_SPEC;
    pub type Wp = crate::EnumBitfieldStruct<u8, Wp_SPEC>;
    impl Wp {
        #[doc = "Write permission"]
        pub const _0: Self = Self::new(0);
        #[doc = "Write protection"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpusdrw_SPEC;
impl crate::sealed::RegSpec for Mmpusdrw_SPEC {
    type DataType = u32;
}
#[doc = "MMPU Start Address Register for DRW %s"]
pub type Mmpusdrw = crate::RegValueT<Mmpusdrw_SPEC>;

impl Mmpusdrw {
    #[doc = "Region Stat Address : Address where the region starts, for use in region determination.NOTE: The low-order 10 bits are fixed to 0."]
    #[inline(always)]
    pub fn mmpus(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Mmpusdrw_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Mmpusdrw_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpusdrw {
    #[inline(always)]
    fn default() -> Mmpusdrw {
        <crate::RegValueT<Mmpusdrw_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuedrw_SPEC;
impl crate::sealed::RegSpec for Mmpuedrw_SPEC {
    type DataType = u32;
}
#[doc = "Graphics Region %s End Address Register"]
pub type Mmpuedrw = crate::RegValueT<Mmpuedrw_SPEC>;

impl Mmpuedrw {
    #[doc = "Region End Address : Address where the region end, for use in region determination.NOTE: The low-order 10 bits are fixed to 1."]
    #[inline(always)]
    pub fn mmpue(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Mmpuedrw_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Mmpuedrw_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuedrw {
    #[inline(always)]
    fn default() -> Mmpuedrw {
        <crate::RegValueT<Mmpuedrw_SPEC> as RegisterValue<_>>::new(1023)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuenmipi_SPEC;
impl crate::sealed::RegSpec for Mmpuenmipi_SPEC {
    type DataType = u16;
}
#[doc = "MMPU Enable Register for MIPI"]
pub type Mmpuenmipi = crate::RegValueT<Mmpuenmipi_SPEC>;

impl Mmpuenmipi {
    #[doc = "Bus master MPU of MIPI Enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuenmipi::Enable,
        Mmpuenmipi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuenmipi::Enable,
            Mmpuenmipi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, Mmpuenmipi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8, Mmpuenmipi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Mmpuenmipi_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Mmpuenmipi_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuenmipi {
    #[inline(always)]
    fn default() -> Mmpuenmipi {
        <crate::RegValueT<Mmpuenmipi_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuenmipi {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "Bus master MPU of MIPI is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Bus master MPU of MIPI is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuenptmipi_SPEC;
impl crate::sealed::RegSpec for Mmpuenptmipi_SPEC {
    type DataType = u16;
}
#[doc = "MMPU Enable Protect Register for MIPI"]
pub type Mmpuenptmipi = crate::RegValueT<Mmpuenptmipi_SPEC>;

impl Mmpuenptmipi {
    #[doc = "Protection of register"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuenptmipi::Protect,
        Mmpuenptmipi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuenptmipi::Protect,
            Mmpuenptmipi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, Mmpuenptmipi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8, Mmpuenptmipi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Mmpuenptmipi_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Mmpuenptmipi_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuenptmipi {
    #[inline(always)]
    fn default() -> Mmpuenptmipi {
        <crate::RegValueT<Mmpuenptmipi_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuenptmipi {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "MMPUENMIPI register write is possible"]
        pub const _0: Self = Self::new(0);
        #[doc = "MMPUENMIPI register write is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpurptmipi_SPEC;
impl crate::sealed::RegSpec for Mmpurptmipi_SPEC {
    type DataType = u16;
}
#[doc = "MMPU Regions Protect Register for MIPI"]
pub type Mmpurptmipi = crate::RegValueT<Mmpurptmipi_SPEC>;

impl Mmpurptmipi {
    #[doc = "Protection of register"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpurptmipi::Protect,
        Mmpurptmipi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpurptmipi::Protect,
            Mmpurptmipi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, Mmpurptmipi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8, Mmpurptmipi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Mmpurptmipi_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Mmpurptmipi_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpurptmipi {
    #[inline(always)]
    fn default() -> Mmpurptmipi {
        <crate::RegValueT<Mmpurptmipi_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpurptmipi {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "Bus master MPU register for MIPI write is possible"]
        pub const _0: Self = Self::new(0);
        #[doc = "Bus master MPU register for MIPI write is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuacmipi_SPEC;
impl crate::sealed::RegSpec for Mmpuacmipi_SPEC {
    type DataType = u16;
}
#[doc = "MMPU Access Control Register for MIPI"]
pub type Mmpuacmipi = crate::RegValueT<Mmpuacmipi_SPEC>;

impl Mmpuacmipi {
    #[doc = "Region Enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuacmipi::Enable,
        Mmpuacmipi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuacmipi::Enable,
            Mmpuacmipi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Read protection"]
    #[inline(always)]
    pub fn rp(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mmpuacmipi::Rp,
        Mmpuacmipi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mmpuacmipi::Rp,
            Mmpuacmipi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn wp(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mmpuacmipi::Wp,
        Mmpuacmipi_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mmpuacmipi::Wp,
            Mmpuacmipi_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 000000000000. The write value should be 000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xfff, 1, 0, u16, Mmpuacmipi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xfff,1,0,u16, Mmpuacmipi_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuacmipi {
    #[inline(always)]
    fn default() -> Mmpuacmipi {
        <crate::RegValueT<Mmpuacmipi_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuacmipi {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "MIPI region n unit is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "MIPI region n unit is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rp_SPEC;
    pub type Rp = crate::EnumBitfieldStruct<u8, Rp_SPEC>;
    impl Rp {
        #[doc = "Read permission"]
        pub const _0: Self = Self::new(0);
        #[doc = "Read protection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wp_SPEC;
    pub type Wp = crate::EnumBitfieldStruct<u8, Wp_SPEC>;
    impl Wp {
        #[doc = "Write permission"]
        pub const _0: Self = Self::new(0);
        #[doc = "Write protection"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpusmipi_SPEC;
impl crate::sealed::RegSpec for Mmpusmipi_SPEC {
    type DataType = u32;
}
#[doc = "MMPU Start Address Register for MIPI"]
pub type Mmpusmipi = crate::RegValueT<Mmpusmipi_SPEC>;

impl Mmpusmipi {
    #[doc = "Region Stat Address : Address where the region starts, for use in region determination.NOTE: The low-order 12 bits are fixed to 0."]
    #[inline(always)]
    pub fn mmpus(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Mmpusmipi_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Mmpusmipi_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpusmipi {
    #[inline(always)]
    fn default() -> Mmpusmipi {
        <crate::RegValueT<Mmpusmipi_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuemipi_SPEC;
impl crate::sealed::RegSpec for Mmpuemipi_SPEC {
    type DataType = u32;
}
#[doc = "MMPU End Address Register for MIPI"]
pub type Mmpuemipi = crate::RegValueT<Mmpuemipi_SPEC>;

impl Mmpuemipi {
    #[doc = "Region End Address : Address where the region end, for use in region determination.NOTE: The low-order 12 bits are fixed to 1."]
    #[inline(always)]
    pub fn mmpue(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Mmpuemipi_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Mmpuemipi_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuemipi {
    #[inline(always)]
    fn default() -> Mmpuemipi {
        <crate::RegValueT<Mmpuemipi_SPEC> as RegisterValue<_>>::new(1023)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuenceu_SPEC;
impl crate::sealed::RegSpec for Mmpuenceu_SPEC {
    type DataType = u16;
}
#[doc = "MMPU Enable Register for CEU"]
pub type Mmpuenceu = crate::RegValueT<Mmpuenceu_SPEC>;

impl Mmpuenceu {
    #[doc = "Bus master MPU of CEU Enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuenceu::Enable,
        Mmpuenceu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuenceu::Enable,
            Mmpuenceu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, Mmpuenceu_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8, Mmpuenceu_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Mmpuenceu_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Mmpuenceu_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuenceu {
    #[inline(always)]
    fn default() -> Mmpuenceu {
        <crate::RegValueT<Mmpuenceu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuenceu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "Bus master MPU of CEU is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Bus master MPU of CEU is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuenptceu_SPEC;
impl crate::sealed::RegSpec for Mmpuenptceu_SPEC {
    type DataType = u16;
}
#[doc = "MMPU Enable Protect Register for CEU"]
pub type Mmpuenptceu = crate::RegValueT<Mmpuenptceu_SPEC>;

impl Mmpuenptceu {
    #[doc = "Protection of register"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuenptceu::Protect,
        Mmpuenptceu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuenptceu::Protect,
            Mmpuenptceu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, Mmpuenptceu_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8, Mmpuenptceu_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Mmpuenptceu_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Mmpuenptceu_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuenptceu {
    #[inline(always)]
    fn default() -> Mmpuenptceu {
        <crate::RegValueT<Mmpuenptceu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuenptceu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "MMPUENCEU register write is possible"]
        pub const _0: Self = Self::new(0);
        #[doc = "MMPUENCEU register write is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpurptceu_SPEC;
impl crate::sealed::RegSpec for Mmpurptceu_SPEC {
    type DataType = u16;
}
#[doc = "MMPU Regions Protect Register for CEU"]
pub type Mmpurptceu = crate::RegValueT<Mmpurptceu_SPEC>;

impl Mmpurptceu {
    #[doc = "Protection of register"]
    #[inline(always)]
    pub fn protect(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpurptceu::Protect,
        Mmpurptceu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpurptceu::Protect,
            Mmpurptceu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, Mmpurptceu_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8, Mmpurptceu_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Key Code"]
    #[inline(always)]
    pub fn key(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Mmpurptceu_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Mmpurptceu_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpurptceu {
    #[inline(always)]
    fn default() -> Mmpurptceu {
        <crate::RegValueT<Mmpurptceu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpurptceu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Protect_SPEC;
    pub type Protect = crate::EnumBitfieldStruct<u8, Protect_SPEC>;
    impl Protect {
        #[doc = "Bus master MPU register for CEU write is possible"]
        pub const _0: Self = Self::new(0);
        #[doc = "Bus master MPU register for CEU write is protected. Read is possible."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpuacceu_SPEC;
impl crate::sealed::RegSpec for Mmpuacceu_SPEC {
    type DataType = u16;
}
#[doc = "MMPU Access Control Register for CEU %s"]
pub type Mmpuacceu = crate::RegValueT<Mmpuacceu_SPEC>;

impl Mmpuacceu {
    #[doc = "Region Enable"]
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mmpuacceu::Enable,
        Mmpuacceu_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mmpuacceu::Enable,
            Mmpuacceu_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Read protection"]
    #[inline(always)]
    pub fn rp(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, mmpuacceu::Rp, Mmpuacceu_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,mmpuacceu::Rp, Mmpuacceu_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Write protection"]
    #[inline(always)]
    pub fn wp(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, mmpuacceu::Wp, Mmpuacceu_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,mmpuacceu::Wp, Mmpuacceu_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 000000000000. The write value should be 000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xfff, 1, 0, u16, Mmpuacceu_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xfff,1,0,u16, Mmpuacceu_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpuacceu {
    #[inline(always)]
    fn default() -> Mmpuacceu {
        <crate::RegValueT<Mmpuacceu_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mmpuacceu {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Enable_SPEC;
    pub type Enable = crate::EnumBitfieldStruct<u8, Enable_SPEC>;
    impl Enable {
        #[doc = "CEU region n unit is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "CEU region n unit is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rp_SPEC;
    pub type Rp = crate::EnumBitfieldStruct<u8, Rp_SPEC>;
    impl Rp {
        #[doc = "Read permission"]
        pub const _0: Self = Self::new(0);
        #[doc = "Read protection"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wp_SPEC;
    pub type Wp = crate::EnumBitfieldStruct<u8, Wp_SPEC>;
    impl Wp {
        #[doc = "Write permission"]
        pub const _0: Self = Self::new(0);
        #[doc = "Write protection"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpusceu_SPEC;
impl crate::sealed::RegSpec for Mmpusceu_SPEC {
    type DataType = u32;
}
#[doc = "MMPU Start Address Register for CEU %s"]
pub type Mmpusceu = crate::RegValueT<Mmpusceu_SPEC>;

impl Mmpusceu {
    #[doc = "Region Stat Address : Address where the region starts, for use in region determination.NOTE: The low-order 12 bits are fixed to 0."]
    #[inline(always)]
    pub fn mmpus(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Mmpusceu_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Mmpusceu_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpusceu {
    #[inline(always)]
    fn default() -> Mmpusceu {
        <crate::RegValueT<Mmpusceu_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mmpueceu_SPEC;
impl crate::sealed::RegSpec for Mmpueceu_SPEC {
    type DataType = u32;
}
#[doc = "MMPU End Address Register for CEU %s"]
pub type Mmpueceu = crate::RegValueT<Mmpueceu_SPEC>;

impl Mmpueceu {
    #[doc = "Region End Address : Address where the region end, for use in region determination.NOTE: The low-order 12 bits are fixed to 1."]
    #[inline(always)]
    pub fn mmpue(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Mmpueceu_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Mmpueceu_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mmpueceu {
    #[inline(always)]
    fn default() -> Mmpueceu {
        <crate::RegValueT<Mmpueceu_SPEC> as RegisterValue<_>>::new(1023)
    }
}
