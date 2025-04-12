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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:17:28 +0000

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
    #[doc = "Mailbox Register"]
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

    #[doc = "Mailbox Register"]
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

    #[doc = "Mailbox Register"]
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

    #[doc = "Mailbox Register"]
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

    #[doc = "Mailbox Register"]
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

    #[doc = "Mailbox Register"]
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

    #[doc = "Mailbox Register"]
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

    #[doc = "Mailbox Register"]
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

    #[doc = "Mailbox Register"]
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

    #[doc = "Mailbox Register"]
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

    #[doc = "Mailbox Register"]
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

    #[doc = "Mask Register"]
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

    #[doc = "FIFO Received ID Compare Registers"]
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

    #[doc = "Mask Invalid Register"]
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

    #[doc = "Mailbox Interrupt Enable Register"]
    #[inline(always)]
    pub const fn mier(&self) -> &'static crate::common::Reg<self::Mier_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mier_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1068usize),
            )
        }
    }

    #[doc = "Mailbox Interrupt Enable Register for FIFO Mailbox Mode"]
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

    #[doc = "Message Control Register for Transmit"]
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

    #[doc = "Message Control Register for Receive"]
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

    #[doc = "Control Register"]
    #[inline(always)]
    pub const fn ctlr(&self) -> &'static crate::common::Reg<self::Ctlr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ctlr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2112usize),
            )
        }
    }

    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn str(&self) -> &'static crate::common::Reg<self::Str_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Str_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2114usize),
            )
        }
    }

    #[doc = "Bit Configuration Register"]
    #[inline(always)]
    pub const fn bcr(&self) -> &'static crate::common::Reg<self::Bcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2116usize),
            )
        }
    }

    #[doc = "Receive FIFO Control Register"]
    #[inline(always)]
    pub const fn rfcr(&self) -> &'static crate::common::Reg<self::Rfcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rfcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2120usize),
            )
        }
    }

    #[doc = "Receive FIFO Pointer Control Register"]
    #[inline(always)]
    pub const fn rfpcr(&self) -> &'static crate::common::Reg<self::Rfpcr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Rfpcr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(2121usize),
            )
        }
    }

    #[doc = "Transmit FIFO Control Register"]
    #[inline(always)]
    pub const fn tfcr(&self) -> &'static crate::common::Reg<self::Tfcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tfcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2122usize),
            )
        }
    }

    #[doc = "Transmit FIFO Pointer Control Register"]
    #[inline(always)]
    pub const fn tfpcr(&self) -> &'static crate::common::Reg<self::Tfpcr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Tfpcr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(2123usize),
            )
        }
    }

    #[doc = "Error Interrupt Enable Register"]
    #[inline(always)]
    pub const fn eier(&self) -> &'static crate::common::Reg<self::Eier_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eier_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2124usize),
            )
        }
    }

    #[doc = "Error Interrupt Factor Judge Register"]
    #[inline(always)]
    pub const fn eifr(&self) -> &'static crate::common::Reg<self::Eifr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Eifr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2125usize),
            )
        }
    }

    #[doc = "Receive Error Count Register"]
    #[inline(always)]
    pub const fn recr(&self) -> &'static crate::common::Reg<self::Recr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Recr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2126usize),
            )
        }
    }

    #[doc = "Transmit Error Count Register"]
    #[inline(always)]
    pub const fn tecr(&self) -> &'static crate::common::Reg<self::Tecr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Tecr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2127usize),
            )
        }
    }

    #[doc = "Error Code Store Register"]
    #[inline(always)]
    pub const fn ecsr(&self) -> &'static crate::common::Reg<self::Ecsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ecsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2128usize),
            )
        }
    }

    #[doc = "Channel Search Support Register"]
    #[inline(always)]
    pub const fn cssr(&self) -> &'static crate::common::Reg<self::Cssr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cssr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2129usize),
            )
        }
    }

    #[doc = "Mailbox Search Status Register"]
    #[inline(always)]
    pub const fn mssr(&self) -> &'static crate::common::Reg<self::Mssr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Mssr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2130usize),
            )
        }
    }

    #[doc = "Mailbox Search Mode Register"]
    #[inline(always)]
    pub const fn msmr(&self) -> &'static crate::common::Reg<self::Msmr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Msmr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2131usize),
            )
        }
    }

    #[doc = "Time Stamp Register"]
    #[inline(always)]
    pub const fn tsr(&self) -> &'static crate::common::Reg<self::Tsr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Tsr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(2132usize),
            )
        }
    }

    #[doc = "Acceptance Filter Support Register"]
    #[inline(always)]
    pub const fn afsr(&self) -> &'static crate::common::Reg<self::Afsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Afsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2134usize),
            )
        }
    }

    #[doc = "Test Control Register"]
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
#[doc = "Mailbox Register"]
pub type MbId = crate::RegValueT<MbId_SPEC>;

