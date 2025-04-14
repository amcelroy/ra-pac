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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:18:26 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"CAN0 Module"]
unsafe impl ::core::marker::Send for super::Can0 {}
unsafe impl ::core::marker::Sync for super::Can0 {}
impl super::Can0 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn mb_id(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::MbId_SPEC, crate::common::RW>,
        32,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x200usize))
        }
    }

    #[inline(always)]
    pub const fn mb_dl(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::MbDl_SPEC, crate::common::RW>,
        32,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x204usize))
        }
    }

    #[inline(always)]
    pub const fn mb_d0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::MbD0_SPEC, crate::common::RW>,
        32,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x206usize))
        }
    }

    #[inline(always)]
    pub const fn mb_d1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::MbD1_SPEC, crate::common::RW>,
        32,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x207usize))
        }
    }

    #[inline(always)]
    pub const fn mb_d2(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::MbD2_SPEC, crate::common::RW>,
        32,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x208usize))
        }
    }

    #[inline(always)]
    pub const fn mb_d3(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::MbD3_SPEC, crate::common::RW>,
        32,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x209usize))
        }
    }

    #[inline(always)]
    pub const fn mb_d4(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::MbD4_SPEC, crate::common::RW>,
        32,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x20ausize))
        }
    }

    #[inline(always)]
    pub const fn mb_d5(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::MbD5_SPEC, crate::common::RW>,
        32,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x20busize))
        }
    }

    #[inline(always)]
    pub const fn mb_d6(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::MbD6_SPEC, crate::common::RW>,
        32,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x20cusize))
        }
    }

    #[inline(always)]
    pub const fn mb_d7(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::MbD7_SPEC, crate::common::RW>,
        32,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x20dusize))
        }
    }

    #[inline(always)]
    pub const fn mb_ts(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::MbTs_SPEC, crate::common::RW>,
        32,
        0x10,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x20eusize))
        }
    }

    #[inline(always)]
    pub const fn mkr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Mkr_SPEC, crate::common::RW>,
        8,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x400usize))
        }
    }

    #[inline(always)]
    pub const fn fidcr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Fidcr_SPEC, crate::common::RW>,
        2,
        0x4,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x420usize))
        }
    }

    #[inline(always)]
    pub const fn mkivlr(
        &self,
    ) -> &'static crate::common::Reg<self::Mkivlr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mkivlr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1064usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mier(&self) -> &'static crate::common::Reg<self::Mier_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mier_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1068usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mier_fifo(
        &self,
    ) -> &'static crate::common::Reg<self::MierFifo_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::MierFifo_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1068usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mctl_tx(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::MctlTx_SPEC, crate::common::RW>,
        32,
        0x1,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x820usize))
        }
    }

    #[inline(always)]
    pub const fn mctl_rx(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::MctlRx_SPEC, crate::common::RW>,
        32,
        0x1,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x820usize))
        }
    }

    #[inline(always)]
    pub const fn ctlr(&self) -> &'static crate::common::Reg<self::Ctlr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctlr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2112usize),
            )
        }
    }

    #[inline(always)]
    pub const fn str(&self) -> &'static crate::common::Reg<self::Str_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Str_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2114usize),
            )
        }
    }

    #[inline(always)]
    pub const fn bcr(&self) -> &'static crate::common::Reg<self::Bcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2116usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rfcr(&self) -> &'static crate::common::Reg<self::Rfcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rfcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2120usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rfpcr(&self) -> &'static crate::common::Reg<self::Rfpcr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Rfpcr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(2121usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tfcr(&self) -> &'static crate::common::Reg<self::Tfcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tfcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2122usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tfpcr(&self) -> &'static crate::common::Reg<self::Tfpcr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Tfpcr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(2123usize),
            )
        }
    }

    #[inline(always)]
    pub const fn eier(&self) -> &'static crate::common::Reg<self::Eier_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eier_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2124usize),
            )
        }
    }

    #[inline(always)]
    pub const fn eifr(&self) -> &'static crate::common::Reg<self::Eifr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eifr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2125usize),
            )
        }
    }

    #[inline(always)]
    pub const fn recr(&self) -> &'static crate::common::Reg<self::Recr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Recr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2126usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tecr(&self) -> &'static crate::common::Reg<self::Tecr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Tecr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2127usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ecsr(&self) -> &'static crate::common::Reg<self::Ecsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ecsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2128usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cssr(&self) -> &'static crate::common::Reg<self::Cssr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cssr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2129usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mssr(&self) -> &'static crate::common::Reg<self::Mssr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mssr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2130usize),
            )
        }
    }

    #[inline(always)]
    pub const fn msmr(&self) -> &'static crate::common::Reg<self::Msmr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Msmr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2131usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tsr(&self) -> &'static crate::common::Reg<self::Tsr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Tsr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2132usize),
            )
        }
    }

    #[inline(always)]
    pub const fn afsr(&self) -> &'static crate::common::Reg<self::Afsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Afsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2134usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tcr(&self) -> &'static crate::common::Reg<self::Tcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2136usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbId_SPEC;
impl crate::sealed::RegSpec for MbId_SPEC {
    type DataType = u32;
}

pub type MbId = crate::RegValueT<MbId_SPEC>;