impl MbId {
    #[doc = "ID Extension"]
    #[inline(always)]
    pub fn ide(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, mb_id::Ide, MbId_SPEC, crate::common::RW> {
        crate::common::RegisterField::<31,0x1,1,0,mb_id::Ide, MbId_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Remote Transmission Request"]
    #[inline(always)]
    pub fn rtr(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, mb_id::Rtr, MbId_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x1,1,0,mb_id::Rtr, MbId_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Standard ID"]
    #[inline(always)]
    pub fn sid(
        self,
    ) -> crate::common::RegisterField<18, 0x7ff, 1, 0, u16, MbId_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x7ff,1,0,u16, MbId_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Extended ID"]
    #[inline(always)]
    pub fn eid(
        self,
    ) -> crate::common::RegisterField<0, 0x3ffff, 1, 0, u32, MbId_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ffff,1,0,u32, MbId_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Standard ID"]
        pub const _0: Self = Self::new(0);
        #[doc = "Extended ID"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtr_SPEC;
    pub type Rtr = crate::EnumBitfieldStruct<u8, Rtr_SPEC>;
    impl Rtr {
        #[doc = "Data frame"]
        pub const _0: Self = Self::new(0);
        #[doc = "Remote frame"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbDl_SPEC;
impl crate::sealed::RegSpec for MbDl_SPEC {
    type DataType = u16;
}
#[doc = "Mailbox Register"]
pub type MbDl = crate::RegValueT<MbDl_SPEC>;

impl MbDl {
    #[doc = "Data Length Code"]
    #[inline(always)]
    pub fn dlc(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, mb_dl::Dlc, MbDl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,mb_dl::Dlc, MbDl_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Data length = 0 byte"]
        pub const _0000: Self = Self::new(0);
        #[doc = "Data length = 1 byte"]
        pub const _0001: Self = Self::new(1);
        #[doc = "Data length = 2 bytes"]
        pub const _0010: Self = Self::new(2);
        #[doc = "Data length = 3 bytes"]
        pub const _0011: Self = Self::new(3);
        #[doc = "Data length = 4 bytes"]
        pub const _0100: Self = Self::new(4);
        #[doc = "Data length = 5 bytes"]
        pub const _0101: Self = Self::new(5);
        #[doc = "Data length = 6 bytes"]
        pub const _0110: Self = Self::new(6);
        #[doc = "Data length = 7 bytes"]
        pub const _0111: Self = Self::new(7);
        #[doc = "Data length = 8 bytes"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbD0_SPEC;
impl crate::sealed::RegSpec for MbD0_SPEC {
    type DataType = u8;
}
#[doc = "Mailbox Register"]
pub type MbD0 = crate::RegValueT<MbD0_SPEC>;

impl MbD0 {
    #[doc = "Data Bytes 0.DATA0  store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[inline(always)]
    pub fn data0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, MbD0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, MbD0_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Mailbox Register"]
pub type MbD1 = crate::RegValueT<MbD1_SPEC>;

impl MbD1 {
    #[doc = "Data Bytes 1DATA1  store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[inline(always)]
    pub fn data1(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, MbD1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, MbD1_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Mailbox Register"]
pub type MbD2 = crate::RegValueT<MbD2_SPEC>;

impl MbD2 {
    #[doc = "Data Bytes 2DATA2  store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[inline(always)]
    pub fn data2(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, MbD2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, MbD2_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Mailbox Register"]
pub type MbD3 = crate::RegValueT<MbD3_SPEC>;

impl MbD3 {
    #[doc = "Data Bytes 3DATA3  store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[inline(always)]
    pub fn data3(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, MbD3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, MbD3_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Mailbox Register"]
pub type MbD4 = crate::RegValueT<MbD4_SPEC>;

impl MbD4 {
    #[doc = "Data Bytes 4DATA4 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[inline(always)]
    pub fn data4(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, MbD4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, MbD4_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Mailbox Register"]
pub type MbD5 = crate::RegValueT<MbD5_SPEC>;

impl MbD5 {
    #[doc = "Data Bytes 5DATA5 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[inline(always)]
    pub fn data5(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, MbD5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, MbD5_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Mailbox Register"]
pub type MbD6 = crate::RegValueT<MbD6_SPEC>;

impl MbD6 {
    #[doc = "Data Bytes 6DATA6 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[inline(always)]
    pub fn data6(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, MbD6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, MbD6_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Mailbox Register"]
pub type MbD7 = crate::RegValueT<MbD7_SPEC>;

impl MbD7 {
    #[doc = "Data Bytes 7DATA7 store the transmitted or received CAN message data. Transmission or reception starts from DATA0. The bit order on the CAN bus is MSB first, and transmission or reception starts from bit 7."]
    #[inline(always)]
    pub fn data7(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, MbD7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, MbD7_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Mailbox Register"]
pub type MbTs = crate::RegValueT<MbTs_SPEC>;

impl MbTs {
    #[doc = "Time Stamp Lower ByteBits TSH\\[7:0\\] store the counter value of the time stamp when received messages are stored in the mailbox."]
    #[inline(always)]
    pub fn tsh(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, MbTs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, MbTs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Time Stamp Higher ByteBits TSL\\[7:0\\] store the counter value of the time stamp when received messages are stored in the mailbox."]
    #[inline(always)]
    pub fn tsl(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, MbTs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, MbTs_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Mask Register"]
pub type Mkr = crate::RegValueT<Mkr_SPEC>;

impl Mkr {
    #[doc = "Standard ID"]
    #[inline(always)]
    pub fn sid(
        self,
    ) -> crate::common::RegisterField<18, 0x7ff, 1, 0, u16, Mkr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x7ff,1,0,u16, Mkr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Extended ID"]
    #[inline(always)]
    pub fn eid(
        self,
    ) -> crate::common::RegisterField<0, 0x3ffff, 1, 0, u32, Mkr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ffff,1,0,u32, Mkr_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "FIFO Received ID Compare Registers"]
pub type Fidcr = crate::RegValueT<Fidcr_SPEC>;

impl Fidcr {
    #[doc = "ID Extension"]
    #[inline(always)]
    pub fn ide(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, fidcr::Ide, Fidcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,fidcr::Ide, Fidcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Remote Transmission Request"]
    #[inline(always)]
    pub fn rtr(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, fidcr::Rtr, Fidcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,fidcr::Rtr, Fidcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Standard ID"]
    #[inline(always)]
    pub fn sid(
        self,
    ) -> crate::common::RegisterField<18, 0x7ff, 1, 0, u16, Fidcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x7ff,1,0,u16, Fidcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Extended ID"]
    #[inline(always)]
    pub fn eid(
        self,
    ) -> crate::common::RegisterField<0, 0x3ffff, 1, 0, u32, Fidcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ffff,1,0,u32, Fidcr_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Standard ID"]
        pub const _0: Self = Self::new(0);
        #[doc = "Extended ID"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rtr_SPEC;
    pub type Rtr = crate::EnumBitfieldStruct<u8, Rtr_SPEC>;
    impl Rtr {
        #[doc = "Data frame"]
        pub const _0: Self = Self::new(0);
        #[doc = "Remote frame"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mkivlr_SPEC;
impl crate::sealed::RegSpec for Mkivlr_SPEC {
    type DataType = u32;
}
#[doc = "Mask Invalid Register"]
pub type Mkivlr = crate::RegValueT<Mkivlr_SPEC>;

impl Mkivlr {
    #[doc = "mailbox 31 Mask Invalid"]
    #[inline(always)]
    pub fn mb31(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, mkivlr::Mb31, Mkivlr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,mkivlr::Mb31, Mkivlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 30 Mask Invalid"]
    #[inline(always)]
    pub fn mb30(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, mkivlr::Mb30, Mkivlr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,mkivlr::Mb30, Mkivlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 29 Mask Invalid"]
    #[inline(always)]
    pub fn mb29(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, mkivlr::Mb29, Mkivlr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,mkivlr::Mb29, Mkivlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 28 Mask Invalid"]
    #[inline(always)]
    pub fn mb28(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, mkivlr::Mb28, Mkivlr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,mkivlr::Mb28, Mkivlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 27 Mask Invalid"]
    #[inline(always)]
    pub fn mb27(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, mkivlr::Mb27, Mkivlr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,mkivlr::Mb27, Mkivlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 26 Mask Invalid"]
    #[inline(always)]
    pub fn mb26(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, mkivlr::Mb26, Mkivlr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,mkivlr::Mb26, Mkivlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 25 Mask Invalid"]
    #[inline(always)]
    pub fn mb25(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, mkivlr::Mb25, Mkivlr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,mkivlr::Mb25, Mkivlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 24 Mask Invalid"]
    #[inline(always)]
    pub fn mb24(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, mkivlr::Mb24, Mkivlr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,mkivlr::Mb24, Mkivlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 23 Mask Invalid"]
    #[inline(always)]
    pub fn mb23(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, mkivlr::Mb23, Mkivlr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,mkivlr::Mb23, Mkivlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 22 Mask Invalid"]
    #[inline(always)]
    pub fn mb22(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, mkivlr::Mb22, Mkivlr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,mkivlr::Mb22, Mkivlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 21 Mask Invalid"]
    #[inline(always)]
    pub fn mb21(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, mkivlr::Mb21, Mkivlr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,mkivlr::Mb21, Mkivlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 20 Mask Invalid"]
    #[inline(always)]
    pub fn mb20(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, mkivlr::Mb20, Mkivlr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,mkivlr::Mb20, Mkivlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 19 Mask Invalid"]
    #[inline(always)]
    pub fn mb19(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, mkivlr::Mb19, Mkivlr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,mkivlr::Mb19, Mkivlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 18 Mask Invalid"]
    #[inline(always)]
    pub fn mb18(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, mkivlr::Mb18, Mkivlr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,mkivlr::Mb18, Mkivlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 17 Mask Invalid"]
    #[inline(always)]
    pub fn mb17(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, mkivlr::Mb17, Mkivlr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,mkivlr::Mb17, Mkivlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 16 Mask Invalid"]
    #[inline(always)]
    pub fn mb16(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, mkivlr::Mb16, Mkivlr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,mkivlr::Mb16, Mkivlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 15 Mask Invalid"]
    #[inline(always)]
    pub fn mb15(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, mkivlr::Mb15, Mkivlr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,mkivlr::Mb15, Mkivlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 14 Mask Invalid"]
    #[inline(always)]
    pub fn mb14(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, mkivlr::Mb14, Mkivlr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,mkivlr::Mb14, Mkivlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 13 Mask Invalid"]
    #[inline(always)]
    pub fn mb13(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, mkivlr::Mb13, Mkivlr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,mkivlr::Mb13, Mkivlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 12 Mask Invalid"]
    #[inline(always)]
    pub fn mb12(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, mkivlr::Mb12, Mkivlr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,mkivlr::Mb12, Mkivlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 11 Mask Invalid"]
    #[inline(always)]
    pub fn mb11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, mkivlr::Mb11, Mkivlr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,mkivlr::Mb11, Mkivlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 10 Mask Invalid"]
    #[inline(always)]
    pub fn mb10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, mkivlr::Mb10, Mkivlr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,mkivlr::Mb10, Mkivlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 9 Mask Invalid"]
    #[inline(always)]
    pub fn mb9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, mkivlr::Mb9, Mkivlr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,mkivlr::Mb9, Mkivlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 8 Mask Invalid"]
    #[inline(always)]
    pub fn mb8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, mkivlr::Mb8, Mkivlr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,mkivlr::Mb8, Mkivlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 7 Mask Invalid"]
    #[inline(always)]
    pub fn mb7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, mkivlr::Mb7, Mkivlr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,mkivlr::Mb7, Mkivlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 6 Mask Invalid"]
    #[inline(always)]
    pub fn mb6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, mkivlr::Mb6, Mkivlr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,mkivlr::Mb6, Mkivlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 5 Mask Invalid"]
    #[inline(always)]
    pub fn mb5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, mkivlr::Mb5, Mkivlr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,mkivlr::Mb5, Mkivlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 4 Mask Invalid"]
    #[inline(always)]
    pub fn mb4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, mkivlr::Mb4, Mkivlr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,mkivlr::Mb4, Mkivlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 3 Mask Invalid"]
    #[inline(always)]
    pub fn mb3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, mkivlr::Mb3, Mkivlr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,mkivlr::Mb3, Mkivlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 2 Mask Invalid"]
    #[inline(always)]
    pub fn mb2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, mkivlr::Mb2, Mkivlr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,mkivlr::Mb2, Mkivlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 1 Mask Invalid"]
    #[inline(always)]
    pub fn mb1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, mkivlr::Mb1, Mkivlr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,mkivlr::Mb1, Mkivlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 0 Mask Invalid"]
    #[inline(always)]
    pub fn mb0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, mkivlr::Mb0, Mkivlr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,mkivlr::Mb0, Mkivlr_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Mask valid"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mask invalid"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb30_SPEC;
    pub type Mb30 = crate::EnumBitfieldStruct<u8, Mb30_SPEC>;
    impl Mb30 {
        #[doc = "Mask valid"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mask invalid"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb29_SPEC;
    pub type Mb29 = crate::EnumBitfieldStruct<u8, Mb29_SPEC>;
    impl Mb29 {
        #[doc = "Mask valid"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mask invalid"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb28_SPEC;
    pub type Mb28 = crate::EnumBitfieldStruct<u8, Mb28_SPEC>;
    impl Mb28 {
        #[doc = "Mask valid"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mask invalid"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb27_SPEC;
    pub type Mb27 = crate::EnumBitfieldStruct<u8, Mb27_SPEC>;
    impl Mb27 {
        #[doc = "Mask valid"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mask invalid"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb26_SPEC;
    pub type Mb26 = crate::EnumBitfieldStruct<u8, Mb26_SPEC>;
    impl Mb26 {
        #[doc = "Mask valid"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mask invalid"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb25_SPEC;
    pub type Mb25 = crate::EnumBitfieldStruct<u8, Mb25_SPEC>;
    impl Mb25 {
        #[doc = "Mask valid"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mask invalid"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb24_SPEC;
    pub type Mb24 = crate::EnumBitfieldStruct<u8, Mb24_SPEC>;
    impl Mb24 {
        #[doc = "Mask valid"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mask invalid"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb23_SPEC;
    pub type Mb23 = crate::EnumBitfieldStruct<u8, Mb23_SPEC>;
    impl Mb23 {
        #[doc = "Mask valid"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mask invalid"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb22_SPEC;
    pub type Mb22 = crate::EnumBitfieldStruct<u8, Mb22_SPEC>;
    impl Mb22 {
        #[doc = "Mask valid"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mask invalid"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb21_SPEC;
    pub type Mb21 = crate::EnumBitfieldStruct<u8, Mb21_SPEC>;
    impl Mb21 {
        #[doc = "Mask valid"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mask invalid"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb20_SPEC;
    pub type Mb20 = crate::EnumBitfieldStruct<u8, Mb20_SPEC>;
    impl Mb20 {
        #[doc = "Mask valid"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mask invalid"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb19_SPEC;
    pub type Mb19 = crate::EnumBitfieldStruct<u8, Mb19_SPEC>;
    impl Mb19 {
        #[doc = "Mask valid"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mask invalid"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb18_SPEC;
    pub type Mb18 = crate::EnumBitfieldStruct<u8, Mb18_SPEC>;
    impl Mb18 {
        #[doc = "Mask valid"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mask invalid"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb17_SPEC;
    pub type Mb17 = crate::EnumBitfieldStruct<u8, Mb17_SPEC>;
    impl Mb17 {
        #[doc = "Mask valid"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mask invalid"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb16_SPEC;
    pub type Mb16 = crate::EnumBitfieldStruct<u8, Mb16_SPEC>;
    impl Mb16 {
        #[doc = "Mask valid"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mask invalid"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb15_SPEC;
    pub type Mb15 = crate::EnumBitfieldStruct<u8, Mb15_SPEC>;
    impl Mb15 {
        #[doc = "Mask valid"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mask invalid"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb14_SPEC;
    pub type Mb14 = crate::EnumBitfieldStruct<u8, Mb14_SPEC>;
    impl Mb14 {
        #[doc = "Mask valid"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mask invalid"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb13_SPEC;
    pub type Mb13 = crate::EnumBitfieldStruct<u8, Mb13_SPEC>;
    impl Mb13 {
        #[doc = "Mask valid"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mask invalid"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb12_SPEC;
    pub type Mb12 = crate::EnumBitfieldStruct<u8, Mb12_SPEC>;
    impl Mb12 {
        #[doc = "Mask valid"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mask invalid"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb11_SPEC;
    pub type Mb11 = crate::EnumBitfieldStruct<u8, Mb11_SPEC>;
    impl Mb11 {
        #[doc = "Mask valid"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mask invalid"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb10_SPEC;
    pub type Mb10 = crate::EnumBitfieldStruct<u8, Mb10_SPEC>;
    impl Mb10 {
        #[doc = "Mask valid"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mask invalid"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb9_SPEC;
    pub type Mb9 = crate::EnumBitfieldStruct<u8, Mb9_SPEC>;
    impl Mb9 {
        #[doc = "Mask valid"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mask invalid"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb8_SPEC;
    pub type Mb8 = crate::EnumBitfieldStruct<u8, Mb8_SPEC>;
    impl Mb8 {
        #[doc = "Mask valid"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mask invalid"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb7_SPEC;
    pub type Mb7 = crate::EnumBitfieldStruct<u8, Mb7_SPEC>;
    impl Mb7 {
        #[doc = "Mask valid"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mask invalid"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb6_SPEC;
    pub type Mb6 = crate::EnumBitfieldStruct<u8, Mb6_SPEC>;
    impl Mb6 {
        #[doc = "Mask valid"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mask invalid"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb5_SPEC;
    pub type Mb5 = crate::EnumBitfieldStruct<u8, Mb5_SPEC>;
    impl Mb5 {
        #[doc = "Mask valid"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mask invalid"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb4_SPEC;
    pub type Mb4 = crate::EnumBitfieldStruct<u8, Mb4_SPEC>;
    impl Mb4 {
        #[doc = "Mask valid"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mask invalid"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb3_SPEC;
    pub type Mb3 = crate::EnumBitfieldStruct<u8, Mb3_SPEC>;
    impl Mb3 {
        #[doc = "Mask valid"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mask invalid"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb2_SPEC;
    pub type Mb2 = crate::EnumBitfieldStruct<u8, Mb2_SPEC>;
    impl Mb2 {
        #[doc = "Mask valid"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mask invalid"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb1_SPEC;
    pub type Mb1 = crate::EnumBitfieldStruct<u8, Mb1_SPEC>;
    impl Mb1 {
        #[doc = "Mask valid"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mask invalid"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb0_SPEC;
    pub type Mb0 = crate::EnumBitfieldStruct<u8, Mb0_SPEC>;
    impl Mb0 {
        #[doc = "Mask valid"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mask invalid"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mier_SPEC;
impl crate::sealed::RegSpec for Mier_SPEC {
    type DataType = u32;
}
#[doc = "Mailbox Interrupt Enable Register"]
pub type Mier = crate::RegValueT<Mier_SPEC>;

impl Mier {
    #[doc = "mailbox 31 Interrupt Enable"]
    #[inline(always)]
    pub fn mb31(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, mier::Mb31, Mier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<31,0x1,1,0,mier::Mb31, Mier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 30 Interrupt Enable"]
    #[inline(always)]
    pub fn mb30(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, mier::Mb30, Mier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x1,1,0,mier::Mb30, Mier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 29 Interrupt Enable"]
    #[inline(always)]
    pub fn mb29(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, mier::Mb29, Mier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x1,1,0,mier::Mb29, Mier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 28 Interrupt Enable"]
    #[inline(always)]
    pub fn mb28(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, mier::Mb28, Mier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x1,1,0,mier::Mb28, Mier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 27 Interrupt Enable"]
    #[inline(always)]
    pub fn mb27(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, mier::Mb27, Mier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1,1,0,mier::Mb27, Mier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 26 Interrupt Enable"]
    #[inline(always)]
    pub fn mb26(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, mier::Mb26, Mier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x1,1,0,mier::Mb26, Mier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 25 Interrupt Enable"]
    #[inline(always)]
    pub fn mb25(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, mier::Mb25, Mier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<25,0x1,1,0,mier::Mb25, Mier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 24 Interrupt Enable"]
    #[inline(always)]
    pub fn mb24(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, mier::Mb24, Mier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1,1,0,mier::Mb24, Mier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 23 Interrupt Enable"]
    #[inline(always)]
    pub fn mb23(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, mier::Mb23, Mier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<23,0x1,1,0,mier::Mb23, Mier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 22 Interrupt Enable"]
    #[inline(always)]
    pub fn mb22(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, mier::Mb22, Mier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0x1,1,0,mier::Mb22, Mier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 21 Interrupt Enable"]
    #[inline(always)]
    pub fn mb21(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, mier::Mb21, Mier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<21,0x1,1,0,mier::Mb21, Mier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 20 Interrupt Enable"]
    #[inline(always)]
    pub fn mb20(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, mier::Mb20, Mier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x1,1,0,mier::Mb20, Mier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 19 Interrupt Enable"]
    #[inline(always)]
    pub fn mb19(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, mier::Mb19, Mier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<19,0x1,1,0,mier::Mb19, Mier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 18 Interrupt Enable"]
    #[inline(always)]
    pub fn mb18(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, mier::Mb18, Mier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x1,1,0,mier::Mb18, Mier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 17 Interrupt Enable"]
    #[inline(always)]
    pub fn mb17(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, mier::Mb17, Mier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<17,0x1,1,0,mier::Mb17, Mier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 16 Interrupt Enable"]
    #[inline(always)]
    pub fn mb16(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, mier::Mb16, Mier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1,1,0,mier::Mb16, Mier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 15 Interrupt Enable"]
    #[inline(always)]
    pub fn mb15(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, mier::Mb15, Mier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<15,0x1,1,0,mier::Mb15, Mier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 14 Interrupt Enable"]
    #[inline(always)]
    pub fn mb14(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, mier::Mb14, Mier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x1,1,0,mier::Mb14, Mier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 13 Interrupt Enable"]
    #[inline(always)]
    pub fn mb13(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, mier::Mb13, Mier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x1,1,0,mier::Mb13, Mier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 12 Interrupt Enable"]
    #[inline(always)]
    pub fn mb12(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, mier::Mb12, Mier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x1,1,0,mier::Mb12, Mier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 11 Interrupt Enable"]
    #[inline(always)]
    pub fn mb11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, mier::Mb11, Mier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x1,1,0,mier::Mb11, Mier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 10 Interrupt Enable"]
    #[inline(always)]
    pub fn mb10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, mier::Mb10, Mier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x1,1,0,mier::Mb10, Mier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 9 Interrupt Enable"]
    #[inline(always)]
    pub fn mb9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, mier::Mb9, Mier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x1,1,0,mier::Mb9, Mier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 8 Interrupt Enable"]
    #[inline(always)]
    pub fn mb8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, mier::Mb8, Mier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1,1,0,mier::Mb8, Mier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 7 Interrupt Enable"]
    #[inline(always)]
    pub fn mb7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, mier::Mb7, Mier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x1,1,0,mier::Mb7, Mier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 6 Interrupt Enable"]
    #[inline(always)]
    pub fn mb6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, mier::Mb6, Mier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x1,1,0,mier::Mb6, Mier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 5 Interrupt Enable"]
    #[inline(always)]
    pub fn mb5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, mier::Mb5, Mier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x1,1,0,mier::Mb5, Mier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 4 Interrupt Enable"]
    #[inline(always)]
    pub fn mb4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, mier::Mb4, Mier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,mier::Mb4, Mier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 3 Interrupt Enable"]
    #[inline(always)]
    pub fn mb3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, mier::Mb3, Mier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,mier::Mb3, Mier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 2 Interrupt Enable"]
    #[inline(always)]
    pub fn mb2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, mier::Mb2, Mier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,mier::Mb2, Mier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 1 Interrupt Enable"]
    #[inline(always)]
    pub fn mb1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, mier::Mb1, Mier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,mier::Mb1, Mier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 0 Interrupt Enable"]
    #[inline(always)]
    pub fn mb0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, mier::Mb0, Mier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,mier::Mb0, Mier_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb30_SPEC;
    pub type Mb30 = crate::EnumBitfieldStruct<u8, Mb30_SPEC>;
    impl Mb30 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb29_SPEC;
    pub type Mb29 = crate::EnumBitfieldStruct<u8, Mb29_SPEC>;
    impl Mb29 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb28_SPEC;
    pub type Mb28 = crate::EnumBitfieldStruct<u8, Mb28_SPEC>;
    impl Mb28 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb27_SPEC;
    pub type Mb27 = crate::EnumBitfieldStruct<u8, Mb27_SPEC>;
    impl Mb27 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb26_SPEC;
    pub type Mb26 = crate::EnumBitfieldStruct<u8, Mb26_SPEC>;
    impl Mb26 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb25_SPEC;
    pub type Mb25 = crate::EnumBitfieldStruct<u8, Mb25_SPEC>;
    impl Mb25 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb24_SPEC;
    pub type Mb24 = crate::EnumBitfieldStruct<u8, Mb24_SPEC>;
    impl Mb24 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb23_SPEC;
    pub type Mb23 = crate::EnumBitfieldStruct<u8, Mb23_SPEC>;
    impl Mb23 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb22_SPEC;
    pub type Mb22 = crate::EnumBitfieldStruct<u8, Mb22_SPEC>;
    impl Mb22 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb21_SPEC;
    pub type Mb21 = crate::EnumBitfieldStruct<u8, Mb21_SPEC>;
    impl Mb21 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb20_SPEC;
    pub type Mb20 = crate::EnumBitfieldStruct<u8, Mb20_SPEC>;
    impl Mb20 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb19_SPEC;
    pub type Mb19 = crate::EnumBitfieldStruct<u8, Mb19_SPEC>;
    impl Mb19 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb18_SPEC;
    pub type Mb18 = crate::EnumBitfieldStruct<u8, Mb18_SPEC>;
    impl Mb18 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb17_SPEC;
    pub type Mb17 = crate::EnumBitfieldStruct<u8, Mb17_SPEC>;
    impl Mb17 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb16_SPEC;
    pub type Mb16 = crate::EnumBitfieldStruct<u8, Mb16_SPEC>;
    impl Mb16 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb15_SPEC;
    pub type Mb15 = crate::EnumBitfieldStruct<u8, Mb15_SPEC>;
    impl Mb15 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb14_SPEC;
    pub type Mb14 = crate::EnumBitfieldStruct<u8, Mb14_SPEC>;
    impl Mb14 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb13_SPEC;
    pub type Mb13 = crate::EnumBitfieldStruct<u8, Mb13_SPEC>;
    impl Mb13 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb12_SPEC;
    pub type Mb12 = crate::EnumBitfieldStruct<u8, Mb12_SPEC>;
    impl Mb12 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb11_SPEC;
    pub type Mb11 = crate::EnumBitfieldStruct<u8, Mb11_SPEC>;
    impl Mb11 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb10_SPEC;
    pub type Mb10 = crate::EnumBitfieldStruct<u8, Mb10_SPEC>;
    impl Mb10 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb9_SPEC;
    pub type Mb9 = crate::EnumBitfieldStruct<u8, Mb9_SPEC>;
    impl Mb9 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb8_SPEC;
    pub type Mb8 = crate::EnumBitfieldStruct<u8, Mb8_SPEC>;
    impl Mb8 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb7_SPEC;
    pub type Mb7 = crate::EnumBitfieldStruct<u8, Mb7_SPEC>;
    impl Mb7 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb6_SPEC;
    pub type Mb6 = crate::EnumBitfieldStruct<u8, Mb6_SPEC>;
    impl Mb6 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb5_SPEC;
    pub type Mb5 = crate::EnumBitfieldStruct<u8, Mb5_SPEC>;
    impl Mb5 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb4_SPEC;
    pub type Mb4 = crate::EnumBitfieldStruct<u8, Mb4_SPEC>;
    impl Mb4 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb3_SPEC;
    pub type Mb3 = crate::EnumBitfieldStruct<u8, Mb3_SPEC>;
    impl Mb3 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb2_SPEC;
    pub type Mb2 = crate::EnumBitfieldStruct<u8, Mb2_SPEC>;
    impl Mb2 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb1_SPEC;
    pub type Mb1 = crate::EnumBitfieldStruct<u8, Mb1_SPEC>;
    impl Mb1 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb0_SPEC;
    pub type Mb0 = crate::EnumBitfieldStruct<u8, Mb0_SPEC>;
    impl Mb0 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MierFifo_SPEC;
impl crate::sealed::RegSpec for MierFifo_SPEC {
    type DataType = u32;
}
#[doc = "Mailbox Interrupt Enable Register for FIFO Mailbox Mode"]
pub type MierFifo = crate::RegValueT<MierFifo_SPEC>;

impl MierFifo {
    #[doc = "Receive FIFO Interrupt Generation Timing Control"]
    #[inline(always)]
    pub fn mb29(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
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
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Receive FIFO Interrupt Enable"]
    #[inline(always)]
    pub fn mb28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
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
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Transmit FIFO Interrupt Generation Timing Control"]
    #[inline(always)]
    pub fn mb25(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
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
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Transmit FIFO Interrupt Enable"]
    #[inline(always)]
    pub fn mb24(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
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
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "mailbox 23 Interrupt Enable"]
    #[inline(always)]
    pub fn mb23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
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
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "mailbox 22 Interrupt Enable"]
    #[inline(always)]
    pub fn mb22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
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
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "mailbox 21 Interrupt Enable"]
    #[inline(always)]
    pub fn mb21(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
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
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "mailbox 20 Interrupt Enable"]
    #[inline(always)]
    pub fn mb20(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
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
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "mailbox 19 Interrupt Enable"]
    #[inline(always)]
    pub fn mb19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
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
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "mailbox 18 Interrupt Enable"]
    #[inline(always)]
    pub fn mb18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
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
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "mailbox 17 Interrupt Enable"]
    #[inline(always)]
    pub fn mb17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
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
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "mailbox 16 Interrupt Enable"]
    #[inline(always)]
    pub fn mb16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
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
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "mailbox 15 Interrupt Enable"]
    #[inline(always)]
    pub fn mb15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
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
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "mailbox 14 Interrupt Enable"]
    #[inline(always)]
    pub fn mb14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
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
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "mailbox 13 Interrupt Enable"]
    #[inline(always)]
    pub fn mb13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
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
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "mailbox 12 Interrupt Enable"]
    #[inline(always)]
    pub fn mb12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
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
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "mailbox 11 Interrupt Enable"]
    #[inline(always)]
    pub fn mb11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
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
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "mailbox 10 Interrupt Enable"]
    #[inline(always)]
    pub fn mb10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
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
            MierFifo_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "mailbox 9 Interrupt Enable"]
    #[inline(always)]
    pub fn mb9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, mier_fifo::Mb9, MierFifo_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,mier_fifo::Mb9, MierFifo_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 8 Interrupt Enable"]
    #[inline(always)]
    pub fn mb8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, mier_fifo::Mb8, MierFifo_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,mier_fifo::Mb8, MierFifo_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 7 Interrupt Enable"]
    #[inline(always)]
    pub fn mb7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, mier_fifo::Mb7, MierFifo_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,mier_fifo::Mb7, MierFifo_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 6 Interrupt Enable"]
    #[inline(always)]
    pub fn mb6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, mier_fifo::Mb6, MierFifo_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,mier_fifo::Mb6, MierFifo_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 5 Interrupt Enable"]
    #[inline(always)]
    pub fn mb5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, mier_fifo::Mb5, MierFifo_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,mier_fifo::Mb5, MierFifo_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 4 Interrupt Enable"]
    #[inline(always)]
    pub fn mb4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, mier_fifo::Mb4, MierFifo_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,mier_fifo::Mb4, MierFifo_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 3 Interrupt Enable"]
    #[inline(always)]
    pub fn mb3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, mier_fifo::Mb3, MierFifo_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,mier_fifo::Mb3, MierFifo_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 2 Interrupt Enable"]
    #[inline(always)]
    pub fn mb2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, mier_fifo::Mb2, MierFifo_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,mier_fifo::Mb2, MierFifo_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 1 Interrupt Enable"]
    #[inline(always)]
    pub fn mb1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, mier_fifo::Mb1, MierFifo_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,mier_fifo::Mb1, MierFifo_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "mailbox 0 Interrupt Enable"]
    #[inline(always)]
    pub fn mb0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, mier_fifo::Mb0, MierFifo_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,mier_fifo::Mb0, MierFifo_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Every time reception is completed"]
        pub const _0: Self = Self::new(0);
        #[doc = "When the receive FIFO becomes buffer warning by completion of reception"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb28_SPEC;
    pub type Mb28 = crate::EnumBitfieldStruct<u8, Mb28_SPEC>;
    impl Mb28 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb25_SPEC;
    pub type Mb25 = crate::EnumBitfieldStruct<u8, Mb25_SPEC>;
    impl Mb25 {
        #[doc = "Every time transmission is completed"]
        pub const _0: Self = Self::new(0);
        #[doc = "When the transmit FIFO becomes empty due to completion of transmission"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb24_SPEC;
    pub type Mb24 = crate::EnumBitfieldStruct<u8, Mb24_SPEC>;
    impl Mb24 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb23_SPEC;
    pub type Mb23 = crate::EnumBitfieldStruct<u8, Mb23_SPEC>;
    impl Mb23 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb22_SPEC;
    pub type Mb22 = crate::EnumBitfieldStruct<u8, Mb22_SPEC>;
    impl Mb22 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb21_SPEC;
    pub type Mb21 = crate::EnumBitfieldStruct<u8, Mb21_SPEC>;
    impl Mb21 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb20_SPEC;
    pub type Mb20 = crate::EnumBitfieldStruct<u8, Mb20_SPEC>;
    impl Mb20 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb19_SPEC;
    pub type Mb19 = crate::EnumBitfieldStruct<u8, Mb19_SPEC>;
    impl Mb19 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb18_SPEC;
    pub type Mb18 = crate::EnumBitfieldStruct<u8, Mb18_SPEC>;
    impl Mb18 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb17_SPEC;
    pub type Mb17 = crate::EnumBitfieldStruct<u8, Mb17_SPEC>;
    impl Mb17 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb16_SPEC;
    pub type Mb16 = crate::EnumBitfieldStruct<u8, Mb16_SPEC>;
    impl Mb16 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb15_SPEC;
    pub type Mb15 = crate::EnumBitfieldStruct<u8, Mb15_SPEC>;
    impl Mb15 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb14_SPEC;
    pub type Mb14 = crate::EnumBitfieldStruct<u8, Mb14_SPEC>;
    impl Mb14 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb13_SPEC;
    pub type Mb13 = crate::EnumBitfieldStruct<u8, Mb13_SPEC>;
    impl Mb13 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb12_SPEC;
    pub type Mb12 = crate::EnumBitfieldStruct<u8, Mb12_SPEC>;
    impl Mb12 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb11_SPEC;
    pub type Mb11 = crate::EnumBitfieldStruct<u8, Mb11_SPEC>;
    impl Mb11 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb10_SPEC;
    pub type Mb10 = crate::EnumBitfieldStruct<u8, Mb10_SPEC>;
    impl Mb10 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb9_SPEC;
    pub type Mb9 = crate::EnumBitfieldStruct<u8, Mb9_SPEC>;
    impl Mb9 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb8_SPEC;
    pub type Mb8 = crate::EnumBitfieldStruct<u8, Mb8_SPEC>;
    impl Mb8 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb7_SPEC;
    pub type Mb7 = crate::EnumBitfieldStruct<u8, Mb7_SPEC>;
    impl Mb7 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb6_SPEC;
    pub type Mb6 = crate::EnumBitfieldStruct<u8, Mb6_SPEC>;
    impl Mb6 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb5_SPEC;
    pub type Mb5 = crate::EnumBitfieldStruct<u8, Mb5_SPEC>;
    impl Mb5 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb4_SPEC;
    pub type Mb4 = crate::EnumBitfieldStruct<u8, Mb4_SPEC>;
    impl Mb4 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb3_SPEC;
    pub type Mb3 = crate::EnumBitfieldStruct<u8, Mb3_SPEC>;
    impl Mb3 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb2_SPEC;
    pub type Mb2 = crate::EnumBitfieldStruct<u8, Mb2_SPEC>;
    impl Mb2 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb1_SPEC;
    pub type Mb1 = crate::EnumBitfieldStruct<u8, Mb1_SPEC>;
    impl Mb1 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mb0_SPEC;
    pub type Mb0 = crate::EnumBitfieldStruct<u8, Mb0_SPEC>;
    impl Mb0 {
        #[doc = "Interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MctlTx_SPEC;
impl crate::sealed::RegSpec for MctlTx_SPEC {
    type DataType = u8;
}
#[doc = "Message Control Register for Transmit"]
pub type MctlTx = crate::RegValueT<MctlTx_SPEC>;

impl MctlTx {
    #[doc = "Transmit Mailbox Request"]
    #[inline(always)]
    pub fn trmreq(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, mctl_tx::Trmreq, MctlTx_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,mctl_tx::Trmreq, MctlTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Receive Mailbox Request"]
    #[inline(always)]
    pub fn recreq(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, mctl_tx::Recreq, MctlTx_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,mctl_tx::Recreq, MctlTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "One-Shot Enable"]
    #[inline(always)]
    pub fn oneshot(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, mctl_tx::Oneshot, MctlTx_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,mctl_tx::Oneshot, MctlTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmission Abort Complete Flag (Transmit mailbox setting enabled)"]
    #[inline(always)]
    pub fn trmabt(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, mctl_tx::Trmabt, MctlTx_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,mctl_tx::Trmabt, MctlTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmission-in-Progress Status Flag (Transmit mailbox setting enabled)"]
    #[inline(always)]
    pub fn trmactive(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, mctl_tx::Trmactive, MctlTx_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mctl_tx::Trmactive,
            MctlTx_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Transmission Complete Flag"]
    #[inline(always)]
    pub fn sentdata(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, mctl_tx::Sentdata, MctlTx_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
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
        #[doc = "Not configured for transmission"]
        pub const _0: Self = Self::new(0);
        #[doc = "Configured for transmission"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Recreq_SPEC;
    pub type Recreq = crate::EnumBitfieldStruct<u8, Recreq_SPEC>;
    impl Recreq {
        #[doc = "Not configured for reception"]
        pub const _0: Self = Self::new(0);
        #[doc = "Configured for reception"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oneshot_SPEC;
    pub type Oneshot = crate::EnumBitfieldStruct<u8, Oneshot_SPEC>;
    impl Oneshot {
        #[doc = "One-shot reception or one-shot transmission disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "One-shot reception or one-shot transmission enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trmabt_SPEC;
    pub type Trmabt = crate::EnumBitfieldStruct<u8, Trmabt_SPEC>;
    impl Trmabt {
        #[doc = "Transmission has started, transmission abort failed because transmission is completed, or transmission abort is not requested"]
        pub const _0: Self = Self::new(0);
        #[doc = "Transmission abort is completed"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trmactive_SPEC;
    pub type Trmactive = crate::EnumBitfieldStruct<u8, Trmactive_SPEC>;
    impl Trmactive {
        #[doc = "Transmission is pending or transmission is not requested"]
        pub const _0: Self = Self::new(0);
        #[doc = "From acceptance of transmission request to completion of transmission, or error/arbitration-lost"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sentdata_SPEC;
    pub type Sentdata = crate::EnumBitfieldStruct<u8, Sentdata_SPEC>;
    impl Sentdata {
        #[doc = "Transmission is not completed"]
        pub const _0: Self = Self::new(0);
        #[doc = "Transmission is completed"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MctlRx_SPEC;
impl crate::sealed::RegSpec for MctlRx_SPEC {
    type DataType = u8;
}
#[doc = "Message Control Register for Receive"]
pub type MctlRx = crate::RegValueT<MctlRx_SPEC>;

impl MctlRx {
    #[doc = "Transmit Mailbox Request"]
    #[inline(always)]
    pub fn trmreq(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, mctl_rx::Trmreq, MctlRx_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,mctl_rx::Trmreq, MctlRx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Receive Mailbox Request"]
    #[inline(always)]
    pub fn recreq(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, mctl_rx::Recreq, MctlRx_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,mctl_rx::Recreq, MctlRx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "One-Shot Enable"]
    #[inline(always)]
    pub fn oneshot(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, mctl_rx::Oneshot, MctlRx_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,mctl_rx::Oneshot, MctlRx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Lost Flag(Receive mailbox setting enabled)"]
    #[inline(always)]
    pub fn msglost(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, mctl_rx::Msglost, MctlRx_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,mctl_rx::Msglost, MctlRx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Reception-in-Progress Status Flag (Receive mailbox setting enabled)"]
    #[inline(always)]
    pub fn invaldata(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, mctl_rx::Invaldata, MctlRx_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mctl_rx::Invaldata,
            MctlRx_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Reception Complete Flag"]
    #[inline(always)]
    pub fn newdata(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, mctl_rx::Newdata, MctlRx_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,mctl_rx::Newdata, MctlRx_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Not configured for transmission"]
        pub const _0: Self = Self::new(0);
        #[doc = "Configured for transmission"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Recreq_SPEC;
    pub type Recreq = crate::EnumBitfieldStruct<u8, Recreq_SPEC>;
    impl Recreq {
        #[doc = "Not configured for reception"]
        pub const _0: Self = Self::new(0);
        #[doc = "Configured for reception"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oneshot_SPEC;
    pub type Oneshot = crate::EnumBitfieldStruct<u8, Oneshot_SPEC>;
    impl Oneshot {
        #[doc = "One-shot reception or one-shot transmission disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "One-shot reception or one-shot transmission enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Msglost_SPEC;
    pub type Msglost = crate::EnumBitfieldStruct<u8, Msglost_SPEC>;
    impl Msglost {
        #[doc = "Message is not overwritten or overrun"]
        pub const _0: Self = Self::new(0);
        #[doc = "Message is overwritten or overrun"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Invaldata_SPEC;
    pub type Invaldata = crate::EnumBitfieldStruct<u8, Invaldata_SPEC>;
    impl Invaldata {
        #[doc = "Message valid"]
        pub const _0: Self = Self::new(0);
        #[doc = "Message being updated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Newdata_SPEC;
    pub type Newdata = crate::EnumBitfieldStruct<u8, Newdata_SPEC>;
    impl Newdata {
        #[doc = "No data has been received or 0 is written to the NEWDATA bit"]
        pub const _0: Self = Self::new(0);
        #[doc = "A new message is being stored or has been stored to the mailbox"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctlr_SPEC;
impl crate::sealed::RegSpec for Ctlr_SPEC {
    type DataType = u16;
}
#[doc = "Control Register"]
pub type Ctlr = crate::RegValueT<Ctlr_SPEC>;

impl Ctlr {
    #[doc = "Forcible Return From Bus-Off"]
    #[inline(always)]
    pub fn rboc(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, ctlr::Rboc, Ctlr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x1,1,0,ctlr::Rboc, Ctlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bus-Off Recovery Mode by a program request"]
    #[inline(always)]
    pub fn bom(
        self,
    ) -> crate::common::RegisterField<11, 0x3, 1, 0, ctlr::Bom, Ctlr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x3,1,0,ctlr::Bom, Ctlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CAN Sleep Mode"]
    #[inline(always)]
    pub fn slpm(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, ctlr::Slpm, Ctlr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x1,1,0,ctlr::Slpm, Ctlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CAN Operating Mode Select"]
    #[inline(always)]
    pub fn canm(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, ctlr::Canm, Ctlr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,ctlr::Canm, Ctlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Time Stamp Prescaler Select"]
    #[inline(always)]
    pub fn tsps(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, ctlr::Tsps, Ctlr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,ctlr::Tsps, Ctlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Time Stamp Counter Reset Command"]
    #[inline(always)]
    pub fn tsrc(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, ctlr::Tsrc, Ctlr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x1,1,0,ctlr::Tsrc, Ctlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmission Priority Mode Select"]
    #[inline(always)]
    pub fn tpm(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, ctlr::Tpm, Ctlr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,ctlr::Tpm, Ctlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Lost Mode Select"]
    #[inline(always)]
    pub fn mlm(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, ctlr::Mlm, Ctlr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,ctlr::Mlm, Ctlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ID Format Mode Select"]
    #[inline(always)]
    pub fn idfm(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, ctlr::Idfm, Ctlr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x3,1,0,ctlr::Idfm, Ctlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CAN Mailbox Mode Select"]
    #[inline(always)]
    pub fn mbm(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, ctlr::Mbm, Ctlr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,ctlr::Mbm, Ctlr_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Nothing occurred"]
        pub const _0: Self = Self::new(0);
        #[doc = "Forcible return from bus-off"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bom_SPEC;
    pub type Bom = crate::EnumBitfieldStruct<u8, Bom_SPEC>;
    impl Bom {
        #[doc = "Normal mode (ISO11898-1 compliant)"]
        pub const _00: Self = Self::new(0);
        #[doc = "Entry to CAN halt mode automatically at bus-off entry"]
        pub const _01: Self = Self::new(1);
        #[doc = "Entry to CAN halt mode automatically at bus-off end"]
        pub const _10: Self = Self::new(2);
        #[doc = "Entry to CAN halt mode (during bus-off recovery period)"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slpm_SPEC;
    pub type Slpm = crate::EnumBitfieldStruct<u8, Slpm_SPEC>;
    impl Slpm {
        #[doc = "Other than CAN sleep mode"]
        pub const _0: Self = Self::new(0);
        #[doc = "CAN sleep mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Canm_SPEC;
    pub type Canm = crate::EnumBitfieldStruct<u8, Canm_SPEC>;
    impl Canm {
        #[doc = "CAN operation mode"]
        pub const _00: Self = Self::new(0);
        #[doc = "CAN reset mode"]
        pub const _01: Self = Self::new(1);
        #[doc = "CAN halt mode"]
        pub const _10: Self = Self::new(2);
        #[doc = "CAN reset mode (forcible transition)"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsps_SPEC;
    pub type Tsps = crate::EnumBitfieldStruct<u8, Tsps_SPEC>;
    impl Tsps {
        #[doc = "Every bit time"]
        pub const _00: Self = Self::new(0);
        #[doc = "Every 2-bit time"]
        pub const _01: Self = Self::new(1);
        #[doc = "Every 4-bit time"]
        pub const _10: Self = Self::new(2);
        #[doc = "Every 8-bit time"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsrc_SPEC;
    pub type Tsrc = crate::EnumBitfieldStruct<u8, Tsrc_SPEC>;
    impl Tsrc {
        #[doc = "Nothing occurred"]
        pub const _0: Self = Self::new(0);
        #[doc = "Reset"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tpm_SPEC;
    pub type Tpm = crate::EnumBitfieldStruct<u8, Tpm_SPEC>;
    impl Tpm {
        #[doc = "ID priority transmit mode"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mailbox number priority transmit mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mlm_SPEC;
    pub type Mlm = crate::EnumBitfieldStruct<u8, Mlm_SPEC>;
    impl Mlm {
        #[doc = "Overwrite mode"]
        pub const _0: Self = Self::new(0);
        #[doc = "Overrun mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Idfm_SPEC;
    pub type Idfm = crate::EnumBitfieldStruct<u8, Idfm_SPEC>;
    impl Idfm {
        #[doc = "Standard ID mode.All mailboxes (including FIFO mailboxes) handle only standard Ids."]
        pub const _00: Self = Self::new(0);
        #[doc = "Extended ID mode.All mailboxes (including FIFO mailboxes) handle only extended IDs."]
        pub const _01: Self = Self::new(1);
        #[doc = "Mixed ID mode.All mailboxes (including FIFO mailboxes) handle both standard IDs and extended IDs. Standard IDs or extended IDs are specified by using the IDE bit in the corresponding mailbox in normal mailbox mode. In FIFO mailbox mode, the IDE bit in the corresponding mailbox is used for mailboxes \\[0\\] to \\[23\\], the IDE bits in FIDCR0 and FIDCR1 are used for the receive FIFO, and the IDE bit in mailbox \\[24\\] is used for the transmit FIFO."]
        pub const _10: Self = Self::new(2);
        #[doc = "Do not use this combination"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mbm_SPEC;
    pub type Mbm = crate::EnumBitfieldStruct<u8, Mbm_SPEC>;
    impl Mbm {
        #[doc = "Normal mailbox mode"]
        pub const _0: Self = Self::new(0);
        #[doc = "FIFO mailbox mode"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Str_SPEC;
impl crate::sealed::RegSpec for Str_SPEC {
    type DataType = u16;
}
#[doc = "Status Register"]
pub type Str = crate::RegValueT<Str_SPEC>;

impl Str {
    #[doc = "Receive Status Flag (receiver)"]
    #[inline(always)]
    pub fn recst(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, str::Recst, Str_SPEC, crate::common::R> {
        crate::common::RegisterField::<14,0x1,1,0,str::Recst, Str_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmit Status Flag (transmitter)"]
    #[inline(always)]
    pub fn trmst(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, str::Trmst, Str_SPEC, crate::common::R> {
        crate::common::RegisterField::<13,0x1,1,0,str::Trmst, Str_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Bus-Off Status Flag"]
    #[inline(always)]
    pub fn bost(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, str::Bost, Str_SPEC, crate::common::R> {
        crate::common::RegisterField::<12,0x1,1,0,str::Bost, Str_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Error-Passive Status Flag"]
    #[inline(always)]
    pub fn epst(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, str::Epst, Str_SPEC, crate::common::R> {
        crate::common::RegisterField::<11,0x1,1,0,str::Epst, Str_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "CAN Sleep Status Flag"]
    #[inline(always)]
    pub fn slpst(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, str::Slpst, Str_SPEC, crate::common::R> {
        crate::common::RegisterField::<10,0x1,1,0,str::Slpst, Str_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "CAN Halt Status Flag"]
    #[inline(always)]
    pub fn hltst(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, str::Hltst, Str_SPEC, crate::common::R> {
        crate::common::RegisterField::<9,0x1,1,0,str::Hltst, Str_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "CAN Reset Status Flag"]
    #[inline(always)]
    pub fn rstst(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, str::Rstst, Str_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x1,1,0,str::Rstst, Str_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Error Status Flag"]
    #[inline(always)]
    pub fn est(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, str::Est, Str_SPEC, crate::common::R> {
        crate::common::RegisterField::<7,0x1,1,0,str::Est, Str_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmission Abort Status Flag"]
    #[inline(always)]
    pub fn tabst(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, str::Tabst, Str_SPEC, crate::common::R> {
        crate::common::RegisterField::<6,0x1,1,0,str::Tabst, Str_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "FIFO Mailbox Message Lost Status Flag"]
    #[inline(always)]
    pub fn fmlst(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, str::Fmlst, Str_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x1,1,0,str::Fmlst, Str_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Normal Mailbox Message Lost Status Flag"]
    #[inline(always)]
    pub fn nmlst(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, str::Nmlst, Str_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x1,1,0,str::Nmlst, Str_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmit FIFO Status Flag"]
    #[inline(always)]
    pub fn tfst(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, str::Tfst, Str_SPEC, crate::common::R> {
        crate::common::RegisterField::<3,0x1,1,0,str::Tfst, Str_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Receive FIFO Status Flag"]
    #[inline(always)]
    pub fn rfst(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, str::Rfst, Str_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x1,1,0,str::Rfst, Str_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "SENTDATA Status Flag"]
    #[inline(always)]
    pub fn sdst(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, str::Sdst, Str_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,str::Sdst, Str_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "NEWDATA Status Flag"]
    #[inline(always)]
    pub fn ndst(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, str::Ndst, Str_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1,1,0,str::Ndst, Str_SPEC,crate::common::R>::from_register(self,0)
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
        #[doc = "Bus idle or transmission in progress"]
        pub const _0: Self = Self::new(0);
        #[doc = "Reception in progress"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trmst_SPEC;
    pub type Trmst = crate::EnumBitfieldStruct<u8, Trmst_SPEC>;
    impl Trmst {
        #[doc = "Bus idle or reception in progress"]
        pub const _0: Self = Self::new(0);
        #[doc = "Transmission in progress or in bus-off state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bost_SPEC;
    pub type Bost = crate::EnumBitfieldStruct<u8, Bost_SPEC>;
    impl Bost {
        #[doc = "Not in bus-off state"]
        pub const _0: Self = Self::new(0);
        #[doc = "In bus-off state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Epst_SPEC;
    pub type Epst = crate::EnumBitfieldStruct<u8, Epst_SPEC>;
    impl Epst {
        #[doc = "Not in error-passive state"]
        pub const _0: Self = Self::new(0);
        #[doc = "In error-passive state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Slpst_SPEC;
    pub type Slpst = crate::EnumBitfieldStruct<u8, Slpst_SPEC>;
    impl Slpst {
        #[doc = "Not in CAN sleep mode"]
        pub const _0: Self = Self::new(0);
        #[doc = "In CAN sleep mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hltst_SPEC;
    pub type Hltst = crate::EnumBitfieldStruct<u8, Hltst_SPEC>;
    impl Hltst {
        #[doc = "Not in CAN halt mode"]
        pub const _0: Self = Self::new(0);
        #[doc = "In CAN halt mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rstst_SPEC;
    pub type Rstst = crate::EnumBitfieldStruct<u8, Rstst_SPEC>;
    impl Rstst {
        #[doc = "Not in CAN reset mode"]
        pub const _0: Self = Self::new(0);
        #[doc = "In CAN reset mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Est_SPEC;
    pub type Est = crate::EnumBitfieldStruct<u8, Est_SPEC>;
    impl Est {
        #[doc = "No error occurred"]
        pub const _0: Self = Self::new(0);
        #[doc = "Error occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tabst_SPEC;
    pub type Tabst = crate::EnumBitfieldStruct<u8, Tabst_SPEC>;
    impl Tabst {
        #[doc = "No mailbox with TRMABT bit = 1"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mailbox(es) with TRMABT bit = 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fmlst_SPEC;
    pub type Fmlst = crate::EnumBitfieldStruct<u8, Fmlst_SPEC>;
    impl Fmlst {
        #[doc = "RFMLF bit = 0"]
        pub const _0: Self = Self::new(0);
        #[doc = "RFMLF bit = 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nmlst_SPEC;
    pub type Nmlst = crate::EnumBitfieldStruct<u8, Nmlst_SPEC>;
    impl Nmlst {
        #[doc = "No mailbox with MSGLOST bit = 1"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mailbox(es) with MSGLOST bit = 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tfst_SPEC;
    pub type Tfst = crate::EnumBitfieldStruct<u8, Tfst_SPEC>;
    impl Tfst {
        #[doc = "Transmit FIFO is full"]
        pub const _0: Self = Self::new(0);
        #[doc = "Transmit FIFO is not full"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfst_SPEC;
    pub type Rfst = crate::EnumBitfieldStruct<u8, Rfst_SPEC>;
    impl Rfst {
        #[doc = "No message in receive FIFO (empty)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Message in receive FIFO"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdst_SPEC;
    pub type Sdst = crate::EnumBitfieldStruct<u8, Sdst_SPEC>;
    impl Sdst {
        #[doc = "No mailbox with SENTDATA bit = 1"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mailbox(es) with SENTDATA bit = 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ndst_SPEC;
    pub type Ndst = crate::EnumBitfieldStruct<u8, Ndst_SPEC>;
    impl Ndst {
        #[doc = "No mailbox with NEWDATA bit = 1"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mailbox(es) with NEWDATA bit = 1"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcr_SPEC;
impl crate::sealed::RegSpec for Bcr_SPEC {
    type DataType = u32;
}
#[doc = "Bit Configuration Register"]
pub type Bcr = crate::RegValueT<Bcr_SPEC>;

impl Bcr {
    #[doc = "Time Segment 1 Control"]
    #[inline(always)]
    pub fn tseg1(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, bcr::Tseg1, Bcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0xf,1,0,bcr::Tseg1, Bcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Prescaler Division Ratio Select . These bits set the frequency of the CAN communication clock (fCANCLK)."]
    #[inline(always)]
    pub fn brp(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, Bcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3ff,1,0,u16, Bcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Resynchronization Jump Width Control"]
    #[inline(always)]
    pub fn sjw(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, bcr::Sjw, Bcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,bcr::Sjw, Bcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Time Segment 2 Control"]
    #[inline(always)]
    pub fn tseg2(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, bcr::Tseg2, Bcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,bcr::Tseg2, Bcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CAN Clock Source Selection"]
    #[inline(always)]
    pub fn cclks(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, bcr::Cclks, Bcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,bcr::Cclks, Bcr_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Setting prohibited"]
        pub const _0000: Self = Self::new(0);
        #[doc = "Setting prohibited"]
        pub const _0001: Self = Self::new(1);
        #[doc = "Setting prohibited"]
        pub const _0010: Self = Self::new(2);
        #[doc = "4 Tq"]
        pub const _0011: Self = Self::new(3);
        #[doc = "5 Tq"]
        pub const _0100: Self = Self::new(4);
        #[doc = "6 Tq"]
        pub const _0101: Self = Self::new(5);
        #[doc = "7 Tq"]
        pub const _0110: Self = Self::new(6);
        #[doc = "8 Tq"]
        pub const _0111: Self = Self::new(7);
        #[doc = "9 Tq"]
        pub const _1000: Self = Self::new(8);
        #[doc = "10 Tq"]
        pub const _1001: Self = Self::new(9);
        #[doc = "11 Tq"]
        pub const _1010: Self = Self::new(10);
        #[doc = "12 Tq"]
        pub const _1011: Self = Self::new(11);
        #[doc = "13 Tq"]
        pub const _1100: Self = Self::new(12);
        #[doc = "14 Tq"]
        pub const _1101: Self = Self::new(13);
        #[doc = "15 Tq"]
        pub const _1110: Self = Self::new(14);
        #[doc = "16 Tq"]
        pub const _1111: Self = Self::new(15);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sjw_SPEC;
    pub type Sjw = crate::EnumBitfieldStruct<u8, Sjw_SPEC>;
    impl Sjw {
        #[doc = "1 Tq"]
        pub const _00: Self = Self::new(0);
        #[doc = "2 Tq"]
        pub const _01: Self = Self::new(1);
        #[doc = "3 Tq"]
        pub const _10: Self = Self::new(2);
        #[doc = "4 Tq"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tseg2_SPEC;
    pub type Tseg2 = crate::EnumBitfieldStruct<u8, Tseg2_SPEC>;
    impl Tseg2 {
        #[doc = "Setting prohibited"]
        pub const _000: Self = Self::new(0);
        #[doc = "2 Tq"]
        pub const _001: Self = Self::new(1);
        #[doc = "3 Tq"]
        pub const _010: Self = Self::new(2);
        #[doc = "4 Tq"]
        pub const _011: Self = Self::new(3);
        #[doc = "5 Tq"]
        pub const _100: Self = Self::new(4);
        #[doc = "6 Tq"]
        pub const _101: Self = Self::new(5);
        #[doc = "7 Tq"]
        pub const _110: Self = Self::new(6);
        #[doc = "8 Tq"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclks_SPEC;
    pub type Cclks = crate::EnumBitfieldStruct<u8, Cclks_SPEC>;
    impl Cclks {
        #[doc = "PCLK (generated by the PLL clock)"]
        pub const _0: Self = Self::new(0);
        #[doc = "CANMCLK (generated by the main clock)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rfcr_SPEC;
impl crate::sealed::RegSpec for Rfcr_SPEC {
    type DataType = u8;
}
#[doc = "Receive FIFO Control Register"]
pub type Rfcr = crate::RegValueT<Rfcr_SPEC>;

impl Rfcr {
    #[doc = "Receive FIFO Empty Status Flag"]
    #[inline(always)]
    pub fn rfest(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, rfcr::Rfest, Rfcr_SPEC, crate::common::R> {
        crate::common::RegisterField::<7,0x1,1,0,rfcr::Rfest, Rfcr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Receive FIFO Buffer Warning Status Flag"]
    #[inline(always)]
    pub fn rfwst(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, rfcr::Rfwst, Rfcr_SPEC, crate::common::R> {
        crate::common::RegisterField::<6,0x1,1,0,rfcr::Rfwst, Rfcr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Receive FIFO Full Status Flag"]
    #[inline(always)]
    pub fn rffst(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, rfcr::Rffst, Rfcr_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x1,1,0,rfcr::Rffst, Rfcr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Receive FIFO Message Lost Flag"]
    #[inline(always)]
    pub fn rfmlf(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, rfcr::Rfmlf, Rfcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,rfcr::Rfmlf, Rfcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Receive FIFO Unread Message Number Status"]
    #[inline(always)]
    pub fn rfust(
        self,
    ) -> crate::common::RegisterField<1, 0x7, 1, 0, rfcr::Rfust, Rfcr_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x7,1,0,rfcr::Rfust, Rfcr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Receive FIFO Enable"]
    #[inline(always)]
    pub fn rfe(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, rfcr::Rfe, Rfcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,rfcr::Rfe, Rfcr_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Unread message in receive FIFO"]
        pub const _0: Self = Self::new(0);
        #[doc = "No unread message in receive FIFO"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfwst_SPEC;
    pub type Rfwst = crate::EnumBitfieldStruct<u8, Rfwst_SPEC>;
    impl Rfwst {
        #[doc = "Receive FIFO is not buffer warning"]
        pub const _0: Self = Self::new(0);
        #[doc = "Receive FIFO is buffer warning (3 unread messages)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rffst_SPEC;
    pub type Rffst = crate::EnumBitfieldStruct<u8, Rffst_SPEC>;
    impl Rffst {
        #[doc = "Receive FIFO is not full"]
        pub const _0: Self = Self::new(0);
        #[doc = "Receive FIFO is full (4 unread messages)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfmlf_SPEC;
    pub type Rfmlf = crate::EnumBitfieldStruct<u8, Rfmlf_SPEC>;
    impl Rfmlf {
        #[doc = "No receive FIFO message lost has occurred"]
        pub const _0: Self = Self::new(0);
        #[doc = "Receive FIFO message lost has occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfust_SPEC;
    pub type Rfust = crate::EnumBitfieldStruct<u8, Rfust_SPEC>;
    impl Rfust {
        #[doc = "No unread message"]
        pub const _000: Self = Self::new(0);
        #[doc = "1 unread message"]
        pub const _001: Self = Self::new(1);
        #[doc = "2 unread messages"]
        pub const _010: Self = Self::new(2);
        #[doc = "3 unread messages"]
        pub const _011: Self = Self::new(3);
        #[doc = "4 unread messages"]
        pub const _100: Self = Self::new(4);
        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfe_SPEC;
    pub type Rfe = crate::EnumBitfieldStruct<u8, Rfe_SPEC>;
    impl Rfe {
        #[doc = "Receive FIFO disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Receive FIFO enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rfpcr_SPEC;
impl crate::sealed::RegSpec for Rfpcr_SPEC {
    type DataType = u8;
}
#[doc = "Receive FIFO Pointer Control Register"]
pub type Rfpcr = crate::RegValueT<Rfpcr_SPEC>;

impl Rfpcr {
    #[doc = "The CPU-side pointer for the receive FIFO is incremented by writing FFh to RFPCR."]
    #[inline(always)]
    pub fn rfpcr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Rfpcr_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Rfpcr_SPEC,crate::common::W>::from_register(self,0)
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
#[doc = "Transmit FIFO Control Register"]
pub type Tfcr = crate::RegValueT<Tfcr_SPEC>;

impl Tfcr {
    #[doc = "Transmit FIFO Empty Status"]
    #[inline(always)]
    pub fn tfest(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, tfcr::Tfest, Tfcr_SPEC, crate::common::R> {
        crate::common::RegisterField::<7,0x1,1,0,tfcr::Tfest, Tfcr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmit FIFO Full Status"]
    #[inline(always)]
    pub fn tffst(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, tfcr::Tffst, Tfcr_SPEC, crate::common::R> {
        crate::common::RegisterField::<6,0x1,1,0,tfcr::Tffst, Tfcr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmit FIFO Unsent Message Number Status"]
    #[inline(always)]
    pub fn tfust(
        self,
    ) -> crate::common::RegisterField<1, 0x7, 1, 0, tfcr::Tfust, Tfcr_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x7,1,0,tfcr::Tfust, Tfcr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmit FIFO Enable"]
    #[inline(always)]
    pub fn tfe(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, tfcr::Tfe, Tfcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,tfcr::Tfe, Tfcr_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Unsent message in transmit FIFO"]
        pub const _0: Self = Self::new(0);
        #[doc = "No unsent message in transmit FIFO"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tffst_SPEC;
    pub type Tffst = crate::EnumBitfieldStruct<u8, Tffst_SPEC>;
    impl Tffst {
        #[doc = "Transmit FIFO is not full"]
        pub const _0: Self = Self::new(0);
        #[doc = "Transmit FIFO is full (4 unsent messages)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tfust_SPEC;
    pub type Tfust = crate::EnumBitfieldStruct<u8, Tfust_SPEC>;
    impl Tfust {
        #[doc = "No unsent message"]
        pub const _000: Self = Self::new(0);
        #[doc = "1 unsent message"]
        pub const _001: Self = Self::new(1);
        #[doc = "2 unsent messages"]
        pub const _010: Self = Self::new(2);
        #[doc = "3 unsent messages"]
        pub const _011: Self = Self::new(3);
        #[doc = "4 unsent messages"]
        pub const _100: Self = Self::new(4);
        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tfe_SPEC;
    pub type Tfe = crate::EnumBitfieldStruct<u8, Tfe_SPEC>;
    impl Tfe {
        #[doc = "Transmit FIFO disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Transmit FIFO enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tfpcr_SPEC;
impl crate::sealed::RegSpec for Tfpcr_SPEC {
    type DataType = u8;
}
#[doc = "Transmit FIFO Pointer Control Register"]
pub type Tfpcr = crate::RegValueT<Tfpcr_SPEC>;

impl Tfpcr {
    #[doc = "The CPU-side pointer for the transmit FIFO is incremented by writing FFh to TFPCR."]
    #[inline(always)]
    pub fn tfpcr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Tfpcr_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Tfpcr_SPEC,crate::common::W>::from_register(self,0)
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
#[doc = "Error Interrupt Enable Register"]
pub type Eier = crate::RegValueT<Eier_SPEC>;

impl Eier {
    #[doc = "Bus Lock Interrupt Enable"]
    #[inline(always)]
    pub fn blie(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, eier::Blie, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x1,1,0,eier::Blie, Eier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Overload Frame Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn olie(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, eier::Olie, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x1,1,0,eier::Olie, Eier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn orie(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, eier::Orie, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x1,1,0,eier::Orie, Eier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bus-Off Recovery Interrupt Enable"]
    #[inline(always)]
    pub fn borie(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, eier::Borie, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,eier::Borie, Eier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bus-Off Entry Interrupt Enable"]
    #[inline(always)]
    pub fn boeie(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, eier::Boeie, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,eier::Boeie, Eier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Error-Passive Interrupt Enable"]
    #[inline(always)]
    pub fn epie(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, eier::Epie, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,eier::Epie, Eier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Error-Warning Interrupt Enable"]
    #[inline(always)]
    pub fn ewie(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, eier::Ewie, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,eier::Ewie, Eier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bus Error Interrupt Enable"]
    #[inline(always)]
    pub fn beie(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, eier::Beie, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,eier::Beie, Eier_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Bus lock interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Bus lock interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Olie_SPEC;
    pub type Olie = crate::EnumBitfieldStruct<u8, Olie_SPEC>;
    impl Olie {
        #[doc = "Overload frame transmit interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Overload frame transmit interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Orie_SPEC;
    pub type Orie = crate::EnumBitfieldStruct<u8, Orie_SPEC>;
    impl Orie {
        #[doc = "Receive overrun interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Receive overrun interrupt enabled"]
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
    pub struct Boeie_SPEC;
    pub type Boeie = crate::EnumBitfieldStruct<u8, Boeie_SPEC>;
    impl Boeie {
        #[doc = "Bus-off entry interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Bus-off entry interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Epie_SPEC;
    pub type Epie = crate::EnumBitfieldStruct<u8, Epie_SPEC>;
    impl Epie {
        #[doc = "Error-passive interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Error-passive interrupt enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ewie_SPEC;
    pub type Ewie = crate::EnumBitfieldStruct<u8, Ewie_SPEC>;
    impl Ewie {
        #[doc = "Error-warning interrupt disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Error-warning interrupt enabled"]
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
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eifr_SPEC;
impl crate::sealed::RegSpec for Eifr_SPEC {
    type DataType = u8;
}
#[doc = "Error Interrupt Factor Judge Register"]
pub type Eifr = crate::RegValueT<Eifr_SPEC>;

impl Eifr {
    #[doc = "Bus Lock Detect Flag"]
    #[inline(always)]
    pub fn blif(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, eifr::Blif, Eifr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x1,1,0,eifr::Blif, Eifr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Overload Frame Transmission Detect Flag"]
    #[inline(always)]
    pub fn olif(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, eifr::Olif, Eifr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x1,1,0,eifr::Olif, Eifr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Receive Overrun Detect Flag"]
    #[inline(always)]
    pub fn orif(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, eifr::Orif, Eifr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x1,1,0,eifr::Orif, Eifr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bus-Off Recovery Detect Flag"]
    #[inline(always)]
    pub fn borif(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, eifr::Borif, Eifr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,eifr::Borif, Eifr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bus-Off Entry Detect Flag"]
    #[inline(always)]
    pub fn boeif(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, eifr::Boeif, Eifr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,eifr::Boeif, Eifr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Error-Passive Detect Flag"]
    #[inline(always)]
    pub fn epif(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, eifr::Epif, Eifr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,eifr::Epif, Eifr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Error-Warning Detect Flag"]
    #[inline(always)]
    pub fn ewif(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, eifr::Ewif, Eifr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,eifr::Ewif, Eifr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bus Error Detect Flag"]
    #[inline(always)]
    pub fn beif(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, eifr::Beif, Eifr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,eifr::Beif, Eifr_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "No bus lock detected"]
        pub const _0: Self = Self::new(0);
        #[doc = "Bus lock detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Olif_SPEC;
    pub type Olif = crate::EnumBitfieldStruct<u8, Olif_SPEC>;
    impl Olif {
        #[doc = "No overload frame transmission detected"]
        pub const _0: Self = Self::new(0);
        #[doc = "Overload frame transmission detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Orif_SPEC;
    pub type Orif = crate::EnumBitfieldStruct<u8, Orif_SPEC>;
    impl Orif {
        #[doc = "No receive overrun detected"]
        pub const _0: Self = Self::new(0);
        #[doc = "Receive overrun detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Borif_SPEC;
    pub type Borif = crate::EnumBitfieldStruct<u8, Borif_SPEC>;
    impl Borif {
        #[doc = "No bus-off recovery detected"]
        pub const _0: Self = Self::new(0);
        #[doc = "Bus-off recovery detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Boeif_SPEC;
    pub type Boeif = crate::EnumBitfieldStruct<u8, Boeif_SPEC>;
    impl Boeif {
        #[doc = "No bus-off entry detected"]
        pub const _0: Self = Self::new(0);
        #[doc = "Bus-off entry detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Epif_SPEC;
    pub type Epif = crate::EnumBitfieldStruct<u8, Epif_SPEC>;
    impl Epif {
        #[doc = "No error-passive detected"]
        pub const _0: Self = Self::new(0);
        #[doc = "Error-passive detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ewif_SPEC;
    pub type Ewif = crate::EnumBitfieldStruct<u8, Ewif_SPEC>;
    impl Ewif {
        #[doc = "No error-warning detected"]
        pub const _0: Self = Self::new(0);
        #[doc = "Error-warning detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Beif_SPEC;
    pub type Beif = crate::EnumBitfieldStruct<u8, Beif_SPEC>;
    impl Beif {
        #[doc = "No bus error detected"]
        pub const _0: Self = Self::new(0);
        #[doc = "Bus error detected"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Recr_SPEC;
impl crate::sealed::RegSpec for Recr_SPEC {
    type DataType = u8;
}
#[doc = "Receive Error Count Register"]
pub type Recr = crate::RegValueT<Recr_SPEC>;

impl Recr {
    #[doc = "Receive error count functionRECR increments or decrements the counter value according to the error status of the CAN module during reception."]
    #[inline(always)]
    pub fn recr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Recr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Recr_SPEC,crate::common::R>::from_register(self,0)
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
#[doc = "Transmit Error Count Register"]
pub type Tecr = crate::RegValueT<Tecr_SPEC>;

impl Tecr {
    #[doc = "Transmit error count functionTECR increments or decrements the counter value according to the error status of the CAN module during transmission."]
    #[inline(always)]
    pub fn tecr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Tecr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Tecr_SPEC,crate::common::R>::from_register(self,0)
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
#[doc = "Error Code Store Register"]
pub type Ecsr = crate::RegValueT<Ecsr_SPEC>;

impl Ecsr {
    #[doc = "Error Display Mode Select"]
    #[inline(always)]
    pub fn edpm(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, ecsr::Edpm, Ecsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x1,1,0,ecsr::Edpm, Ecsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ACK Delimiter Error Flag"]
    #[inline(always)]
    pub fn adef(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, ecsr::Adef, Ecsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x1,1,0,ecsr::Adef, Ecsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bit Error (dominant) Flag"]
    #[inline(always)]
    pub fn be0f(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, ecsr::Be0F, Ecsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x1,1,0,ecsr::Be0F, Ecsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bit Error (recessive) Flag"]
    #[inline(always)]
    pub fn be1f(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, ecsr::Be1F, Ecsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,ecsr::Be1F, Ecsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CRC Error Flag"]
    #[inline(always)]
    pub fn cef(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, ecsr::Cef, Ecsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,ecsr::Cef, Ecsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ACK Error Flag"]
    #[inline(always)]
    pub fn aef(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, ecsr::Aef, Ecsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,ecsr::Aef, Ecsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Form Error Flag"]
    #[inline(always)]
    pub fn fef(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, ecsr::Fef, Ecsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,ecsr::Fef, Ecsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Stuff Error Flag"]
    #[inline(always)]
    pub fn sef(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, ecsr::Sef, Ecsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,ecsr::Sef, Ecsr_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Output of first detected error code"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output of accumulated error code"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adef_SPEC;
    pub type Adef = crate::EnumBitfieldStruct<u8, Adef_SPEC>;
    impl Adef {
        #[doc = "No ACK delimiter error detected"]
        pub const _0: Self = Self::new(0);
        #[doc = "ACK delimiter error detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Be0F_SPEC;
    pub type Be0F = crate::EnumBitfieldStruct<u8, Be0F_SPEC>;
    impl Be0F {
        #[doc = "No bit error (dominant) detected"]
        pub const _0: Self = Self::new(0);
        #[doc = "Bit error (dominant) detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Be1F_SPEC;
    pub type Be1F = crate::EnumBitfieldStruct<u8, Be1F_SPEC>;
    impl Be1F {
        #[doc = "No bit error (recessive) detected"]
        pub const _0: Self = Self::new(0);
        #[doc = "Bit error (recessive) detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cef_SPEC;
    pub type Cef = crate::EnumBitfieldStruct<u8, Cef_SPEC>;
    impl Cef {
        #[doc = "No CRC error detected"]
        pub const _0: Self = Self::new(0);
        #[doc = "CRC error detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aef_SPEC;
    pub type Aef = crate::EnumBitfieldStruct<u8, Aef_SPEC>;
    impl Aef {
        #[doc = "No ACK error detected"]
        pub const _0: Self = Self::new(0);
        #[doc = "ACK error detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fef_SPEC;
    pub type Fef = crate::EnumBitfieldStruct<u8, Fef_SPEC>;
    impl Fef {
        #[doc = "No form error detected"]
        pub const _0: Self = Self::new(0);
        #[doc = "Form error detected"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sef_SPEC;
    pub type Sef = crate::EnumBitfieldStruct<u8, Sef_SPEC>;
    impl Sef {
        #[doc = "No stuff error detected"]
        pub const _0: Self = Self::new(0);
        #[doc = "Stuff error detected"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cssr_SPEC;
impl crate::sealed::RegSpec for Cssr_SPEC {
    type DataType = u8;
}
#[doc = "Channel Search Support Register"]
pub type Cssr = crate::RegValueT<Cssr_SPEC>;

impl Cssr {
    #[doc = "When the value for the channel search is input, the channel number is output to MSSR."]
    #[inline(always)]
    pub fn cssr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Cssr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Cssr_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Mailbox Search Status Register"]
pub type Mssr = crate::RegValueT<Mssr_SPEC>;

impl Mssr {
    #[doc = "Search Result Status"]
    #[inline(always)]
    pub fn sest(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, mssr::Sest, Mssr_SPEC, crate::common::R> {
        crate::common::RegisterField::<7,0x1,1,0,mssr::Sest, Mssr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Search Result Mailbox Number Status These bits output the smallest mailbox number that is searched in each mode of MSMR."]
    #[inline(always)]
    pub fn mbnst(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, Mssr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, Mssr_SPEC,crate::common::R>::from_register(self,0)
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
        #[doc = "Search result found"]
        pub const _0: Self = Self::new(0);
        #[doc = "No search result"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msmr_SPEC;
impl crate::sealed::RegSpec for Msmr_SPEC {
    type DataType = u8;
}
#[doc = "Mailbox Search Mode Register"]
pub type Msmr = crate::RegValueT<Msmr_SPEC>;

impl Msmr {
    #[doc = "Mailbox Search Mode Select"]
    #[inline(always)]
    pub fn mbsm(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, msmr::Mbsm, Msmr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,msmr::Mbsm, Msmr_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Receive mailbox search mode"]
        pub const _00: Self = Self::new(0);
        #[doc = "Transmit mailbox search mode"]
        pub const _01: Self = Self::new(1);
        #[doc = "Message lost search mode"]
        pub const _10: Self = Self::new(2);
        #[doc = "Channel search mode"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsr_SPEC;
impl crate::sealed::RegSpec for Tsr_SPEC {
    type DataType = u16;
}
#[doc = "Time Stamp Register"]
pub type Tsr = crate::RegValueT<Tsr_SPEC>;

impl Tsr {
    #[doc = "Free-running counter value for the time stamp function"]
    #[inline(always)]
    pub fn tsr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Tsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Tsr_SPEC,crate::common::R>::from_register(self,0)
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
#[doc = "Acceptance Filter Support Register"]
pub type Afsr = crate::RegValueT<Afsr_SPEC>;

impl Afsr {
    #[doc = "After the standard ID of a received message is written, the value converted for data table search can be read."]
    #[inline(always)]
    pub fn afsr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Afsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Afsr_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Test Control Register"]
pub type Tcr = crate::RegValueT<Tcr_SPEC>;

impl Tcr {
    #[doc = "CAN Test Mode Select"]
    #[inline(always)]
    pub fn tstm(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, tcr::Tstm, Tcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x3,1,0,tcr::Tstm, Tcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CAN Test Mode Enable"]
    #[inline(always)]
    pub fn tste(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, tcr::Tste, Tcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,tcr::Tste, Tcr_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Other than CAN test mode"]
        pub const _00: Self = Self::new(0);
        #[doc = "Listen-only mode"]
        pub const _01: Self = Self::new(1);
        #[doc = "Self-test mode 0 (external loopback)"]
        pub const _10: Self = Self::new(2);
        #[doc = "Self-test mode 1 (internal loopback)"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tste_SPEC;
    pub type Tste = crate::EnumBitfieldStruct<u8, Tste_SPEC>;
    impl Tste {
        #[doc = "CAN test mode disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "CAN test mode enabled"]
        pub const _1: Self = Self::new(1);
    }
}