impl MbId {
    #[inline(always)]
    pub fn ide(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        mb_id::Ide,
        mb_id::Ide,
        MbId_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            mb_id::Ide,
            mb_id::Ide,
            MbId_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rtr(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        mb_id::Rtr,
        mb_id::Rtr,
        MbId_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            mb_id::Rtr,
            mb_id::Rtr,
            MbId_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, MbId_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, MbId_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn sid(
        self,
    ) -> crate::common::RegisterField<18, 0x7ff, 1, 0, u16, u16, MbId_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x7ff,1,0,u16,u16,MbId_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn eid(
        self,
    ) -> crate::common::RegisterField<0, 0x3ffff, 1, 0, u32, u32, MbId_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ffff,1,0,u32,u32,MbId_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for MbId {
    #[inline(always)]
    fn default() -> MbId {
        <crate::RegValueT<MbId_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mb_id {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ide_SPEC;
    pub type Ide = crate::EnumBitfieldStruct<u8, Ide_SPEC>;
    impl Ide {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtr_SPEC;
    pub type Rtr = crate::EnumBitfieldStruct<u8, Rtr_SPEC>;
    impl Rtr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbDl_SPEC;
impl crate::sealed::RegSpec for MbDl_SPEC {
    type DataType = u16;
}

pub type MbDl = crate::RegValueT<MbDl_SPEC>;

impl MbDl {
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xfff, 1, 0, u16, u16, MbDl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xfff,1,0,u16,u16,MbDl_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dlc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        mb_dl::Dlc,
        mb_dl::Dlc,
        MbDl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            mb_dl::Dlc,
            mb_dl::Dlc,
            MbDl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for MbDl {
    #[inline(always)]
    fn default() -> MbDl {
        <crate::RegValueT<MbDl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mb_dl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dlc_SPEC;
    pub type Dlc = crate::EnumBitfieldStruct<u8, Dlc_SPEC>;
    impl Dlc {
        pub const _0000: Self = Self::new(0);

        pub const _0001: Self = Self::new(1);

        pub const _0010: Self = Self::new(2);

        pub const _0011: Self = Self::new(3);

        pub const _0100: Self = Self::new(4);

        pub const _0101: Self = Self::new(5);

        pub const _0110: Self = Self::new(6);

        pub const _0111: Self = Self::new(7);

        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbD0_SPEC;
impl crate::sealed::RegSpec for MbD0_SPEC {
    type DataType = u8;
}

pub type MbD0 = crate::RegValueT<MbD0_SPEC>;

impl MbD0 {
    #[inline(always)]
    pub fn data0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, MbD0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,MbD0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for MbD0 {
    #[inline(always)]
    fn default() -> MbD0 {
        <crate::RegValueT<MbD0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbD1_SPEC;
impl crate::sealed::RegSpec for MbD1_SPEC {
    type DataType = u8;
}

pub type MbD1 = crate::RegValueT<MbD1_SPEC>;

impl MbD1 {
    #[inline(always)]
    pub fn data1(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, MbD1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,MbD1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for MbD1 {
    #[inline(always)]
    fn default() -> MbD1 {
        <crate::RegValueT<MbD1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbD2_SPEC;
impl crate::sealed::RegSpec for MbD2_SPEC {
    type DataType = u8;
}

pub type MbD2 = crate::RegValueT<MbD2_SPEC>;

impl MbD2 {
    #[inline(always)]
    pub fn data2(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, MbD2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,MbD2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for MbD2 {
    #[inline(always)]
    fn default() -> MbD2 {
        <crate::RegValueT<MbD2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbD3_SPEC;
impl crate::sealed::RegSpec for MbD3_SPEC {
    type DataType = u8;
}

pub type MbD3 = crate::RegValueT<MbD3_SPEC>;

impl MbD3 {
    #[inline(always)]
    pub fn data3(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, MbD3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,MbD3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for MbD3 {
    #[inline(always)]
    fn default() -> MbD3 {
        <crate::RegValueT<MbD3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbD4_SPEC;
impl crate::sealed::RegSpec for MbD4_SPEC {
    type DataType = u8;
}

pub type MbD4 = crate::RegValueT<MbD4_SPEC>;

impl MbD4 {
    #[inline(always)]
    pub fn data4(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, MbD4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,MbD4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for MbD4 {
    #[inline(always)]
    fn default() -> MbD4 {
        <crate::RegValueT<MbD4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbD5_SPEC;
impl crate::sealed::RegSpec for MbD5_SPEC {
    type DataType = u8;
}

pub type MbD5 = crate::RegValueT<MbD5_SPEC>;

impl MbD5 {
    #[inline(always)]
    pub fn data5(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, MbD5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,MbD5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for MbD5 {
    #[inline(always)]
    fn default() -> MbD5 {
        <crate::RegValueT<MbD5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbD6_SPEC;
impl crate::sealed::RegSpec for MbD6_SPEC {
    type DataType = u8;
}

pub type MbD6 = crate::RegValueT<MbD6_SPEC>;

impl MbD6 {
    #[inline(always)]
    pub fn data6(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, MbD6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,MbD6_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for MbD6 {
    #[inline(always)]
    fn default() -> MbD6 {
        <crate::RegValueT<MbD6_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbD7_SPEC;
impl crate::sealed::RegSpec for MbD7_SPEC {
    type DataType = u8;
}

pub type MbD7 = crate::RegValueT<MbD7_SPEC>;

impl MbD7 {
    #[inline(always)]
    pub fn data7(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, MbD7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,MbD7_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for MbD7 {
    #[inline(always)]
    fn default() -> MbD7 {
        <crate::RegValueT<MbD7_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbTs_SPEC;
impl crate::sealed::RegSpec for MbTs_SPEC {
    type DataType = u16;
}

pub type MbTs = crate::RegValueT<MbTs_SPEC>;

impl MbTs {
    #[inline(always)]
    pub fn tsh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, MbTs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,MbTs_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tsl(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, MbTs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,MbTs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for MbTs {
    #[inline(always)]
    fn default() -> MbTs {
        <crate::RegValueT<MbTs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mkr_SPEC;
impl crate::sealed::RegSpec for Mkr_SPEC {
    type DataType = u32;
}

pub type Mkr = crate::RegValueT<Mkr_SPEC>;

impl Mkr {
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<29, 0x7, 1, 0, u8, u8, Mkr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x7,1,0,u8,u8,Mkr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sid(
        self,
    ) -> crate::common::RegisterField<18, 0x7ff, 1, 0, u16, u16, Mkr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x7ff,1,0,u16,u16,Mkr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn eid(
        self,
    ) -> crate::common::RegisterField<0, 0x3ffff, 1, 0, u32, u32, Mkr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ffff,1,0,u32,u32,Mkr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mkr {
    #[inline(always)]
    fn default() -> Mkr {
        <crate::RegValueT<Mkr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fidcr_SPEC;
impl crate::sealed::RegSpec for Fidcr_SPEC {
    type DataType = u32;
}

pub type Fidcr = crate::RegValueT<Fidcr_SPEC>;

impl Fidcr {
    #[inline(always)]
    pub fn ide(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        fidcr::Ide,
        fidcr::Ide,
        Fidcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            fidcr::Ide,
            fidcr::Ide,
            Fidcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rtr(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        fidcr::Rtr,
        fidcr::Rtr,
        Fidcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            fidcr::Rtr,
            fidcr::Rtr,
            Fidcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Fidcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Fidcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn sid(
        self,
    ) -> crate::common::RegisterField<18, 0x7ff, 1, 0, u16, u16, Fidcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x7ff,1,0,u16,u16,Fidcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn eid(
        self,
    ) -> crate::common::RegisterField<0, 0x3ffff, 1, 0, u32, u32, Fidcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ffff,1,0,u32,u32,Fidcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fidcr {
    #[inline(always)]
    fn default() -> Fidcr {
        <crate::RegValueT<Fidcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fidcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ide_SPEC;
    pub type Ide = crate::EnumBitfieldStruct<u8, Ide_SPEC>;
    impl Ide {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtr_SPEC;
    pub type Rtr = crate::EnumBitfieldStruct<u8, Rtr_SPEC>;
    impl Rtr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mkivlr_SPEC;
impl crate::sealed::RegSpec for Mkivlr_SPEC {
    type DataType = u32;
}

pub type Mkivlr = crate::RegValueT<Mkivlr_SPEC>;

impl Mkivlr {
    #[inline(always)]
    pub fn mb31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        mkivlr::Mb31,
        mkivlr::Mb31,
        Mkivlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            mkivlr::Mb31,
            mkivlr::Mb31,
            Mkivlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb30(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        mkivlr::Mb30,
        mkivlr::Mb30,
        Mkivlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            mkivlr::Mb30,
            mkivlr::Mb30,
            Mkivlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb29(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        mkivlr::Mb29,
        mkivlr::Mb29,
        Mkivlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            mkivlr::Mb29,
            mkivlr::Mb29,
            Mkivlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        mkivlr::Mb28,
        mkivlr::Mb28,
        Mkivlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            mkivlr::Mb28,
            mkivlr::Mb28,
            Mkivlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        mkivlr::Mb27,
        mkivlr::Mb27,
        Mkivlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            mkivlr::Mb27,
            mkivlr::Mb27,
            Mkivlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb26(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        mkivlr::Mb26,
        mkivlr::Mb26,
        Mkivlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            mkivlr::Mb26,
            mkivlr::Mb26,
            Mkivlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb25(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        mkivlr::Mb25,
        mkivlr::Mb25,
        Mkivlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            mkivlr::Mb25,
            mkivlr::Mb25,
            Mkivlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb24(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        mkivlr::Mb24,
        mkivlr::Mb24,
        Mkivlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            mkivlr::Mb24,
            mkivlr::Mb24,
            Mkivlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        mkivlr::Mb23,
        mkivlr::Mb23,
        Mkivlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            mkivlr::Mb23,
            mkivlr::Mb23,
            Mkivlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        mkivlr::Mb22,
        mkivlr::Mb22,
        Mkivlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            mkivlr::Mb22,
            mkivlr::Mb22,
            Mkivlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb21(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        mkivlr::Mb21,
        mkivlr::Mb21,
        Mkivlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            mkivlr::Mb21,
            mkivlr::Mb21,
            Mkivlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb20(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        mkivlr::Mb20,
        mkivlr::Mb20,
        Mkivlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            mkivlr::Mb20,
            mkivlr::Mb20,
            Mkivlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        mkivlr::Mb19,
        mkivlr::Mb19,
        Mkivlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            mkivlr::Mb19,
            mkivlr::Mb19,
            Mkivlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        mkivlr::Mb18,
        mkivlr::Mb18,
        Mkivlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            mkivlr::Mb18,
            mkivlr::Mb18,
            Mkivlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        mkivlr::Mb17,
        mkivlr::Mb17,
        Mkivlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            mkivlr::Mb17,
            mkivlr::Mb17,
            Mkivlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        mkivlr::Mb16,
        mkivlr::Mb16,
        Mkivlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            mkivlr::Mb16,
            mkivlr::Mb16,
            Mkivlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        mkivlr::Mb15,
        mkivlr::Mb15,
        Mkivlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            mkivlr::Mb15,
            mkivlr::Mb15,
            Mkivlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        mkivlr::Mb14,
        mkivlr::Mb14,
        Mkivlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            mkivlr::Mb14,
            mkivlr::Mb14,
            Mkivlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        mkivlr::Mb13,
        mkivlr::Mb13,
        Mkivlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            mkivlr::Mb13,
            mkivlr::Mb13,
            Mkivlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        mkivlr::Mb12,
        mkivlr::Mb12,
        Mkivlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            mkivlr::Mb12,
            mkivlr::Mb12,
            Mkivlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        mkivlr::Mb11,
        mkivlr::Mb11,
        Mkivlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            mkivlr::Mb11,
            mkivlr::Mb11,
            Mkivlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        mkivlr::Mb10,
        mkivlr::Mb10,
        Mkivlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            mkivlr::Mb10,
            mkivlr::Mb10,
            Mkivlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        mkivlr::Mb9,
        mkivlr::Mb9,
        Mkivlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            mkivlr::Mb9,
            mkivlr::Mb9,
            Mkivlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        mkivlr::Mb8,
        mkivlr::Mb8,
        Mkivlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            mkivlr::Mb8,
            mkivlr::Mb8,
            Mkivlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        mkivlr::Mb7,
        mkivlr::Mb7,
        Mkivlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            mkivlr::Mb7,
            mkivlr::Mb7,
            Mkivlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        mkivlr::Mb6,
        mkivlr::Mb6,
        Mkivlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            mkivlr::Mb6,
            mkivlr::Mb6,
            Mkivlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        mkivlr::Mb5,
        mkivlr::Mb5,
        Mkivlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            mkivlr::Mb5,
            mkivlr::Mb5,
            Mkivlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        mkivlr::Mb4,
        mkivlr::Mb4,
        Mkivlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            mkivlr::Mb4,
            mkivlr::Mb4,
            Mkivlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        mkivlr::Mb3,
        mkivlr::Mb3,
        Mkivlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            mkivlr::Mb3,
            mkivlr::Mb3,
            Mkivlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mkivlr::Mb2,
        mkivlr::Mb2,
        Mkivlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mkivlr::Mb2,
            mkivlr::Mb2,
            Mkivlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mkivlr::Mb1,
        mkivlr::Mb1,
        Mkivlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mkivlr::Mb1,
            mkivlr::Mb1,
            Mkivlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mkivlr::Mb0,
        mkivlr::Mb0,
        Mkivlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mkivlr::Mb0,
            mkivlr::Mb0,
            Mkivlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mkivlr {
    #[inline(always)]
    fn default() -> Mkivlr {
        <crate::RegValueT<Mkivlr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mkivlr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb31_SPEC;
    pub type Mb31 = crate::EnumBitfieldStruct<u8, Mb31_SPEC>;
    impl Mb31 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb30_SPEC;
    pub type Mb30 = crate::EnumBitfieldStruct<u8, Mb30_SPEC>;
    impl Mb30 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb29_SPEC;
    pub type Mb29 = crate::EnumBitfieldStruct<u8, Mb29_SPEC>;
    impl Mb29 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb28_SPEC;
    pub type Mb28 = crate::EnumBitfieldStruct<u8, Mb28_SPEC>;
    impl Mb28 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb27_SPEC;
    pub type Mb27 = crate::EnumBitfieldStruct<u8, Mb27_SPEC>;
    impl Mb27 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb26_SPEC;
    pub type Mb26 = crate::EnumBitfieldStruct<u8, Mb26_SPEC>;
    impl Mb26 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb25_SPEC;
    pub type Mb25 = crate::EnumBitfieldStruct<u8, Mb25_SPEC>;
    impl Mb25 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb24_SPEC;
    pub type Mb24 = crate::EnumBitfieldStruct<u8, Mb24_SPEC>;
    impl Mb24 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb23_SPEC;
    pub type Mb23 = crate::EnumBitfieldStruct<u8, Mb23_SPEC>;
    impl Mb23 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb22_SPEC;
    pub type Mb22 = crate::EnumBitfieldStruct<u8, Mb22_SPEC>;
    impl Mb22 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb21_SPEC;
    pub type Mb21 = crate::EnumBitfieldStruct<u8, Mb21_SPEC>;
    impl Mb21 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb20_SPEC;
    pub type Mb20 = crate::EnumBitfieldStruct<u8, Mb20_SPEC>;
    impl Mb20 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb19_SPEC;
    pub type Mb19 = crate::EnumBitfieldStruct<u8, Mb19_SPEC>;
    impl Mb19 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb18_SPEC;
    pub type Mb18 = crate::EnumBitfieldStruct<u8, Mb18_SPEC>;
    impl Mb18 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb17_SPEC;
    pub type Mb17 = crate::EnumBitfieldStruct<u8, Mb17_SPEC>;
    impl Mb17 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb16_SPEC;
    pub type Mb16 = crate::EnumBitfieldStruct<u8, Mb16_SPEC>;
    impl Mb16 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb15_SPEC;
    pub type Mb15 = crate::EnumBitfieldStruct<u8, Mb15_SPEC>;
    impl Mb15 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb14_SPEC;
    pub type Mb14 = crate::EnumBitfieldStruct<u8, Mb14_SPEC>;
    impl Mb14 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb13_SPEC;
    pub type Mb13 = crate::EnumBitfieldStruct<u8, Mb13_SPEC>;
    impl Mb13 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb12_SPEC;
    pub type Mb12 = crate::EnumBitfieldStruct<u8, Mb12_SPEC>;
    impl Mb12 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb11_SPEC;
    pub type Mb11 = crate::EnumBitfieldStruct<u8, Mb11_SPEC>;
    impl Mb11 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb10_SPEC;
    pub type Mb10 = crate::EnumBitfieldStruct<u8, Mb10_SPEC>;
    impl Mb10 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb9_SPEC;
    pub type Mb9 = crate::EnumBitfieldStruct<u8, Mb9_SPEC>;
    impl Mb9 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb8_SPEC;
    pub type Mb8 = crate::EnumBitfieldStruct<u8, Mb8_SPEC>;
    impl Mb8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb7_SPEC;
    pub type Mb7 = crate::EnumBitfieldStruct<u8, Mb7_SPEC>;
    impl Mb7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb6_SPEC;
    pub type Mb6 = crate::EnumBitfieldStruct<u8, Mb6_SPEC>;
    impl Mb6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb5_SPEC;
    pub type Mb5 = crate::EnumBitfieldStruct<u8, Mb5_SPEC>;
    impl Mb5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb4_SPEC;
    pub type Mb4 = crate::EnumBitfieldStruct<u8, Mb4_SPEC>;
    impl Mb4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb3_SPEC;
    pub type Mb3 = crate::EnumBitfieldStruct<u8, Mb3_SPEC>;
    impl Mb3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb2_SPEC;
    pub type Mb2 = crate::EnumBitfieldStruct<u8, Mb2_SPEC>;
    impl Mb2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb1_SPEC;
    pub type Mb1 = crate::EnumBitfieldStruct<u8, Mb1_SPEC>;
    impl Mb1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb0_SPEC;
    pub type Mb0 = crate::EnumBitfieldStruct<u8, Mb0_SPEC>;
    impl Mb0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mier_SPEC;
impl crate::sealed::RegSpec for Mier_SPEC {
    type DataType = u32;
}

pub type Mier = crate::RegValueT<Mier_SPEC>;

impl Mier {
    #[inline(always)]
    pub fn mb31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        mier::Mb31,
        mier::Mb31,
        Mier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            mier::Mb31,
            mier::Mb31,
            Mier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb30(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        mier::Mb30,
        mier::Mb30,
        Mier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            mier::Mb30,
            mier::Mb30,
            Mier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb29(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        mier::Mb29,
        mier::Mb29,
        Mier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            mier::Mb29,
            mier::Mb29,
            Mier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        mier::Mb28,
        mier::Mb28,
        Mier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            mier::Mb28,
            mier::Mb28,
            Mier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        mier::Mb27,
        mier::Mb27,
        Mier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            mier::Mb27,
            mier::Mb27,
            Mier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb26(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        mier::Mb26,
        mier::Mb26,
        Mier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            mier::Mb26,
            mier::Mb26,
            Mier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb25(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        mier::Mb25,
        mier::Mb25,
        Mier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            mier::Mb25,
            mier::Mb25,
            Mier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb24(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        mier::Mb24,
        mier::Mb24,
        Mier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            mier::Mb24,
            mier::Mb24,
            Mier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        mier::Mb23,
        mier::Mb23,
        Mier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            mier::Mb23,
            mier::Mb23,
            Mier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        mier::Mb22,
        mier::Mb22,
        Mier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            mier::Mb22,
            mier::Mb22,
            Mier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb21(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        mier::Mb21,
        mier::Mb21,
        Mier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            mier::Mb21,
            mier::Mb21,
            Mier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb20(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        mier::Mb20,
        mier::Mb20,
        Mier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            mier::Mb20,
            mier::Mb20,
            Mier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        mier::Mb19,
        mier::Mb19,
        Mier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            mier::Mb19,
            mier::Mb19,
            Mier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        mier::Mb18,
        mier::Mb18,
        Mier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            mier::Mb18,
            mier::Mb18,
            Mier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        mier::Mb17,
        mier::Mb17,
        Mier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            mier::Mb17,
            mier::Mb17,
            Mier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        mier::Mb16,
        mier::Mb16,
        Mier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            mier::Mb16,
            mier::Mb16,
            Mier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        mier::Mb15,
        mier::Mb15,
        Mier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            mier::Mb15,
            mier::Mb15,
            Mier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        mier::Mb14,
        mier::Mb14,
        Mier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            mier::Mb14,
            mier::Mb14,
            Mier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        mier::Mb13,
        mier::Mb13,
        Mier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            mier::Mb13,
            mier::Mb13,
            Mier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        mier::Mb12,
        mier::Mb12,
        Mier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            mier::Mb12,
            mier::Mb12,
            Mier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        mier::Mb11,
        mier::Mb11,
        Mier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            mier::Mb11,
            mier::Mb11,
            Mier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        mier::Mb10,
        mier::Mb10,
        Mier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            mier::Mb10,
            mier::Mb10,
            Mier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        mier::Mb9,
        mier::Mb9,
        Mier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            mier::Mb9,
            mier::Mb9,
            Mier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        mier::Mb8,
        mier::Mb8,
        Mier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            mier::Mb8,
            mier::Mb8,
            Mier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        mier::Mb7,
        mier::Mb7,
        Mier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            mier::Mb7,
            mier::Mb7,
            Mier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        mier::Mb6,
        mier::Mb6,
        Mier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            mier::Mb6,
            mier::Mb6,
            Mier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        mier::Mb5,
        mier::Mb5,
        Mier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            mier::Mb5,
            mier::Mb5,
            Mier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        mier::Mb4,
        mier::Mb4,
        Mier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            mier::Mb4,
            mier::Mb4,
            Mier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        mier::Mb3,
        mier::Mb3,
        Mier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            mier::Mb3,
            mier::Mb3,
            Mier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mier::Mb2,
        mier::Mb2,
        Mier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mier::Mb2,
            mier::Mb2,
            Mier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mier::Mb1,
        mier::Mb1,
        Mier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mier::Mb1,
            mier::Mb1,
            Mier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mier::Mb0,
        mier::Mb0,
        Mier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mier::Mb0,
            mier::Mb0,
            Mier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mier {
    #[inline(always)]
    fn default() -> Mier {
        <crate::RegValueT<Mier_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mier {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb31_SPEC;
    pub type Mb31 = crate::EnumBitfieldStruct<u8, Mb31_SPEC>;
    impl Mb31 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb30_SPEC;
    pub type Mb30 = crate::EnumBitfieldStruct<u8, Mb30_SPEC>;
    impl Mb30 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb29_SPEC;
    pub type Mb29 = crate::EnumBitfieldStruct<u8, Mb29_SPEC>;
    impl Mb29 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb28_SPEC;
    pub type Mb28 = crate::EnumBitfieldStruct<u8, Mb28_SPEC>;
    impl Mb28 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb27_SPEC;
    pub type Mb27 = crate::EnumBitfieldStruct<u8, Mb27_SPEC>;
    impl Mb27 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb26_SPEC;
    pub type Mb26 = crate::EnumBitfieldStruct<u8, Mb26_SPEC>;
    impl Mb26 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb25_SPEC;
    pub type Mb25 = crate::EnumBitfieldStruct<u8, Mb25_SPEC>;
    impl Mb25 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb24_SPEC;
    pub type Mb24 = crate::EnumBitfieldStruct<u8, Mb24_SPEC>;
    impl Mb24 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb23_SPEC;
    pub type Mb23 = crate::EnumBitfieldStruct<u8, Mb23_SPEC>;
    impl Mb23 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb22_SPEC;
    pub type Mb22 = crate::EnumBitfieldStruct<u8, Mb22_SPEC>;
    impl Mb22 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb21_SPEC;
    pub type Mb21 = crate::EnumBitfieldStruct<u8, Mb21_SPEC>;
    impl Mb21 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb20_SPEC;
    pub type Mb20 = crate::EnumBitfieldStruct<u8, Mb20_SPEC>;
    impl Mb20 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb19_SPEC;
    pub type Mb19 = crate::EnumBitfieldStruct<u8, Mb19_SPEC>;
    impl Mb19 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb18_SPEC;
    pub type Mb18 = crate::EnumBitfieldStruct<u8, Mb18_SPEC>;
    impl Mb18 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb17_SPEC;
    pub type Mb17 = crate::EnumBitfieldStruct<u8, Mb17_SPEC>;
    impl Mb17 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb16_SPEC;
    pub type Mb16 = crate::EnumBitfieldStruct<u8, Mb16_SPEC>;
    impl Mb16 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb15_SPEC;
    pub type Mb15 = crate::EnumBitfieldStruct<u8, Mb15_SPEC>;
    impl Mb15 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb14_SPEC;
    pub type Mb14 = crate::EnumBitfieldStruct<u8, Mb14_SPEC>;
    impl Mb14 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb13_SPEC;
    pub type Mb13 = crate::EnumBitfieldStruct<u8, Mb13_SPEC>;
    impl Mb13 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb12_SPEC;
    pub type Mb12 = crate::EnumBitfieldStruct<u8, Mb12_SPEC>;
    impl Mb12 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb11_SPEC;
    pub type Mb11 = crate::EnumBitfieldStruct<u8, Mb11_SPEC>;
    impl Mb11 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb10_SPEC;
    pub type Mb10 = crate::EnumBitfieldStruct<u8, Mb10_SPEC>;
    impl Mb10 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb9_SPEC;
    pub type Mb9 = crate::EnumBitfieldStruct<u8, Mb9_SPEC>;
    impl Mb9 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb8_SPEC;
    pub type Mb8 = crate::EnumBitfieldStruct<u8, Mb8_SPEC>;
    impl Mb8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb7_SPEC;
    pub type Mb7 = crate::EnumBitfieldStruct<u8, Mb7_SPEC>;
    impl Mb7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb6_SPEC;
    pub type Mb6 = crate::EnumBitfieldStruct<u8, Mb6_SPEC>;
    impl Mb6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb5_SPEC;
    pub type Mb5 = crate::EnumBitfieldStruct<u8, Mb5_SPEC>;
    impl Mb5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb4_SPEC;
    pub type Mb4 = crate::EnumBitfieldStruct<u8, Mb4_SPEC>;
    impl Mb4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb3_SPEC;
    pub type Mb3 = crate::EnumBitfieldStruct<u8, Mb3_SPEC>;
    impl Mb3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb2_SPEC;
    pub type Mb2 = crate::EnumBitfieldStruct<u8, Mb2_SPEC>;
    impl Mb2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb1_SPEC;
    pub type Mb1 = crate::EnumBitfieldStruct<u8, Mb1_SPEC>;
    impl Mb1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb0_SPEC;
    pub type Mb0 = crate::EnumBitfieldStruct<u8, Mb0_SPEC>;
    impl Mb0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MierFifo_SPEC;
impl crate::sealed::RegSpec for MierFifo_SPEC {
    type DataType = u32;
}

pub type MierFifo = crate::RegValueT<MierFifo_SPEC>;

impl MierFifo {
    #[inline(always)]
    pub fn mb29(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        mier_fifo::Mb29,
        mier_fifo::Mb29,
        MierFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            mier_fifo::Mb29,
            mier_fifo::Mb29,
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        mier_fifo::Mb28,
        mier_fifo::Mb28,
        MierFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            mier_fifo::Mb28,
            mier_fifo::Mb28,
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, u8, MierFifo_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x3,1,0,u8,u8,MierFifo_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mb25(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        mier_fifo::Mb25,
        mier_fifo::Mb25,
        MierFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            mier_fifo::Mb25,
            mier_fifo::Mb25,
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb24(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        mier_fifo::Mb24,
        mier_fifo::Mb24,
        MierFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            mier_fifo::Mb24,
            mier_fifo::Mb24,
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        mier_fifo::Mb23,
        mier_fifo::Mb23,
        MierFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            mier_fifo::Mb23,
            mier_fifo::Mb23,
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        mier_fifo::Mb22,
        mier_fifo::Mb22,
        MierFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            mier_fifo::Mb22,
            mier_fifo::Mb22,
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb21(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        mier_fifo::Mb21,
        mier_fifo::Mb21,
        MierFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            mier_fifo::Mb21,
            mier_fifo::Mb21,
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb20(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        mier_fifo::Mb20,
        mier_fifo::Mb20,
        MierFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            mier_fifo::Mb20,
            mier_fifo::Mb20,
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        mier_fifo::Mb19,
        mier_fifo::Mb19,
        MierFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            mier_fifo::Mb19,
            mier_fifo::Mb19,
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        mier_fifo::Mb18,
        mier_fifo::Mb18,
        MierFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            mier_fifo::Mb18,
            mier_fifo::Mb18,
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        mier_fifo::Mb17,
        mier_fifo::Mb17,
        MierFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            mier_fifo::Mb17,
            mier_fifo::Mb17,
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        mier_fifo::Mb16,
        mier_fifo::Mb16,
        MierFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            mier_fifo::Mb16,
            mier_fifo::Mb16,
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        mier_fifo::Mb15,
        mier_fifo::Mb15,
        MierFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            mier_fifo::Mb15,
            mier_fifo::Mb15,
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        mier_fifo::Mb14,
        mier_fifo::Mb14,
        MierFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            mier_fifo::Mb14,
            mier_fifo::Mb14,
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        mier_fifo::Mb13,
        mier_fifo::Mb13,
        MierFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            mier_fifo::Mb13,
            mier_fifo::Mb13,
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        mier_fifo::Mb12,
        mier_fifo::Mb12,
        MierFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            mier_fifo::Mb12,
            mier_fifo::Mb12,
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        mier_fifo::Mb11,
        mier_fifo::Mb11,
        MierFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            mier_fifo::Mb11,
            mier_fifo::Mb11,
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        mier_fifo::Mb10,
        mier_fifo::Mb10,
        MierFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            mier_fifo::Mb10,
            mier_fifo::Mb10,
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        mier_fifo::Mb9,
        mier_fifo::Mb9,
        MierFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            mier_fifo::Mb9,
            mier_fifo::Mb9,
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        mier_fifo::Mb8,
        mier_fifo::Mb8,
        MierFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            mier_fifo::Mb8,
            mier_fifo::Mb8,
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        mier_fifo::Mb7,
        mier_fifo::Mb7,
        MierFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            mier_fifo::Mb7,
            mier_fifo::Mb7,
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        mier_fifo::Mb6,
        mier_fifo::Mb6,
        MierFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            mier_fifo::Mb6,
            mier_fifo::Mb6,
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        mier_fifo::Mb5,
        mier_fifo::Mb5,
        MierFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            mier_fifo::Mb5,
            mier_fifo::Mb5,
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        mier_fifo::Mb4,
        mier_fifo::Mb4,
        MierFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            mier_fifo::Mb4,
            mier_fifo::Mb4,
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        mier_fifo::Mb3,
        mier_fifo::Mb3,
        MierFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            mier_fifo::Mb3,
            mier_fifo::Mb3,
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mier_fifo::Mb2,
        mier_fifo::Mb2,
        MierFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mier_fifo::Mb2,
            mier_fifo::Mb2,
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mier_fifo::Mb1,
        mier_fifo::Mb1,
        MierFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mier_fifo::Mb1,
            mier_fifo::Mb1,
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mb0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mier_fifo::Mb0,
        mier_fifo::Mb0,
        MierFifo_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mier_fifo::Mb0,
            mier_fifo::Mb0,
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for MierFifo {
    #[inline(always)]
    fn default() -> MierFifo {
        <crate::RegValueT<MierFifo_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mier_fifo {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb29_SPEC;
    pub type Mb29 = crate::EnumBitfieldStruct<u8, Mb29_SPEC>;
    impl Mb29 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb28_SPEC;
    pub type Mb28 = crate::EnumBitfieldStruct<u8, Mb28_SPEC>;
    impl Mb28 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb25_SPEC;
    pub type Mb25 = crate::EnumBitfieldStruct<u8, Mb25_SPEC>;
    impl Mb25 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb24_SPEC;
    pub type Mb24 = crate::EnumBitfieldStruct<u8, Mb24_SPEC>;
    impl Mb24 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb23_SPEC;
    pub type Mb23 = crate::EnumBitfieldStruct<u8, Mb23_SPEC>;
    impl Mb23 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb22_SPEC;
    pub type Mb22 = crate::EnumBitfieldStruct<u8, Mb22_SPEC>;
    impl Mb22 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb21_SPEC;
    pub type Mb21 = crate::EnumBitfieldStruct<u8, Mb21_SPEC>;
    impl Mb21 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb20_SPEC;
    pub type Mb20 = crate::EnumBitfieldStruct<u8, Mb20_SPEC>;
    impl Mb20 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb19_SPEC;
    pub type Mb19 = crate::EnumBitfieldStruct<u8, Mb19_SPEC>;
    impl Mb19 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb18_SPEC;
    pub type Mb18 = crate::EnumBitfieldStruct<u8, Mb18_SPEC>;
    impl Mb18 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb17_SPEC;
    pub type Mb17 = crate::EnumBitfieldStruct<u8, Mb17_SPEC>;
    impl Mb17 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb16_SPEC;
    pub type Mb16 = crate::EnumBitfieldStruct<u8, Mb16_SPEC>;
    impl Mb16 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb15_SPEC;
    pub type Mb15 = crate::EnumBitfieldStruct<u8, Mb15_SPEC>;
    impl Mb15 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb14_SPEC;
    pub type Mb14 = crate::EnumBitfieldStruct<u8, Mb14_SPEC>;
    impl Mb14 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb13_SPEC;
    pub type Mb13 = crate::EnumBitfieldStruct<u8, Mb13_SPEC>;
    impl Mb13 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb12_SPEC;
    pub type Mb12 = crate::EnumBitfieldStruct<u8, Mb12_SPEC>;
    impl Mb12 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb11_SPEC;
    pub type Mb11 = crate::EnumBitfieldStruct<u8, Mb11_SPEC>;
    impl Mb11 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb10_SPEC;
    pub type Mb10 = crate::EnumBitfieldStruct<u8, Mb10_SPEC>;
    impl Mb10 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb9_SPEC;
    pub type Mb9 = crate::EnumBitfieldStruct<u8, Mb9_SPEC>;
    impl Mb9 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb8_SPEC;
    pub type Mb8 = crate::EnumBitfieldStruct<u8, Mb8_SPEC>;
    impl Mb8 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb7_SPEC;
    pub type Mb7 = crate::EnumBitfieldStruct<u8, Mb7_SPEC>;
    impl Mb7 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb6_SPEC;
    pub type Mb6 = crate::EnumBitfieldStruct<u8, Mb6_SPEC>;
    impl Mb6 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb5_SPEC;
    pub type Mb5 = crate::EnumBitfieldStruct<u8, Mb5_SPEC>;
    impl Mb5 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb4_SPEC;
    pub type Mb4 = crate::EnumBitfieldStruct<u8, Mb4_SPEC>;
    impl Mb4 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb3_SPEC;
    pub type Mb3 = crate::EnumBitfieldStruct<u8, Mb3_SPEC>;
    impl Mb3 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb2_SPEC;
    pub type Mb2 = crate::EnumBitfieldStruct<u8, Mb2_SPEC>;
    impl Mb2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb1_SPEC;
    pub type Mb1 = crate::EnumBitfieldStruct<u8, Mb1_SPEC>;
    impl Mb1 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb0_SPEC;
    pub type Mb0 = crate::EnumBitfieldStruct<u8, Mb0_SPEC>;
    impl Mb0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MctlTx_SPEC;
impl crate::sealed::RegSpec for MctlTx_SPEC {
    type DataType = u8;
}

pub type MctlTx = crate::RegValueT<MctlTx_SPEC>;

impl MctlTx {
    #[inline(always)]
    pub fn trmreq(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        mctl_tx::Trmreq,
        mctl_tx::Trmreq,
        MctlTx_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            mctl_tx::Trmreq,
            mctl_tx::Trmreq,
            MctlTx_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn recreq(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        mctl_tx::Recreq,
        mctl_tx::Recreq,
        MctlTx_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            mctl_tx::Recreq,
            mctl_tx::Recreq,
            MctlTx_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn oneshot(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        mctl_tx::Oneshot,
        mctl_tx::Oneshot,
        MctlTx_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            mctl_tx::Oneshot,
            mctl_tx::Oneshot,
            MctlTx_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, MctlTx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, MctlTx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn trmabt(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mctl_tx::Trmabt,
        mctl_tx::Trmabt,
        MctlTx_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mctl_tx::Trmabt,
            mctl_tx::Trmabt,
            MctlTx_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn trmactive(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mctl_tx::Trmactive,
        mctl_tx::Trmactive,
        MctlTx_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mctl_tx::Trmactive,
            mctl_tx::Trmactive,
            MctlTx_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sentdata(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mctl_tx::Sentdata,
        mctl_tx::Sentdata,
        MctlTx_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mctl_tx::Sentdata,
            mctl_tx::Sentdata,
            MctlTx_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for MctlTx {
    #[inline(always)]
    fn default() -> MctlTx {
        <crate::RegValueT<MctlTx_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mctl_tx {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trmreq_SPEC;
    pub type Trmreq = crate::EnumBitfieldStruct<u8, Trmreq_SPEC>;
    impl Trmreq {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Recreq_SPEC;
    pub type Recreq = crate::EnumBitfieldStruct<u8, Recreq_SPEC>;
    impl Recreq {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oneshot_SPEC;
    pub type Oneshot = crate::EnumBitfieldStruct<u8, Oneshot_SPEC>;
    impl Oneshot {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trmabt_SPEC;
    pub type Trmabt = crate::EnumBitfieldStruct<u8, Trmabt_SPEC>;
    impl Trmabt {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trmactive_SPEC;
    pub type Trmactive = crate::EnumBitfieldStruct<u8, Trmactive_SPEC>;
    impl Trmactive {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sentdata_SPEC;
    pub type Sentdata = crate::EnumBitfieldStruct<u8, Sentdata_SPEC>;
    impl Sentdata {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MctlRx_SPEC;
impl crate::sealed::RegSpec for MctlRx_SPEC {
    type DataType = u8;
}

pub type MctlRx = crate::RegValueT<MctlRx_SPEC>;

impl MctlRx {
    #[inline(always)]
    pub fn trmreq(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        mctl_rx::Trmreq,
        mctl_rx::Trmreq,
        MctlRx_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            mctl_rx::Trmreq,
            mctl_rx::Trmreq,
            MctlRx_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn recreq(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        mctl_rx::Recreq,
        mctl_rx::Recreq,
        MctlRx_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            mctl_rx::Recreq,
            mctl_rx::Recreq,
            MctlRx_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn oneshot(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        mctl_rx::Oneshot,
        mctl_rx::Oneshot,
        MctlRx_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            mctl_rx::Oneshot,
            mctl_rx::Oneshot,
            MctlRx_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, MctlRx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, MctlRx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn msglost(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mctl_rx::Msglost,
        mctl_rx::Msglost,
        MctlRx_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mctl_rx::Msglost,
            mctl_rx::Msglost,
            MctlRx_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn invaldata(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mctl_rx::Invaldata,
        mctl_rx::Invaldata,
        MctlRx_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mctl_rx::Invaldata,
            mctl_rx::Invaldata,
            MctlRx_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn newdata(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mctl_rx::Newdata,
        mctl_rx::Newdata,
        MctlRx_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mctl_rx::Newdata,
            mctl_rx::Newdata,
            MctlRx_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for MctlRx {
    #[inline(always)]
    fn default() -> MctlRx {
        <crate::RegValueT<MctlRx_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mctl_rx {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trmreq_SPEC;
    pub type Trmreq = crate::EnumBitfieldStruct<u8, Trmreq_SPEC>;
    impl Trmreq {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Recreq_SPEC;
    pub type Recreq = crate::EnumBitfieldStruct<u8, Recreq_SPEC>;
    impl Recreq {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oneshot_SPEC;
    pub type Oneshot = crate::EnumBitfieldStruct<u8, Oneshot_SPEC>;
    impl Oneshot {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Msglost_SPEC;
    pub type Msglost = crate::EnumBitfieldStruct<u8, Msglost_SPEC>;
    impl Msglost {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Invaldata_SPEC;
    pub type Invaldata = crate::EnumBitfieldStruct<u8, Invaldata_SPEC>;
    impl Invaldata {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Newdata_SPEC;
    pub type Newdata = crate::EnumBitfieldStruct<u8, Newdata_SPEC>;
    impl Newdata {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctlr_SPEC;
impl crate::sealed::RegSpec for Ctlr_SPEC {
    type DataType = u16;
}

pub type Ctlr = crate::RegValueT<Ctlr_SPEC>;

impl Ctlr {
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, Ctlr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,Ctlr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rboc(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        ctlr::Rboc,
        ctlr::Rboc,
        Ctlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            ctlr::Rboc,
            ctlr::Rboc,
            Ctlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bom(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x3,
        1,
        0,
        ctlr::Bom,
        ctlr::Bom,
        Ctlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x3,
            1,
            0,
            ctlr::Bom,
            ctlr::Bom,
            Ctlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn slpm(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        ctlr::Slpm,
        ctlr::Slpm,
        Ctlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            ctlr::Slpm,
            ctlr::Slpm,
            Ctlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn canm(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        ctlr::Canm,
        ctlr::Canm,
        Ctlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            ctlr::Canm,
            ctlr::Canm,
            Ctlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tsps(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        ctlr::Tsps,
        ctlr::Tsps,
        Ctlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            ctlr::Tsps,
            ctlr::Tsps,
            Ctlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tsrc(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ctlr::Tsrc,
        ctlr::Tsrc,
        Ctlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ctlr::Tsrc,
            ctlr::Tsrc,
            Ctlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tpm(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ctlr::Tpm,
        ctlr::Tpm,
        Ctlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ctlr::Tpm,
            ctlr::Tpm,
            Ctlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mlm(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ctlr::Mlm,
        ctlr::Mlm,
        Ctlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ctlr::Mlm,
            ctlr::Mlm,
            Ctlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn idfm(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x3,
        1,
        0,
        ctlr::Idfm,
        ctlr::Idfm,
        Ctlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x3,
            1,
            0,
            ctlr::Idfm,
            ctlr::Idfm,
            Ctlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mbm(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ctlr::Mbm,
        ctlr::Mbm,
        Ctlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ctlr::Mbm,
            ctlr::Mbm,
            Ctlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ctlr {
    #[inline(always)]
    fn default() -> Ctlr {
        <crate::RegValueT<Ctlr_SPEC> as RegisterValue<_>>::new(1280)
    }
}
pub mod ctlr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rboc_SPEC;
    pub type Rboc = crate::EnumBitfieldStruct<u8, Rboc_SPEC>;
    impl Rboc {
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
    pub struct Slpm_SPEC;
    pub type Slpm = crate::EnumBitfieldStruct<u8, Slpm_SPEC>;
    impl Slpm {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Canm_SPEC;
    pub type Canm = crate::EnumBitfieldStruct<u8, Canm_SPEC>;
    impl Canm {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsps_SPEC;
    pub type Tsps = crate::EnumBitfieldStruct<u8, Tsps_SPEC>;
    impl Tsps {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsrc_SPEC;
    pub type Tsrc = crate::EnumBitfieldStruct<u8, Tsrc_SPEC>;
    impl Tsrc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tpm_SPEC;
    pub type Tpm = crate::EnumBitfieldStruct<u8, Tpm_SPEC>;
    impl Tpm {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mlm_SPEC;
    pub type Mlm = crate::EnumBitfieldStruct<u8, Mlm_SPEC>;
    impl Mlm {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Idfm_SPEC;
    pub type Idfm = crate::EnumBitfieldStruct<u8, Idfm_SPEC>;
    impl Idfm {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mbm_SPEC;
    pub type Mbm = crate::EnumBitfieldStruct<u8, Mbm_SPEC>;
    impl Mbm {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Str_SPEC;
impl crate::sealed::RegSpec for Str_SPEC {
    type DataType = u16;
}

pub type Str = crate::RegValueT<Str_SPEC>;

impl Str {
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Str_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Str_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn recst(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        str::Recst,
        str::Recst,
        Str_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            str::Recst,
            str::Recst,
            Str_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn trmst(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        str::Trmst,
        str::Trmst,
        Str_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            str::Trmst,
            str::Trmst,
            Str_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bost(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, str::Bost, str::Bost, Str_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            str::Bost,
            str::Bost,
            Str_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn epst(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, str::Epst, str::Epst, Str_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            str::Epst,
            str::Epst,
            Str_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn slpst(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        str::Slpst,
        str::Slpst,
        Str_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            str::Slpst,
            str::Slpst,
            Str_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn hltst(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        str::Hltst,
        str::Hltst,
        Str_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            str::Hltst,
            str::Hltst,
            Str_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rstst(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        str::Rstst,
        str::Rstst,
        Str_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            str::Rstst,
            str::Rstst,
            Str_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn est(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, str::Est, str::Est, Str_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<7,0x1,1,0,str::Est,str::Est,Str_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tabst(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        str::Tabst,
        str::Tabst,
        Str_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            str::Tabst,
            str::Tabst,
            Str_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fmlst(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        str::Fmlst,
        str::Fmlst,
        Str_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            str::Fmlst,
            str::Fmlst,
            Str_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn nmlst(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        str::Nmlst,
        str::Nmlst,
        Str_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            str::Nmlst,
            str::Nmlst,
            Str_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tfst(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, str::Tfst, str::Tfst, Str_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<3,0x1,1,0,str::Tfst,str::Tfst,Str_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfst(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, str::Rfst, str::Rfst, Str_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x1,1,0,str::Rfst,str::Rfst,Str_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sdst(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, str::Sdst, str::Sdst, Str_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,str::Sdst,str::Sdst,Str_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ndst(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, str::Ndst, str::Ndst, Str_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,str::Ndst,str::Ndst,Str_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Str {
    #[inline(always)]
    fn default() -> Str {
        <crate::RegValueT<Str_SPEC> as RegisterValue<_>>::new(1280)
    }
}
pub mod str {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Recst_SPEC;
    pub type Recst = crate::EnumBitfieldStruct<u8, Recst_SPEC>;
    impl Recst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trmst_SPEC;
    pub type Trmst = crate::EnumBitfieldStruct<u8, Trmst_SPEC>;
    impl Trmst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bost_SPEC;
    pub type Bost = crate::EnumBitfieldStruct<u8, Bost_SPEC>;
    impl Bost {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Epst_SPEC;
    pub type Epst = crate::EnumBitfieldStruct<u8, Epst_SPEC>;
    impl Epst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slpst_SPEC;
    pub type Slpst = crate::EnumBitfieldStruct<u8, Slpst_SPEC>;
    impl Slpst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hltst_SPEC;
    pub type Hltst = crate::EnumBitfieldStruct<u8, Hltst_SPEC>;
    impl Hltst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rstst_SPEC;
    pub type Rstst = crate::EnumBitfieldStruct<u8, Rstst_SPEC>;
    impl Rstst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Est_SPEC;
    pub type Est = crate::EnumBitfieldStruct<u8, Est_SPEC>;
    impl Est {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tabst_SPEC;
    pub type Tabst = crate::EnumBitfieldStruct<u8, Tabst_SPEC>;
    impl Tabst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fmlst_SPEC;
    pub type Fmlst = crate::EnumBitfieldStruct<u8, Fmlst_SPEC>;
    impl Fmlst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nmlst_SPEC;
    pub type Nmlst = crate::EnumBitfieldStruct<u8, Nmlst_SPEC>;
    impl Nmlst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tfst_SPEC;
    pub type Tfst = crate::EnumBitfieldStruct<u8, Tfst_SPEC>;
    impl Tfst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfst_SPEC;
    pub type Rfst = crate::EnumBitfieldStruct<u8, Rfst_SPEC>;
    impl Rfst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdst_SPEC;
    pub type Sdst = crate::EnumBitfieldStruct<u8, Sdst_SPEC>;
    impl Sdst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ndst_SPEC;
    pub type Ndst = crate::EnumBitfieldStruct<u8, Ndst_SPEC>;
    impl Ndst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcr_SPEC;
impl crate::sealed::RegSpec for Bcr_SPEC {
    type DataType = u32;
}

pub type Bcr = crate::RegValueT<Bcr_SPEC>;

impl Bcr {
    #[inline(always)]
    pub fn tseg1(
        self,
    ) -> crate::common::RegisterField<
        28,
        0xf,
        1,
        0,
        bcr::Tseg1,
        bcr::Tseg1,
        Bcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0xf,
            1,
            0,
            bcr::Tseg1,
            bcr::Tseg1,
            Bcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn brp(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, u16, Bcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3ff,1,0,u16,u16,Bcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sjw(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, bcr::Sjw, bcr::Sjw, Bcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,bcr::Sjw,bcr::Sjw,Bcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tseg2(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        bcr::Tseg2,
        bcr::Tseg2,
        Bcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            bcr::Tseg2,
            bcr::Tseg2,
            Bcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, Bcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,Bcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cclks(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        bcr::Cclks,
        bcr::Cclks,
        Bcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            bcr::Cclks,
            bcr::Cclks,
            Bcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Bcr {
    #[inline(always)]
    fn default() -> Bcr {
        <crate::RegValueT<Bcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tseg1_SPEC;
    pub type Tseg1 = crate::EnumBitfieldStruct<u8, Tseg1_SPEC>;
    impl Tseg1 {
        pub const _0011: Self = Self::new(3);

        pub const _0100: Self = Self::new(4);

        pub const _0101: Self = Self::new(5);

        pub const _0110: Self = Self::new(6);

        pub const _0111: Self = Self::new(7);

        pub const _1000: Self = Self::new(8);

        pub const _1001: Self = Self::new(9);

        pub const _1010: Self = Self::new(10);

        pub const _1011: Self = Self::new(11);

        pub const _1100: Self = Self::new(12);

        pub const _1101: Self = Self::new(13);

        pub const _1110: Self = Self::new(14);

        pub const _1111: Self = Self::new(15);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sjw_SPEC;
    pub type Sjw = crate::EnumBitfieldStruct<u8, Sjw_SPEC>;
    impl Sjw {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tseg2_SPEC;
    pub type Tseg2 = crate::EnumBitfieldStruct<u8, Tseg2_SPEC>;
    impl Tseg2 {
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
    pub struct Cclks_SPEC;
    pub type Cclks = crate::EnumBitfieldStruct<u8, Cclks_SPEC>;
    impl Cclks {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rfcr_SPEC;
impl crate::sealed::RegSpec for Rfcr_SPEC {
    type DataType = u8;
}

pub type Rfcr = crate::RegValueT<Rfcr_SPEC>;

impl Rfcr {
    #[inline(always)]
    pub fn rfest(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        rfcr::Rfest,
        rfcr::Rfest,
        Rfcr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            rfcr::Rfest,
            rfcr::Rfest,
            Rfcr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfwst(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        rfcr::Rfwst,
        rfcr::Rfwst,
        Rfcr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            rfcr::Rfwst,
            rfcr::Rfwst,
            Rfcr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rffst(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        rfcr::Rffst,
        rfcr::Rffst,
        Rfcr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            rfcr::Rffst,
            rfcr::Rffst,
            Rfcr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfmlf(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        rfcr::Rfmlf,
        rfcr::Rfmlf,
        Rfcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            rfcr::Rfmlf,
            rfcr::Rfmlf,
            Rfcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfust(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x7,
        1,
        0,
        rfcr::Rfust,
        rfcr::Rfust,
        Rfcr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x7,
            1,
            0,
            rfcr::Rfust,
            rfcr::Rfust,
            Rfcr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rfe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        rfcr::Rfe,
        rfcr::Rfe,
        Rfcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            rfcr::Rfe,
            rfcr::Rfe,
            Rfcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Rfcr {
    #[inline(always)]
    fn default() -> Rfcr {
        <crate::RegValueT<Rfcr_SPEC> as RegisterValue<_>>::new(128)
    }
}
pub mod rfcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfest_SPEC;
    pub type Rfest = crate::EnumBitfieldStruct<u8, Rfest_SPEC>;
    impl Rfest {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfwst_SPEC;
    pub type Rfwst = crate::EnumBitfieldStruct<u8, Rfwst_SPEC>;
    impl Rfwst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rffst_SPEC;
    pub type Rffst = crate::EnumBitfieldStruct<u8, Rffst_SPEC>;
    impl Rffst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfmlf_SPEC;
    pub type Rfmlf = crate::EnumBitfieldStruct<u8, Rfmlf_SPEC>;
    impl Rfmlf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfust_SPEC;
    pub type Rfust = crate::EnumBitfieldStruct<u8, Rfust_SPEC>;
    impl Rfust {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfe_SPEC;
    pub type Rfe = crate::EnumBitfieldStruct<u8, Rfe_SPEC>;
    impl Rfe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rfpcr_SPEC;
impl crate::sealed::RegSpec for Rfpcr_SPEC {
    type DataType = u8;
}

pub type Rfpcr = crate::RegValueT<Rfpcr_SPEC>;

impl Rfpcr {
    #[inline(always)]
    pub fn rfpcr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Rfpcr_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Rfpcr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Rfpcr {
    #[inline(always)]
    fn default() -> Rfpcr {
        <crate::RegValueT<Rfpcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tfcr_SPEC;
impl crate::sealed::RegSpec for Tfcr_SPEC {
    type DataType = u8;
}

pub type Tfcr = crate::RegValueT<Tfcr_SPEC>;

impl Tfcr {
    #[inline(always)]
    pub fn tfest(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        tfcr::Tfest,
        tfcr::Tfest,
        Tfcr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            tfcr::Tfest,
            tfcr::Tfest,
            Tfcr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tffst(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        tfcr::Tffst,
        tfcr::Tffst,
        Tfcr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            tfcr::Tffst,
            tfcr::Tffst,
            Tfcr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Tfcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Tfcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tfust(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x7,
        1,
        0,
        tfcr::Tfust,
        tfcr::Tfust,
        Tfcr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x7,
            1,
            0,
            tfcr::Tfust,
            tfcr::Tfust,
            Tfcr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tfe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        tfcr::Tfe,
        tfcr::Tfe,
        Tfcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            tfcr::Tfe,
            tfcr::Tfe,
            Tfcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Tfcr {
    #[inline(always)]
    fn default() -> Tfcr {
        <crate::RegValueT<Tfcr_SPEC> as RegisterValue<_>>::new(128)
    }
}
pub mod tfcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tfest_SPEC;
    pub type Tfest = crate::EnumBitfieldStruct<u8, Tfest_SPEC>;
    impl Tfest {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tffst_SPEC;
    pub type Tffst = crate::EnumBitfieldStruct<u8, Tffst_SPEC>;
    impl Tffst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tfust_SPEC;
    pub type Tfust = crate::EnumBitfieldStruct<u8, Tfust_SPEC>;
    impl Tfust {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tfe_SPEC;
    pub type Tfe = crate::EnumBitfieldStruct<u8, Tfe_SPEC>;
    impl Tfe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tfpcr_SPEC;
impl crate::sealed::RegSpec for Tfpcr_SPEC {
    type DataType = u8;
}

pub type Tfpcr = crate::RegValueT<Tfpcr_SPEC>;

impl Tfpcr {
    #[inline(always)]
    pub fn tfpcr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Tfpcr_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Tfpcr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Tfpcr {
    #[inline(always)]
    fn default() -> Tfpcr {
        <crate::RegValueT<Tfpcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eier_SPEC;
impl crate::sealed::RegSpec for Eier_SPEC {
    type DataType = u8;
}

pub type Eier = crate::RegValueT<Eier_SPEC>;

impl Eier {
    #[inline(always)]
    pub fn blie(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        eier::Blie,
        eier::Blie,
        Eier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            eier::Blie,
            eier::Blie,
            Eier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn olie(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        eier::Olie,
        eier::Olie,
        Eier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            eier::Olie,
            eier::Olie,
            Eier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn orie(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        eier::Orie,
        eier::Orie,
        Eier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            eier::Orie,
            eier::Orie,
            Eier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn borie(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        eier::Borie,
        eier::Borie,
        Eier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            eier::Borie,
            eier::Borie,
            Eier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn boeie(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        eier::Boeie,
        eier::Boeie,
        Eier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            eier::Boeie,
            eier::Boeie,
            Eier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn epie(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        eier::Epie,
        eier::Epie,
        Eier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            eier::Epie,
            eier::Epie,
            Eier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ewie(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        eier::Ewie,
        eier::Ewie,
        Eier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            eier::Ewie,
            eier::Ewie,
            Eier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn beie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        eier::Beie,
        eier::Beie,
        Eier_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            eier::Beie,
            eier::Beie,
            Eier_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eier {
    #[inline(always)]
    fn default() -> Eier {
        <crate::RegValueT<Eier_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eier {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Blie_SPEC;
    pub type Blie = crate::EnumBitfieldStruct<u8, Blie_SPEC>;
    impl Blie {
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
    pub struct Orie_SPEC;
    pub type Orie = crate::EnumBitfieldStruct<u8, Orie_SPEC>;
    impl Orie {
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
    pub struct Boeie_SPEC;
    pub type Boeie = crate::EnumBitfieldStruct<u8, Boeie_SPEC>;
    impl Boeie {
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
    pub struct Ewie_SPEC;
    pub type Ewie = crate::EnumBitfieldStruct<u8, Ewie_SPEC>;
    impl Ewie {
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
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eifr_SPEC;
impl crate::sealed::RegSpec for Eifr_SPEC {
    type DataType = u8;
}

pub type Eifr = crate::RegValueT<Eifr_SPEC>;

impl Eifr {
    #[inline(always)]
    pub fn blif(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        eifr::Blif,
        eifr::Blif,
        Eifr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            eifr::Blif,
            eifr::Blif,
            Eifr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn olif(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        eifr::Olif,
        eifr::Olif,
        Eifr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            eifr::Olif,
            eifr::Olif,
            Eifr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn orif(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        eifr::Orif,
        eifr::Orif,
        Eifr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            eifr::Orif,
            eifr::Orif,
            Eifr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn borif(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        eifr::Borif,
        eifr::Borif,
        Eifr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            eifr::Borif,
            eifr::Borif,
            Eifr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn boeif(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        eifr::Boeif,
        eifr::Boeif,
        Eifr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            eifr::Boeif,
            eifr::Boeif,
            Eifr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn epif(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        eifr::Epif,
        eifr::Epif,
        Eifr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            eifr::Epif,
            eifr::Epif,
            Eifr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ewif(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        eifr::Ewif,
        eifr::Ewif,
        Eifr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            eifr::Ewif,
            eifr::Ewif,
            Eifr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn beif(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        eifr::Beif,
        eifr::Beif,
        Eifr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            eifr::Beif,
            eifr::Beif,
            Eifr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Eifr {
    #[inline(always)]
    fn default() -> Eifr {
        <crate::RegValueT<Eifr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eifr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Blif_SPEC;
    pub type Blif = crate::EnumBitfieldStruct<u8, Blif_SPEC>;
    impl Blif {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Olif_SPEC;
    pub type Olif = crate::EnumBitfieldStruct<u8, Olif_SPEC>;
    impl Olif {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Orif_SPEC;
    pub type Orif = crate::EnumBitfieldStruct<u8, Orif_SPEC>;
    impl Orif {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Borif_SPEC;
    pub type Borif = crate::EnumBitfieldStruct<u8, Borif_SPEC>;
    impl Borif {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Boeif_SPEC;
    pub type Boeif = crate::EnumBitfieldStruct<u8, Boeif_SPEC>;
    impl Boeif {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Epif_SPEC;
    pub type Epif = crate::EnumBitfieldStruct<u8, Epif_SPEC>;
    impl Epif {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ewif_SPEC;
    pub type Ewif = crate::EnumBitfieldStruct<u8, Ewif_SPEC>;
    impl Ewif {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Beif_SPEC;
    pub type Beif = crate::EnumBitfieldStruct<u8, Beif_SPEC>;
    impl Beif {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Recr_SPEC;
impl crate::sealed::RegSpec for Recr_SPEC {
    type DataType = u8;
}

pub type Recr = crate::RegValueT<Recr_SPEC>;

impl Recr {
    #[inline(always)]
    pub fn recr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Recr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Recr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Recr {
    #[inline(always)]
    fn default() -> Recr {
        <crate::RegValueT<Recr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tecr_SPEC;
impl crate::sealed::RegSpec for Tecr_SPEC {
    type DataType = u8;
}

pub type Tecr = crate::RegValueT<Tecr_SPEC>;

impl Tecr {
    #[inline(always)]
    pub fn tecr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Tecr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Tecr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Tecr {
    #[inline(always)]
    fn default() -> Tecr {
        <crate::RegValueT<Tecr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecsr_SPEC;
impl crate::sealed::RegSpec for Ecsr_SPEC {
    type DataType = u8;
}

pub type Ecsr = crate::RegValueT<Ecsr_SPEC>;

impl Ecsr {
    #[inline(always)]
    pub fn edpm(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ecsr::Edpm,
        ecsr::Edpm,
        Ecsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ecsr::Edpm,
            ecsr::Edpm,
            Ecsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adef(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        ecsr::Adef,
        ecsr::Adef,
        Ecsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ecsr::Adef,
            ecsr::Adef,
            Ecsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn be0f(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ecsr::Be0F,
        ecsr::Be0F,
        Ecsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ecsr::Be0F,
            ecsr::Be0F,
            Ecsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn be1f(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ecsr::Be1F,
        ecsr::Be1F,
        Ecsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ecsr::Be1F,
            ecsr::Be1F,
            Ecsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cef(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ecsr::Cef,
        ecsr::Cef,
        Ecsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ecsr::Cef,
            ecsr::Cef,
            Ecsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn aef(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ecsr::Aef,
        ecsr::Aef,
        Ecsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ecsr::Aef,
            ecsr::Aef,
            Ecsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fef(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ecsr::Fef,
        ecsr::Fef,
        Ecsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ecsr::Fef,
            ecsr::Fef,
            Ecsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sef(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ecsr::Sef,
        ecsr::Sef,
        Ecsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ecsr::Sef,
            ecsr::Sef,
            Ecsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ecsr {
    #[inline(always)]
    fn default() -> Ecsr {
        <crate::RegValueT<Ecsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ecsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Edpm_SPEC;
    pub type Edpm = crate::EnumBitfieldStruct<u8, Edpm_SPEC>;
    impl Edpm {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adef_SPEC;
    pub type Adef = crate::EnumBitfieldStruct<u8, Adef_SPEC>;
    impl Adef {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Be0F_SPEC;
    pub type Be0F = crate::EnumBitfieldStruct<u8, Be0F_SPEC>;
    impl Be0F {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Be1F_SPEC;
    pub type Be1F = crate::EnumBitfieldStruct<u8, Be1F_SPEC>;
    impl Be1F {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cef_SPEC;
    pub type Cef = crate::EnumBitfieldStruct<u8, Cef_SPEC>;
    impl Cef {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aef_SPEC;
    pub type Aef = crate::EnumBitfieldStruct<u8, Aef_SPEC>;
    impl Aef {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fef_SPEC;
    pub type Fef = crate::EnumBitfieldStruct<u8, Fef_SPEC>;
    impl Fef {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sef_SPEC;
    pub type Sef = crate::EnumBitfieldStruct<u8, Sef_SPEC>;
    impl Sef {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cssr_SPEC;
impl crate::sealed::RegSpec for Cssr_SPEC {
    type DataType = u8;
}

pub type Cssr = crate::RegValueT<Cssr_SPEC>;

impl Cssr {
    #[inline(always)]
    pub fn cssr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Cssr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Cssr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cssr {
    #[inline(always)]
    fn default() -> Cssr {
        <crate::RegValueT<Cssr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mssr_SPEC;
impl crate::sealed::RegSpec for Mssr_SPEC {
    type DataType = u8;
}

pub type Mssr = crate::RegValueT<Mssr_SPEC>;

impl Mssr {
    #[inline(always)]
    pub fn sest(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        mssr::Sest,
        mssr::Sest,
        Mssr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            mssr::Sest,
            mssr::Sest,
            Mssr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, Mssr_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,Mssr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mbnst(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Mssr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Mssr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Mssr {
    #[inline(always)]
    fn default() -> Mssr {
        <crate::RegValueT<Mssr_SPEC> as RegisterValue<_>>::new(128)
    }
}
pub mod mssr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sest_SPEC;
    pub type Sest = crate::EnumBitfieldStruct<u8, Sest_SPEC>;
    impl Sest {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msmr_SPEC;
impl crate::sealed::RegSpec for Msmr_SPEC {
    type DataType = u8;
}

pub type Msmr = crate::RegValueT<Msmr_SPEC>;

impl Msmr {
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, u8, Msmr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3f,1,0,u8,u8,Msmr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mbsm(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        msmr::Mbsm,
        msmr::Mbsm,
        Msmr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            msmr::Mbsm,
            msmr::Mbsm,
            Msmr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Msmr {
    #[inline(always)]
    fn default() -> Msmr {
        <crate::RegValueT<Msmr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod msmr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mbsm_SPEC;
    pub type Mbsm = crate::EnumBitfieldStruct<u8, Mbsm_SPEC>;
    impl Mbsm {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsr_SPEC;
impl crate::sealed::RegSpec for Tsr_SPEC {
    type DataType = u16;
}

pub type Tsr = crate::RegValueT<Tsr_SPEC>;

impl Tsr {
    #[inline(always)]
    pub fn tsr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Tsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Tsr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Tsr {
    #[inline(always)]
    fn default() -> Tsr {
        <crate::RegValueT<Tsr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Afsr_SPEC;
impl crate::sealed::RegSpec for Afsr_SPEC {
    type DataType = u16;
}

pub type Afsr = crate::RegValueT<Afsr_SPEC>;

impl Afsr {
    #[inline(always)]
    pub fn afsr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Afsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Afsr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Afsr {
    #[inline(always)]
    fn default() -> Afsr {
        <crate::RegValueT<Afsr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcr_SPEC;
impl crate::sealed::RegSpec for Tcr_SPEC {
    type DataType = u8;
}

pub type Tcr = crate::RegValueT<Tcr_SPEC>;

impl Tcr {
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, u8, Tcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8,u8,Tcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tstm(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, tcr::Tstm, tcr::Tstm, Tcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            1,
            0x3,
            1,
            0,
            tcr::Tstm,
            tcr::Tstm,
            Tcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tste(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, tcr::Tste, tcr::Tste, Tcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            tcr::Tste,
            tcr::Tste,
            Tcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Tcr {
    #[inline(always)]
    fn default() -> Tcr {
        <crate::RegValueT<Tcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tstm_SPEC;
    pub type Tstm = crate::EnumBitfieldStruct<u8, Tstm_SPEC>;
    impl Tstm {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tste_SPEC;
    pub type Tste = crate::EnumBitfieldStruct<u8, Tste_SPEC>;
    impl Tste {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
