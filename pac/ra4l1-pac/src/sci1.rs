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
// Generated from SVD 0.90.02, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:15:45 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Serial Communication Interface 1"]
unsafe impl ::core::marker::Send for super::Sci1 {}
unsafe impl ::core::marker::Sync for super::Sci1 {}
impl super::Sci1 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "Serial Mode Register for Non-Smart Card Interface Mode (SCMR.SMIF = 0)"]
    #[inline(always)]
    pub const fn smr(&self) -> &'static crate::common::Reg<self::Smr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Smr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Serial Mode Register for Smart Card Interface Mode (SCMR.SMIF = 1)"]
    #[inline(always)]
    pub const fn smr_smci(
        &self,
    ) -> &'static crate::common::Reg<self::SmrSmci_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SmrSmci_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Bit Rate Register"]
    #[inline(always)]
    pub const fn brr(&self) -> &'static crate::common::Reg<self::Brr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Brr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1usize),
            )
        }
    }

    #[doc = "Serial Control Register for Non-Smart Card Interface Mode (SCMR.SMIF = 0)"]
    #[inline(always)]
    pub const fn scr(&self) -> &'static crate::common::Reg<self::Scr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Scr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "Serial Control Register for Smart Card Interface Mode (SCMR.SMIF = 1)"]
    #[inline(always)]
    pub const fn scr_smci(
        &self,
    ) -> &'static crate::common::Reg<self::ScrSmci_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ScrSmci_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "Transmit Data Register"]
    #[inline(always)]
    pub const fn tdr(&self) -> &'static crate::common::Reg<self::Tdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(3usize),
            )
        }
    }

    #[doc = "Serial Status Register for Non-Smart Card Interface and Non-FIFO Mode (SCMR.SMIF = 0, FCR.FM = 0, and MMR.MANEN = 0)"]
    #[inline(always)]
    pub const fn ssr(&self) -> &'static crate::common::Reg<self::Ssr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ssr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Serial Status Register for Smart Card Interface Mode (SCMR.SMIF = 1, and MMR.MANEN = 0)"]
    #[inline(always)]
    pub const fn ssr_smci(
        &self,
    ) -> &'static crate::common::Reg<self::SsrSmci_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SsrSmci_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Receive Data Register"]
    #[inline(always)]
    pub const fn rdr(&self) -> &'static crate::common::Reg<self::Rdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(5usize),
            )
        }
    }

    #[doc = "Smart Card Mode Register"]
    #[inline(always)]
    pub const fn scmr(&self) -> &'static crate::common::Reg<self::Scmr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Scmr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[doc = "Simple LIN Mode Register"]
    #[inline(always)]
    pub const fn semr(&self) -> &'static crate::common::Reg<self::Semr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Semr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(7usize),
            )
        }
    }

    #[doc = "Noise Filter Setting Register"]
    #[inline(always)]
    pub const fn snfr(&self) -> &'static crate::common::Reg<self::Snfr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Snfr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "IIC Mode Register 1"]
    #[inline(always)]
    pub const fn simr1(&self) -> &'static crate::common::Reg<self::Simr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Simr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(9usize),
            )
        }
    }

    #[doc = "IIC Mode Register 2"]
    #[inline(always)]
    pub const fn simr2(&self) -> &'static crate::common::Reg<self::Simr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Simr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[doc = "IIC Mode Register 3"]
    #[inline(always)]
    pub const fn simr3(&self) -> &'static crate::common::Reg<self::Simr3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Simr3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(11usize),
            )
        }
    }

    #[doc = "IIC Status Register"]
    #[inline(always)]
    pub const fn sisr(&self) -> &'static crate::common::Reg<self::Sisr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Sisr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "SPI Mode Register"]
    #[inline(always)]
    pub const fn spmr(&self) -> &'static crate::common::Reg<self::Spmr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Spmr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(13usize),
            )
        }
    }

    #[doc = "Transmit Data Register for Non-Manchester Mode (MMR.MANEN = 0)"]
    #[inline(always)]
    pub const fn tdrhl(&self) -> &'static crate::common::Reg<self::Tdrhl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tdrhl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[doc = "Receive Data Register for Non-Manchester Mode (MMR.MANEN = 0)"]
    #[inline(always)]
    pub const fn rdrhl(&self) -> &'static crate::common::Reg<self::Rdrhl_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rdrhl_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Modulation Duty Register"]
    #[inline(always)]
    pub const fn mddr(&self) -> &'static crate::common::Reg<self::Mddr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mddr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18usize),
            )
        }
    }

    #[doc = "Simple LIN Mode Enable Register"]
    #[inline(always)]
    pub const fn esmer(&self) -> &'static crate::common::Reg<self::Esmer_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Esmer_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Control Register 0"]
    #[inline(always)]
    pub const fn cr0(&self) -> &'static crate::common::Reg<self::Cr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(33usize),
            )
        }
    }

    #[doc = "Control Register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &'static crate::common::Reg<self::Cr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(34usize),
            )
        }
    }

    #[doc = "Control Register 2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &'static crate::common::Reg<self::Cr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(35usize),
            )
        }
    }

    #[doc = "Control Register 3"]
    #[inline(always)]
    pub const fn cr3(&self) -> &'static crate::common::Reg<self::Cr3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cr3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "Port Control Register"]
    #[inline(always)]
    pub const fn pcr(&self) -> &'static crate::common::Reg<self::Pcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(37usize),
            )
        }
    }

    #[doc = "Interrupt Control Register"]
    #[inline(always)]
    pub const fn icr(&self) -> &'static crate::common::Reg<self::Icr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(38usize),
            )
        }
    }

    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn str(&self) -> &'static crate::common::Reg<self::Str_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Str_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(39usize),
            )
        }
    }

    #[doc = "Status Clear Register"]
    #[inline(always)]
    pub const fn stcr(&self) -> &'static crate::common::Reg<self::Stcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Stcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "Control Field 0 Data Register"]
    #[inline(always)]
    pub const fn cf0dr(&self) -> &'static crate::common::Reg<self::Cf0Dr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cf0Dr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(41usize),
            )
        }
    }

    #[doc = "Control Field 0 Compare Enable Register"]
    #[inline(always)]
    pub const fn cf0cr(&self) -> &'static crate::common::Reg<self::Cf0Cr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cf0Cr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(42usize),
            )
        }
    }

    #[doc = "Control Field 0 Receive Data Register"]
    #[inline(always)]
    pub const fn cf0rr(&self) -> &'static crate::common::Reg<self::Cf0Rr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cf0Rr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(43usize),
            )
        }
    }

    #[doc = "Primary Control Field 1 Data Register"]
    #[inline(always)]
    pub const fn pcf1dr(
        &self,
    ) -> &'static crate::common::Reg<self::Pcf1Dr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pcf1Dr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "Secondary Control Field 1 Data Register"]
    #[inline(always)]
    pub const fn scf1dr(
        &self,
    ) -> &'static crate::common::Reg<self::Scf1Dr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Scf1Dr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(45usize),
            )
        }
    }

    #[doc = "Control Field 1 Compare Enable Register"]
    #[inline(always)]
    pub const fn cf1cr(&self) -> &'static crate::common::Reg<self::Cf1Cr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cf1Cr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(46usize),
            )
        }
    }

    #[doc = "Control Field 1 Receive Data Register"]
    #[inline(always)]
    pub const fn cf1rr(&self) -> &'static crate::common::Reg<self::Cf1Rr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Cf1Rr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(47usize),
            )
        }
    }

    #[doc = "Timer Control Register"]
    #[inline(always)]
    pub const fn tcr(&self) -> &'static crate::common::Reg<self::Tcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "Timer Mode Register"]
    #[inline(always)]
    pub const fn tmr(&self) -> &'static crate::common::Reg<self::Tmr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tmr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(49usize),
            )
        }
    }

    #[doc = "Timer Prescaler Register"]
    #[inline(always)]
    pub const fn tpre(&self) -> &'static crate::common::Reg<self::Tpre_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tpre_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(50usize),
            )
        }
    }

    #[doc = "Timer Count Register"]
    #[inline(always)]
    pub const fn tcnt(&self) -> &'static crate::common::Reg<self::Tcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(51usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smr_SPEC;
impl crate::sealed::RegSpec for Smr_SPEC {
    type DataType = u8;
}
#[doc = "Serial Mode Register for Non-Smart Card Interface Mode (SCMR.SMIF = 0)"]
pub type Smr = crate::RegValueT<Smr_SPEC>;

impl Smr {
    #[doc = "Clock Select"]
    #[inline(always)]
    pub fn cks(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, smr::Cks, Smr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,smr::Cks, Smr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Multi-Processor Mode"]
    #[inline(always)]
    pub fn mp(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, smr::Mp, Smr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,smr::Mp, Smr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Stop Bit Length"]
    #[inline(always)]
    pub fn stop(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, smr::Stop, Smr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,smr::Stop, Smr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Parity Mode"]
    #[inline(always)]
    pub fn pm(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, smr::Pm, Smr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,smr::Pm, Smr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Parity Enable"]
    #[inline(always)]
    pub fn pe(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, smr::Pe, Smr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x1,1,0,smr::Pe, Smr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Character Length"]
    #[inline(always)]
    pub fn chr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, smr::Chr, Smr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x1,1,0,smr::Chr, Smr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Communication Mode"]
    #[inline(always)]
    pub fn cm(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, smr::Cm, Smr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x1,1,0,smr::Cm, Smr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Smr {
    #[inline(always)]
    fn default() -> Smr {
        <crate::RegValueT<Smr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod smr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cks_SPEC;
    pub type Cks = crate::EnumBitfieldStruct<u8, Cks_SPEC>;
    impl Cks {
        #[doc = "PCLK clock (n = 0)"]
        pub const _00: Self = Self::new(0);
        #[doc = "PCLK/4 clock (n = 1)"]
        pub const _01: Self = Self::new(1);
        #[doc = "PCLK/16 clock (n = 2)"]
        pub const _10: Self = Self::new(2);
        #[doc = "PCLK/64 clock (n = 3)"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mp_SPEC;
    pub type Mp = crate::EnumBitfieldStruct<u8, Mp_SPEC>;
    impl Mp {
        #[doc = "Disable multi-processor communications function"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable multi-processor communications function"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stop_SPEC;
    pub type Stop = crate::EnumBitfieldStruct<u8, Stop_SPEC>;
    impl Stop {
        #[doc = "1 stop bit"]
        pub const _0: Self = Self::new(0);
        #[doc = "2 stop bits"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pm_SPEC;
    pub type Pm = crate::EnumBitfieldStruct<u8, Pm_SPEC>;
    impl Pm {
        #[doc = "Even parity"]
        pub const _0: Self = Self::new(0);
        #[doc = "Odd parity"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pe_SPEC;
    pub type Pe = crate::EnumBitfieldStruct<u8, Pe_SPEC>;
    impl Pe {
        #[doc = "When transmitting: Do not add parity bit When receiving: Do not check parity bit"]
        pub const _0: Self = Self::new(0);
        #[doc = "When transmitting: Add parity bit When receiving: Check parity bit"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chr_SPEC;
    pub type Chr = crate::EnumBitfieldStruct<u8, Chr_SPEC>;
    impl Chr {
        #[doc = "SCMR.CHR1 = 0: Transmit/receive in 9-bit data length SCMR.CHR1 = 1: Transmit/receive in 8-bit data length (initial value)"]
        pub const _0: Self = Self::new(0);
        #[doc = "SCMR.CHR1 = 0: Transmit/receive in 9-bit data length SCMR.CHR1 = 1: Transmit/receive in 7-bit data length"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cm_SPEC;
    pub type Cm = crate::EnumBitfieldStruct<u8, Cm_SPEC>;
    impl Cm {
        #[doc = "Asynchronous mode or simple IIC mode"]
        pub const _0: Self = Self::new(0);
        #[doc = "Clock synchronous mode or simple SPI mode"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmrSmci_SPEC;
impl crate::sealed::RegSpec for SmrSmci_SPEC {
    type DataType = u8;
}
#[doc = "Serial Mode Register for Smart Card Interface Mode (SCMR.SMIF = 1)"]
pub type SmrSmci = crate::RegValueT<SmrSmci_SPEC>;

impl SmrSmci {
    #[doc = "Clock Select"]
    #[inline(always)]
    pub fn cks(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, smr_smci::Cks, SmrSmci_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,smr_smci::Cks, SmrSmci_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Base Clock Pulse"]
    #[inline(always)]
    pub fn bcp(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, SmrSmci_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8, SmrSmci_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Parity Mode"]
    #[inline(always)]
    pub fn pm(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, smr_smci::Pm, SmrSmci_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,smr_smci::Pm, SmrSmci_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Parity Enable"]
    #[inline(always)]
    pub fn pe(self) -> crate::common::RegisterFieldBool<5, 1, 0, SmrSmci_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, SmrSmci_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Block Transfer Mode"]
    #[inline(always)]
    pub fn blk(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, smr_smci::Blk, SmrSmci_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,smr_smci::Blk, SmrSmci_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GSM Mode"]
    #[inline(always)]
    pub fn gm(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, smr_smci::Gm, SmrSmci_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,smr_smci::Gm, SmrSmci_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SmrSmci {
    #[inline(always)]
    fn default() -> SmrSmci {
        <crate::RegValueT<SmrSmci_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod smr_smci {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cks_SPEC;
    pub type Cks = crate::EnumBitfieldStruct<u8, Cks_SPEC>;
    impl Cks {
        #[doc = "PCLK clock (n = 0)"]
        pub const _00: Self = Self::new(0);
        #[doc = "PCLK/4 clock (n = 1)"]
        pub const _01: Self = Self::new(1);
        #[doc = "PCLK/16 clock (n = 2)"]
        pub const _10: Self = Self::new(2);
        #[doc = "PCLK/64 clock (n = 3)"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pm_SPEC;
    pub type Pm = crate::EnumBitfieldStruct<u8, Pm_SPEC>;
    impl Pm {
        #[doc = "Even parity"]
        pub const _0: Self = Self::new(0);
        #[doc = "Odd parity"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Blk_SPEC;
    pub type Blk = crate::EnumBitfieldStruct<u8, Blk_SPEC>;
    impl Blk {
        #[doc = "Normal mode operation"]
        pub const _0: Self = Self::new(0);
        #[doc = "Block transfer mode operation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gm_SPEC;
    pub type Gm = crate::EnumBitfieldStruct<u8, Gm_SPEC>;
    impl Gm {
        #[doc = "Normal mode operation"]
        pub const _0: Self = Self::new(0);
        #[doc = "GSM mode operation"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Brr_SPEC;
impl crate::sealed::RegSpec for Brr_SPEC {
    type DataType = u8;
}
#[doc = "Bit Rate Register"]
pub type Brr = crate::RegValueT<Brr_SPEC>;

impl NoBitfieldReg<Brr_SPEC> for Brr {}
impl ::core::default::Default for Brr {
    #[inline(always)]
    fn default() -> Brr {
        <crate::RegValueT<Brr_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr_SPEC;
impl crate::sealed::RegSpec for Scr_SPEC {
    type DataType = u8;
}
#[doc = "Serial Control Register for Non-Smart Card Interface Mode (SCMR.SMIF = 0)"]
pub type Scr = crate::RegValueT<Scr_SPEC>;

impl Scr {
    #[doc = "Clock Enable"]
    #[inline(always)]
    pub fn cke(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, scr::Cke, Scr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,scr::Cke, Scr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmit End Interrupt Enable"]
    #[inline(always)]
    pub fn teie(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, scr::Teie, Scr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,scr::Teie, Scr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Multi-Processor Interrupt Enable"]
    #[inline(always)]
    pub fn mpie(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, scr::Mpie, Scr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,scr::Mpie, Scr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Receive Enable"]
    #[inline(always)]
    pub fn re(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, scr::Re, Scr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,scr::Re, Scr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmit Enable"]
    #[inline(always)]
    pub fn te(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, scr::Te, Scr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x1,1,0,scr::Te, Scr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rie(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, scr::Rie, Scr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x1,1,0,scr::Rie, Scr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn tie(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, scr::Tie, Scr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x1,1,0,scr::Tie, Scr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Scr {
    #[inline(always)]
    fn default() -> Scr {
        <crate::RegValueT<Scr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod scr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cke_SPEC;
    pub type Cke = crate::EnumBitfieldStruct<u8, Cke_SPEC>;
    impl Cke {
        #[doc = "In asynchronous mode, the SCKn pin is available for use as an I/O port based on the I/O port settings. In clock synchronous mode, the SCKn pin functions as the clock output pin."]
        pub const _00: Self = Self::new(0);
        #[doc = "In asynchronous mode, a clock with the same frequency as the bit rate is output from the SCKn pin. In clock synchronous mode, the SCKn pin functions as the clock output pin."]
        pub const _01: Self = Self::new(1);
        #[doc = "In asynchronous mode, input a clock with a frequency 16 times the bit rate from the SCKn pin when the SEMR.ABCS bit is 0. Input a clock signal with a frequency eight times the bit rate when the SEMR.ABCS bit is 1. The SCKn pin is available for use as an I/O port based on the I/O port settings when the GPT clock is used. In clock synchronous mode, the SCKn pin functions as the clock input pin."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Teie_SPEC;
    pub type Teie = crate::EnumBitfieldStruct<u8, Teie_SPEC>;
    impl Teie {
        #[doc = "Disable SCIn_TEI interrupt requests"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable SCIn_TEI interrupt requests"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpie_SPEC;
    pub type Mpie = crate::EnumBitfieldStruct<u8, Mpie_SPEC>;
    impl Mpie {
        #[doc = "Normal reception"]
        pub const _0: Self = Self::new(0);
        #[doc = "When data with the multi-processor bit set to 0 is received, the data is not read, and setting the status flags RDRF, ORER, and FER in SSR to 1 and the status flags SYER, PFER, and SBER in MESR are disabled. When data with the multi-processor bit set to 1 is received, the MPIE bit is automatically set to 0, and normal reception is resumed."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Re_SPEC;
    pub type Re = crate::EnumBitfieldStruct<u8, Re_SPEC>;
    impl Re {
        #[doc = "Disable serial reception"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable serial reception"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Te_SPEC;
    pub type Te = crate::EnumBitfieldStruct<u8, Te_SPEC>;
    impl Te {
        #[doc = "Disable serial transmission"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable serial transmission"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rie_SPEC;
    pub type Rie = crate::EnumBitfieldStruct<u8, Rie_SPEC>;
    impl Rie {
        #[doc = "Disable SCIn_RXI and SCIn_ERI interrupt requests"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable SCIn_RXI and SCIn_ERI interrupt requests"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tie_SPEC;
    pub type Tie = crate::EnumBitfieldStruct<u8, Tie_SPEC>;
    impl Tie {
        #[doc = "Disable SCIn_TXI interrupt requests"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable SCIn_TXI interrupt requests"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ScrSmci_SPEC;
impl crate::sealed::RegSpec for ScrSmci_SPEC {
    type DataType = u8;
}
#[doc = "Serial Control Register for Smart Card Interface Mode (SCMR.SMIF = 1)"]
pub type ScrSmci = crate::RegValueT<ScrSmci_SPEC>;

impl ScrSmci {
    #[doc = "Clock Enable"]
    #[inline(always)]
    pub fn cke(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, scr_smci::Cke, ScrSmci_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,scr_smci::Cke, ScrSmci_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmit End Interrupt Enable"]
    #[inline(always)]
    pub fn teie(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ScrSmci_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, ScrSmci_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Multi-Processor Interrupt Enable"]
    #[inline(always)]
    pub fn mpie(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ScrSmci_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, ScrSmci_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Receive Enable"]
    #[inline(always)]
    pub fn re(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, scr_smci::Re, ScrSmci_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,scr_smci::Re, ScrSmci_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmit Enable"]
    #[inline(always)]
    pub fn te(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, scr_smci::Te, ScrSmci_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,scr_smci::Te, ScrSmci_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rie(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, scr_smci::Rie, ScrSmci_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,scr_smci::Rie, ScrSmci_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn tie(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, scr_smci::Tie, ScrSmci_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,scr_smci::Tie, ScrSmci_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ScrSmci {
    #[inline(always)]
    fn default() -> ScrSmci {
        <crate::RegValueT<ScrSmci_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod scr_smci {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cke_SPEC;
    pub type Cke = crate::EnumBitfieldStruct<u8, Cke_SPEC>;
    impl Cke {
        #[doc = "When SMR_SMCI.GM = 0: Disable output The SCKn pin is available for use as an I/O port if set up in the I/O port settings When SMR_SMCI.GM = 1: Fix output low"]
        pub const _00: Self = Self::new(0);
        #[doc = "When SMR_SMCI.GM = 0: Output clock When SMR_SMCI.GM = 1: Output clock"]
        pub const _01: Self = Self::new(1);
        #[doc = "When SMR_SMCI.GM = 0: Setting prohibited When SMR_SMCI.GM = 1: Fix output high"]
        pub const _10: Self = Self::new(2);
        #[doc = "When SMR_SMCI.GM = 0: Setting prohibited When SMR_SMCI.GM = 1: Output clock"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Re_SPEC;
    pub type Re = crate::EnumBitfieldStruct<u8, Re_SPEC>;
    impl Re {
        #[doc = "Disable serial reception"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable serial reception"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Te_SPEC;
    pub type Te = crate::EnumBitfieldStruct<u8, Te_SPEC>;
    impl Te {
        #[doc = "Disable serial transmission"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable serial transmission"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rie_SPEC;
    pub type Rie = crate::EnumBitfieldStruct<u8, Rie_SPEC>;
    impl Rie {
        #[doc = "Disable SCIn_RXI and SCIn_ERI interrupt requests"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable SCIn_RXI and SCIn_ERI interrupt requests"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tie_SPEC;
    pub type Tie = crate::EnumBitfieldStruct<u8, Tie_SPEC>;
    impl Tie {
        #[doc = "Disable SCIn_TXI interrupt requests"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable SCIn_TXI interrupt requests"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdr_SPEC;
impl crate::sealed::RegSpec for Tdr_SPEC {
    type DataType = u8;
}
#[doc = "Transmit Data Register"]
pub type Tdr = crate::RegValueT<Tdr_SPEC>;

impl NoBitfieldReg<Tdr_SPEC> for Tdr {}
impl ::core::default::Default for Tdr {
    #[inline(always)]
    fn default() -> Tdr {
        <crate::RegValueT<Tdr_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssr_SPEC;
impl crate::sealed::RegSpec for Ssr_SPEC {
    type DataType = u8;
}
#[doc = "Serial Status Register for Non-Smart Card Interface and Non-FIFO Mode (SCMR.SMIF = 0, FCR.FM = 0, and MMR.MANEN = 0)"]
pub type Ssr = crate::RegValueT<Ssr_SPEC>;

impl Ssr {
    #[doc = "Multi-Processor Bit Transfer"]
    #[inline(always)]
    pub fn mpbt(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, ssr::Mpbt, Ssr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,ssr::Mpbt, Ssr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Multi-Processor"]
    #[inline(always)]
    pub fn mpb(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, ssr::Mpb, Ssr_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,ssr::Mpb, Ssr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmit End Flag"]
    #[inline(always)]
    pub fn tend(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, ssr::Tend, Ssr_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x1,1,0,ssr::Tend, Ssr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Parity Error Flag"]
    #[inline(always)]
    pub fn per(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, ssr::Per, Ssr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,ssr::Per, Ssr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Framing Error Flag"]
    #[inline(always)]
    pub fn fer(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, ssr::Fer, Ssr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,ssr::Fer, Ssr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Overrun Error Flag"]
    #[inline(always)]
    pub fn orer(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, ssr::Orer, Ssr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x1,1,0,ssr::Orer, Ssr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Receive Data Full Flag"]
    #[inline(always)]
    pub fn rdrf(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, ssr::Rdrf, Ssr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x1,1,0,ssr::Rdrf, Ssr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmit Data Empty Flag"]
    #[inline(always)]
    pub fn tdre(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, ssr::Tdre, Ssr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x1,1,0,ssr::Tdre, Ssr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ssr {
    #[inline(always)]
    fn default() -> Ssr {
        <crate::RegValueT<Ssr_SPEC> as RegisterValue<_>>::new(132)
    }
}
pub mod ssr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpbt_SPEC;
    pub type Mpbt = crate::EnumBitfieldStruct<u8, Mpbt_SPEC>;
    impl Mpbt {
        #[doc = "Data transmission cycle"]
        pub const _0: Self = Self::new(0);
        #[doc = "ID transmission cycle"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpb_SPEC;
    pub type Mpb = crate::EnumBitfieldStruct<u8, Mpb_SPEC>;
    impl Mpb {
        #[doc = "Data transmission cycle"]
        pub const _0: Self = Self::new(0);
        #[doc = "ID transmission cycle"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tend_SPEC;
    pub type Tend = crate::EnumBitfieldStruct<u8, Tend_SPEC>;
    impl Tend {
        #[doc = "A character is being transmitted"]
        pub const _0: Self = Self::new(0);
        #[doc = "Character transfer is complete"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Per_SPEC;
    pub type Per = crate::EnumBitfieldStruct<u8, Per_SPEC>;
    impl Per {
        #[doc = "No parity error occurred"]
        pub const _0: Self = Self::new(0);
        #[doc = "Parity error occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fer_SPEC;
    pub type Fer = crate::EnumBitfieldStruct<u8, Fer_SPEC>;
    impl Fer {
        #[doc = "No framing error occurred"]
        pub const _0: Self = Self::new(0);
        #[doc = "Framing error occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Orer_SPEC;
    pub type Orer = crate::EnumBitfieldStruct<u8, Orer_SPEC>;
    impl Orer {
        #[doc = "No overrun error occurred"]
        pub const _0: Self = Self::new(0);
        #[doc = "Overrun error occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdrf_SPEC;
    pub type Rdrf = crate::EnumBitfieldStruct<u8, Rdrf_SPEC>;
    impl Rdrf {
        #[doc = "No received data in RDR register"]
        pub const _0: Self = Self::new(0);
        #[doc = "Received data in RDR register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdre_SPEC;
    pub type Tdre = crate::EnumBitfieldStruct<u8, Tdre_SPEC>;
    impl Tdre {
        #[doc = "Transmit data in TDR register"]
        pub const _0: Self = Self::new(0);
        #[doc = "No transmit data in TDR register"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SsrSmci_SPEC;
impl crate::sealed::RegSpec for SsrSmci_SPEC {
    type DataType = u8;
}
#[doc = "Serial Status Register for Smart Card Interface Mode (SCMR.SMIF = 1, and MMR.MANEN = 0)"]
pub type SsrSmci = crate::RegValueT<SsrSmci_SPEC>;

impl SsrSmci {
    #[doc = "Multi-Processor Bit Transfer"]
    #[inline(always)]
    pub fn mpbt(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SsrSmci_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, SsrSmci_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Multi-Processor"]
    #[inline(always)]
    pub fn mpb(self) -> crate::common::RegisterFieldBool<1, 1, 0, SsrSmci_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, SsrSmci_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmit End Flag"]
    #[inline(always)]
    pub fn tend(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, ssr_smci::Tend, SsrSmci_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x1,1,0,ssr_smci::Tend, SsrSmci_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Parity Error Flag"]
    #[inline(always)]
    pub fn per(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, ssr_smci::Per, SsrSmci_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,ssr_smci::Per, SsrSmci_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Error Signal Status Flag"]
    #[inline(always)]
    pub fn ers(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, ssr_smci::Ers, SsrSmci_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,ssr_smci::Ers, SsrSmci_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Overrun Error Flag"]
    #[inline(always)]
    pub fn orer(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, ssr_smci::Orer, SsrSmci_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,ssr_smci::Orer, SsrSmci_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Receive Data Full Flag"]
    #[inline(always)]
    pub fn rdrf(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, ssr_smci::Rdrf, SsrSmci_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,ssr_smci::Rdrf, SsrSmci_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmit Data Empty Flag"]
    #[inline(always)]
    pub fn tdre(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, ssr_smci::Tdre, SsrSmci_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,ssr_smci::Tdre, SsrSmci_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SsrSmci {
    #[inline(always)]
    fn default() -> SsrSmci {
        <crate::RegValueT<SsrSmci_SPEC> as RegisterValue<_>>::new(132)
    }
}
pub mod ssr_smci {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tend_SPEC;
    pub type Tend = crate::EnumBitfieldStruct<u8, Tend_SPEC>;
    impl Tend {
        #[doc = "A character is being transmitted"]
        pub const _0: Self = Self::new(0);
        #[doc = "Character transfer is complete"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Per_SPEC;
    pub type Per = crate::EnumBitfieldStruct<u8, Per_SPEC>;
    impl Per {
        #[doc = "No parity error occurred"]
        pub const _0: Self = Self::new(0);
        #[doc = "Parity error occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ers_SPEC;
    pub type Ers = crate::EnumBitfieldStruct<u8, Ers_SPEC>;
    impl Ers {
        #[doc = "No low error signal response"]
        pub const _0: Self = Self::new(0);
        #[doc = "Low error signal response occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Orer_SPEC;
    pub type Orer = crate::EnumBitfieldStruct<u8, Orer_SPEC>;
    impl Orer {
        #[doc = "No overrun error occurred"]
        pub const _0: Self = Self::new(0);
        #[doc = "Overrun error occurred"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdrf_SPEC;
    pub type Rdrf = crate::EnumBitfieldStruct<u8, Rdrf_SPEC>;
    impl Rdrf {
        #[doc = "No received data in RDR register"]
        pub const _0: Self = Self::new(0);
        #[doc = "Received data in RDR register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdre_SPEC;
    pub type Tdre = crate::EnumBitfieldStruct<u8, Tdre_SPEC>;
    impl Tdre {
        #[doc = "Transmit data in TDR register"]
        pub const _0: Self = Self::new(0);
        #[doc = "No transmit data in TDR register"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdr_SPEC;
impl crate::sealed::RegSpec for Rdr_SPEC {
    type DataType = u8;
}
#[doc = "Receive Data Register"]
pub type Rdr = crate::RegValueT<Rdr_SPEC>;

impl NoBitfieldReg<Rdr_SPEC> for Rdr {}
impl ::core::default::Default for Rdr {
    #[inline(always)]
    fn default() -> Rdr {
        <crate::RegValueT<Rdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scmr_SPEC;
impl crate::sealed::RegSpec for Scmr_SPEC {
    type DataType = u8;
}
#[doc = "Smart Card Mode Register"]
pub type Scmr = crate::RegValueT<Scmr_SPEC>;

impl Scmr {
    #[doc = "Smart Card Interface Mode Select"]
    #[inline(always)]
    pub fn smif(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, scmr::Smif, Scmr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,scmr::Smif, Scmr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmitted/Received Data Invert"]
    #[inline(always)]
    pub fn sinv(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, scmr::Sinv, Scmr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,scmr::Sinv, Scmr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmitted/Received Data Transfer Direction"]
    #[inline(always)]
    pub fn sdir(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, scmr::Sdir, Scmr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,scmr::Sdir, Scmr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Character Length 1"]
    #[inline(always)]
    pub fn chr1(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, scmr::Chr1, Scmr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,scmr::Chr1, Scmr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Base Clock Pulse 2"]
    #[inline(always)]
    pub fn bcp2(self) -> crate::common::RegisterFieldBool<7, 1, 0, Scmr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Scmr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Scmr {
    #[inline(always)]
    fn default() -> Scmr {
        <crate::RegValueT<Scmr_SPEC> as RegisterValue<_>>::new(242)
    }
}
pub mod scmr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Smif_SPEC;
    pub type Smif = crate::EnumBitfieldStruct<u8, Smif_SPEC>;
    impl Smif {
        #[doc = "Non-smart card interface mode (asynchronous mode, clock synchronous mode, simple SPI mode, or simple IIC mode)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Smart card interface mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sinv_SPEC;
    pub type Sinv = crate::EnumBitfieldStruct<u8, Sinv_SPEC>;
    impl Sinv {
        #[doc = "TDR contents are transmitted as they are. Received data is stored as received in the RDR register."]
        pub const _0: Self = Self::new(0);
        #[doc = "TDR register contents are inverted before transmission. Receive data is stored in inverted form in the RDR register."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdir_SPEC;
    pub type Sdir = crate::EnumBitfieldStruct<u8, Sdir_SPEC>;
    impl Sdir {
        #[doc = "Transfer LSB-first"]
        pub const _0: Self = Self::new(0);
        #[doc = "Transfer MSB-first"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chr1_SPEC;
    pub type Chr1 = crate::EnumBitfieldStruct<u8, Chr1_SPEC>;
    impl Chr1 {
        #[doc = "SMR.CHR = 0: Transmit/receive in 9-bit data length SMR.CHR = 1: Transmit/receive in 9-bit data length"]
        pub const _0: Self = Self::new(0);
        #[doc = "SMR.CHR = 0: Transmit/receive in 8-bit data length (initial value) SMR.CHR = 1: Transmit/receive in 7-bit data length"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Semr_SPEC;
impl crate::sealed::RegSpec for Semr_SPEC {
    type DataType = u8;
}
#[doc = "Simple LIN Mode Register"]
pub type Semr = crate::RegValueT<Semr_SPEC>;

impl Semr {
    #[doc = "Asynchronous Mode Clock Source Select"]
    #[inline(always)]
    pub fn acs0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, semr::Acs0, Semr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,semr::Acs0, Semr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Preamble function Disable"]
    #[inline(always)]
    pub fn padis(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, semr::Padis, Semr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,semr::Padis, Semr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bit Rate Modulation Enable"]
    #[inline(always)]
    pub fn brme(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, semr::Brme, Semr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,semr::Brme, Semr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Asynchronous Mode Extended Base Clock Select 1"]
    #[inline(always)]
    pub fn abcse(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, semr::Abcse, Semr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,semr::Abcse, Semr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Asynchronous Mode Base Clock Select"]
    #[inline(always)]
    pub fn abcs(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, semr::Abcs, Semr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,semr::Abcs, Semr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Digital Noise Filter Function Enable"]
    #[inline(always)]
    pub fn nfen(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, semr::Nfen, Semr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x1,1,0,semr::Nfen, Semr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Baud Rate Generator Double-Speed Mode Select"]
    #[inline(always)]
    pub fn bgdm(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, semr::Bgdm, Semr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x1,1,0,semr::Bgdm, Semr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Asynchronous Start Bit Edge Detection Select"]
    #[inline(always)]
    pub fn rxdesel(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, semr::Rxdesel, Semr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,semr::Rxdesel, Semr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Semr {
    #[inline(always)]
    fn default() -> Semr {
        <crate::RegValueT<Semr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod semr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Acs0_SPEC;
    pub type Acs0 = crate::EnumBitfieldStruct<u8, Acs0_SPEC>;
    impl Acs0 {
        #[doc = "External clock input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Logical AND of compare matches output from the internal GPT. These bit for the other SCI channels than SCIn (n = 1) are reserved."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Padis_SPEC;
    pub type Padis = crate::EnumBitfieldStruct<u8, Padis_SPEC>;
    impl Padis {
        #[doc = "Preamble output function is enabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Preamble output function is disabled These bits for the other SCI channels than SCIn (n = 0, 3 to 5, 9) are reserved."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Brme_SPEC;
    pub type Brme = crate::EnumBitfieldStruct<u8, Brme_SPEC>;
    impl Brme {
        #[doc = "Disable bit rate modulation function"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable bit rate modulation function"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Abcse_SPEC;
    pub type Abcse = crate::EnumBitfieldStruct<u8, Abcse_SPEC>;
    impl Abcse {
        #[doc = "Clock cycles for 1-bit period determined by combination of the BGDM and ABCS bits in the SEMR register"]
        pub const _0: Self = Self::new(0);
        #[doc = "Baud rate is 6 base clock cycles for 1-bit period These bits for the other SCI channels than SCIn (n = 0, 3 to 5, 9) are reserved."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Abcs_SPEC;
    pub type Abcs = crate::EnumBitfieldStruct<u8, Abcs_SPEC>;
    impl Abcs {
        #[doc = "Select 16 base clock cycles for 1-bit period"]
        pub const _0: Self = Self::new(0);
        #[doc = "Select 8 base clock cycles for 1-bit period"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfen_SPEC;
    pub type Nfen = crate::EnumBitfieldStruct<u8, Nfen_SPEC>;
    impl Nfen {
        #[doc = "In asynchronous mode: Disable noise cancellation function for RXDn input signal In simple I2C mode: Disable noise cancellation function for SCLn and SDAn input signals"]
        pub const _0: Self = Self::new(0);
        #[doc = "In asynchronous mode: Enable noise cancellation function for RXDn input signal In simple I2C mode: Enable noise cancellation function for SCLn and SDAn input signals"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bgdm_SPEC;
    pub type Bgdm = crate::EnumBitfieldStruct<u8, Bgdm_SPEC>;
    impl Bgdm {
        #[doc = "Output clock from baud rate generator with normal frequency"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output clock from baud rate generator with doubled frequency"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxdesel_SPEC;
    pub type Rxdesel = crate::EnumBitfieldStruct<u8, Rxdesel_SPEC>;
    impl Rxdesel {
        #[doc = "Detect low level on RXDn pin as start bit"]
        pub const _0: Self = Self::new(0);
        #[doc = "Detect falling edge of RXDn pin as start bit"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Snfr_SPEC;
impl crate::sealed::RegSpec for Snfr_SPEC {
    type DataType = u8;
}
#[doc = "Noise Filter Setting Register"]
pub type Snfr = crate::RegValueT<Snfr_SPEC>;

impl Snfr {
    #[doc = "Noise Filter Clock Select"]
    #[inline(always)]
    pub fn nfcs(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, snfr::Nfcs, Snfr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,snfr::Nfcs, Snfr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Snfr {
    #[inline(always)]
    fn default() -> Snfr {
        <crate::RegValueT<Snfr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod snfr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfcs_SPEC;
    pub type Nfcs = crate::EnumBitfieldStruct<u8, Nfcs_SPEC>;
    impl Nfcs {
        #[doc = "In asynchronous mode: Use clock signal divided by 1 with noise filter In simple I2C mode: Setting prohibited"]
        pub const _000: Self = Self::new(0);
        #[doc = "In asynchronous mode: Setting prohibited In simple I2C mode: Use clock signal divided by 1 with noise filter"]
        pub const _001: Self = Self::new(1);
        #[doc = "In asynchronous mode: Setting prohibited In simple I2C mode: Use clock signal divided by 2 with noise filter"]
        pub const _010: Self = Self::new(2);
        #[doc = "In asynchronous mode: Setting prohibited In simple I2C mode: Use clock signal divided by 4 with noise filter"]
        pub const _011: Self = Self::new(3);
        #[doc = "In asynchronous mode: Setting prohibited In simple I2C mode: Use clock signal divided by 8 with noise filter"]
        pub const _100: Self = Self::new(4);
        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Simr1_SPEC;
impl crate::sealed::RegSpec for Simr1_SPEC {
    type DataType = u8;
}
#[doc = "IIC Mode Register 1"]
pub type Simr1 = crate::RegValueT<Simr1_SPEC>;

impl Simr1 {
    #[doc = "Simple IIC Mode Select"]
    #[inline(always)]
    pub fn iicm(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, simr1::Iicm, Simr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,simr1::Iicm, Simr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SDAn Delay Output Select"]
    #[inline(always)]
    pub fn iicdl(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, simr1::Iicdl, Simr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1f,1,0,simr1::Iicdl, Simr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Simr1 {
    #[inline(always)]
    fn default() -> Simr1 {
        <crate::RegValueT<Simr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod simr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicm_SPEC;
    pub type Iicm = crate::EnumBitfieldStruct<u8, Iicm_SPEC>;
    impl Iicm {
        #[doc = "SCMR.SMIF = 0: Asynchronous mode (including multi-processor mode), clock synchronous mode, or simple SPI mode SCMR.SMIF = 1: Smart card interface mode"]
        pub const _0: Self = Self::new(0);
        #[doc = "SCMR.SMIF = 0: Simple IIC mode SCMR.SMIF = 1: Setting prohibited"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicdl_SPEC;
    pub type Iicdl = crate::EnumBitfieldStruct<u8, Iicdl_SPEC>;
    impl Iicdl {
        #[doc = "No output delay"]
        pub const _0_X_00: Self = Self::new(0);
        #[doc = "(IICDL - 1) to (IICDL) cycles"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Simr2_SPEC;
impl crate::sealed::RegSpec for Simr2_SPEC {
    type DataType = u8;
}
#[doc = "IIC Mode Register 2"]
pub type Simr2 = crate::RegValueT<Simr2_SPEC>;

impl Simr2 {
    #[doc = "IIC Interrupt Mode Select"]
    #[inline(always)]
    pub fn iicintm(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, simr2::Iicintm, Simr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,simr2::Iicintm, Simr2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock Synchronization"]
    #[inline(always)]
    pub fn iiccsc(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, simr2::Iiccsc, Simr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,simr2::Iiccsc, Simr2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ACK Transmission Data"]
    #[inline(always)]
    pub fn iicackt(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, simr2::Iicackt, Simr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,simr2::Iicackt, Simr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Simr2 {
    #[inline(always)]
    fn default() -> Simr2 {
        <crate::RegValueT<Simr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod simr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicintm_SPEC;
    pub type Iicintm = crate::EnumBitfieldStruct<u8, Iicintm_SPEC>;
    impl Iicintm {
        #[doc = "Use ACK/NACK interrupts"]
        pub const _0: Self = Self::new(0);
        #[doc = "Use reception and transmission interrupts"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iiccsc_SPEC;
    pub type Iiccsc = crate::EnumBitfieldStruct<u8, Iiccsc_SPEC>;
    impl Iiccsc {
        #[doc = "Do not synchronize with clock signal"]
        pub const _0: Self = Self::new(0);
        #[doc = "Synchronize with clock signal"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicackt_SPEC;
    pub type Iicackt = crate::EnumBitfieldStruct<u8, Iicackt_SPEC>;
    impl Iicackt {
        #[doc = "ACK transmission"]
        pub const _0: Self = Self::new(0);
        #[doc = "NACK transmission and ACK/NACK reception"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Simr3_SPEC;
impl crate::sealed::RegSpec for Simr3_SPEC {
    type DataType = u8;
}
#[doc = "IIC Mode Register 3"]
pub type Simr3 = crate::RegValueT<Simr3_SPEC>;

impl Simr3 {
    #[doc = "Start Condition Generation"]
    #[inline(always)]
    pub fn iicstareq(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, simr3::Iicstareq, Simr3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,simr3::Iicstareq, Simr3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Restart Condition Generation"]
    #[inline(always)]
    pub fn iicrstareq(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, simr3::Iicrstareq, Simr3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,simr3::Iicrstareq, Simr3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Stop Condition Generation"]
    #[inline(always)]
    pub fn iicstpreq(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, simr3::Iicstpreq, Simr3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,simr3::Iicstpreq, Simr3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Issuing of Start, Restart, or Stop Condition Completed Flag"]
    #[inline(always)]
    pub fn iicstif(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, simr3::Iicstif, Simr3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,simr3::Iicstif, Simr3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SDAn Output Select"]
    #[inline(always)]
    pub fn iicsdas(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, simr3::Iicsdas, Simr3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,simr3::Iicsdas, Simr3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SCLn Output Select"]
    #[inline(always)]
    pub fn iicscls(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, simr3::Iicscls, Simr3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,simr3::Iicscls, Simr3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Simr3 {
    #[inline(always)]
    fn default() -> Simr3 {
        <crate::RegValueT<Simr3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod simr3 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicstareq_SPEC;
    pub type Iicstareq = crate::EnumBitfieldStruct<u8, Iicstareq_SPEC>;
    impl Iicstareq {
        #[doc = "Do not generate start condition"]
        pub const _0: Self = Self::new(0);
        #[doc = "Generate start condition"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicrstareq_SPEC;
    pub type Iicrstareq = crate::EnumBitfieldStruct<u8, Iicrstareq_SPEC>;
    impl Iicrstareq {
        #[doc = "Do not generate restart condition"]
        pub const _0: Self = Self::new(0);
        #[doc = "Generate restart condition"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicstpreq_SPEC;
    pub type Iicstpreq = crate::EnumBitfieldStruct<u8, Iicstpreq_SPEC>;
    impl Iicstpreq {
        #[doc = "Do not generate stop condition"]
        pub const _0: Self = Self::new(0);
        #[doc = "Generate stop condition"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicstif_SPEC;
    pub type Iicstif = crate::EnumBitfieldStruct<u8, Iicstif_SPEC>;
    impl Iicstif {
        #[doc = "No requests are being made for generating conditions, or a condition is being generated"]
        pub const _0: Self = Self::new(0);
        #[doc = "Generation of start, restart, or stop condition is complete. When 0 is written to IICSTIF, it is set to 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicsdas_SPEC;
    pub type Iicsdas = crate::EnumBitfieldStruct<u8, Iicsdas_SPEC>;
    impl Iicsdas {
        #[doc = "Output serial data"]
        pub const _00: Self = Self::new(0);
        #[doc = "Generate start, restart, or stop condition"]
        pub const _01: Self = Self::new(1);
        #[doc = "Output low on SDAn pin"]
        pub const _10: Self = Self::new(2);
        #[doc = "Drive SDAn pin to high-impedance state"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicscls_SPEC;
    pub type Iicscls = crate::EnumBitfieldStruct<u8, Iicscls_SPEC>;
    impl Iicscls {
        #[doc = "Output serial clock"]
        pub const _00: Self = Self::new(0);
        #[doc = "Generate start, restart, or stop condition"]
        pub const _01: Self = Self::new(1);
        #[doc = "Output low on SCLn pin"]
        pub const _10: Self = Self::new(2);
        #[doc = "Drive SCLn pin to high-impedance state"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sisr_SPEC;
impl crate::sealed::RegSpec for Sisr_SPEC {
    type DataType = u8;
}
#[doc = "IIC Status Register"]
pub type Sisr = crate::RegValueT<Sisr_SPEC>;

impl Sisr {
    #[doc = "ACK Reception Data Flag"]
    #[inline(always)]
    pub fn iicackr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, sisr::Iicackr, Sisr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,sisr::Iicackr, Sisr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Sisr {
    #[inline(always)]
    fn default() -> Sisr {
        <crate::RegValueT<Sisr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sisr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicackr_SPEC;
    pub type Iicackr = crate::EnumBitfieldStruct<u8, Iicackr_SPEC>;
    impl Iicackr {
        #[doc = "ACK received"]
        pub const _0: Self = Self::new(0);
        #[doc = "NACK received"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spmr_SPEC;
impl crate::sealed::RegSpec for Spmr_SPEC {
    type DataType = u8;
}
#[doc = "SPI Mode Register"]
pub type Spmr = crate::RegValueT<Spmr_SPEC>;

impl Spmr {
    #[doc = "SSn Pin Function Enable"]
    #[inline(always)]
    pub fn sse(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, spmr::Sse, Spmr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,spmr::Sse, Spmr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CTS Enable"]
    #[inline(always)]
    pub fn ctse(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, spmr::Ctse, Spmr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,spmr::Ctse, Spmr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Master Slave Select"]
    #[inline(always)]
    pub fn mss(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, spmr::Mss, Spmr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,spmr::Mss, Spmr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CTS External Pin Enable"]
    #[inline(always)]
    pub fn ctspen(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, spmr::Ctspen, Spmr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,spmr::Ctspen, Spmr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Mode Fault Flag"]
    #[inline(always)]
    pub fn mff(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, spmr::Mff, Spmr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,spmr::Mff, Spmr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock Polarity Select"]
    #[inline(always)]
    pub fn ckpol(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, spmr::Ckpol, Spmr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x1,1,0,spmr::Ckpol, Spmr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock Phase Select"]
    #[inline(always)]
    pub fn ckph(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, spmr::Ckph, Spmr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x1,1,0,spmr::Ckph, Spmr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Spmr {
    #[inline(always)]
    fn default() -> Spmr {
        <crate::RegValueT<Spmr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod spmr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sse_SPEC;
    pub type Sse = crate::EnumBitfieldStruct<u8, Sse_SPEC>;
    impl Sse {
        #[doc = "Disable SSn pin function"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable SSn pin function"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctse_SPEC;
    pub type Ctse = crate::EnumBitfieldStruct<u8, Ctse_SPEC>;
    impl Ctse {
        #[doc = "Disable CTS function (enable RTS output function)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable CTS function"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mss_SPEC;
    pub type Mss = crate::EnumBitfieldStruct<u8, Mss_SPEC>;
    impl Mss {
        #[doc = "Transmit through TXDn pin and receive through RXDn pin (master mode)"]
        pub const _0: Self = Self::new(0);
        #[doc = "Receive through TXDn pin and transmit through RXDn pin (slave mode)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctspen_SPEC;
    pub type Ctspen = crate::EnumBitfieldStruct<u8, Ctspen_SPEC>;
    impl Ctspen {
        #[doc = "Alternate setting to use CTS and RTS functions as either one terminal"]
        pub const _0: Self = Self::new(0);
        #[doc = "Dedicated setting for separately using CTS and RTS functions with 2 terminals These bits for the other SCI channels than SCIn (n = 0, 3 to 5, 9) are reserved."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mff_SPEC;
    pub type Mff = crate::EnumBitfieldStruct<u8, Mff_SPEC>;
    impl Mff {
        #[doc = "No mode fault error"]
        pub const _0: Self = Self::new(0);
        #[doc = "Mode fault error"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ckpol_SPEC;
    pub type Ckpol = crate::EnumBitfieldStruct<u8, Ckpol_SPEC>;
    impl Ckpol {
        #[doc = "Do not invert clock polarity"]
        pub const _0: Self = Self::new(0);
        #[doc = "Invert clock polarity"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ckph_SPEC;
    pub type Ckph = crate::EnumBitfieldStruct<u8, Ckph_SPEC>;
    impl Ckph {
        #[doc = "Do not delay clock"]
        pub const _0: Self = Self::new(0);
        #[doc = "Delay clock"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdrhl_SPEC;
impl crate::sealed::RegSpec for Tdrhl_SPEC {
    type DataType = u16;
}
#[doc = "Transmit Data Register for Non-Manchester Mode (MMR.MANEN = 0)"]
pub type Tdrhl = crate::RegValueT<Tdrhl_SPEC>;

impl Tdrhl {
    #[doc = "Serial Transmit Data"]
    #[inline(always)]
    pub fn tdat(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, Tdrhl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16, Tdrhl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tdrhl {
    #[inline(always)]
    fn default() -> Tdrhl {
        <crate::RegValueT<Tdrhl_SPEC> as RegisterValue<_>>::new(65535)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdrhl_SPEC;
impl crate::sealed::RegSpec for Rdrhl_SPEC {
    type DataType = u16;
}
#[doc = "Receive Data Register for Non-Manchester Mode (MMR.MANEN = 0)"]
pub type Rdrhl = crate::RegValueT<Rdrhl_SPEC>;

impl Rdrhl {
    #[doc = "Serial Receive Data"]
    #[inline(always)]
    pub fn rdat(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, Rdrhl_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16, Rdrhl_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Rdrhl {
    #[inline(always)]
    fn default() -> Rdrhl {
        <crate::RegValueT<Rdrhl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mddr_SPEC;
impl crate::sealed::RegSpec for Mddr_SPEC {
    type DataType = u8;
}
#[doc = "Modulation Duty Register"]
pub type Mddr = crate::RegValueT<Mddr_SPEC>;

impl NoBitfieldReg<Mddr_SPEC> for Mddr {}
impl ::core::default::Default for Mddr {
    #[inline(always)]
    fn default() -> Mddr {
        <crate::RegValueT<Mddr_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Esmer_SPEC;
impl crate::sealed::RegSpec for Esmer_SPEC {
    type DataType = u8;
}
#[doc = "Simple LIN Mode Enable Register"]
pub type Esmer = crate::RegValueT<Esmer_SPEC>;

impl Esmer {
    #[doc = "Simple LIN Mode Enable"]
    #[inline(always)]
    pub fn esme(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, esmer::Esme, Esmer_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,esmer::Esme, Esmer_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Esmer {
    #[inline(always)]
    fn default() -> Esmer {
        <crate::RegValueT<Esmer_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod esmer {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Esme_SPEC;
    pub type Esme = crate::EnumBitfieldStruct<u8, Esme_SPEC>;
    impl Esme {
        #[doc = "The simple LIN mode is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "The simple LIN mode is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr0_SPEC;
impl crate::sealed::RegSpec for Cr0_SPEC {
    type DataType = u8;
}
#[doc = "Control Register 0"]
pub type Cr0 = crate::RegValueT<Cr0_SPEC>;

impl Cr0 {
    #[doc = "Start Frame Status Flag"]
    #[inline(always)]
    pub fn sfsf(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, cr0::Sfsf, Cr0_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,cr0::Sfsf, Cr0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RXDXn Input Status Flag"]
    #[inline(always)]
    pub fn rxdsf(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, cr0::Rxdsf, Cr0_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x1,1,0,cr0::Rxdsf, Cr0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Bit Rate Measurement Enable"]
    #[inline(always)]
    pub fn brme(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, cr0::Brme, Cr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,cr0::Brme, Cr0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cr0 {
    #[inline(always)]
    fn default() -> Cr0 {
        <crate::RegValueT<Cr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfsf_SPEC;
    pub type Sfsf = crate::EnumBitfieldStruct<u8, Sfsf_SPEC>;
    impl Sfsf {
        #[doc = "Start frame detection function is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Start frame detection function is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxdsf_SPEC;
    pub type Rxdsf = crate::EnumBitfieldStruct<u8, Rxdsf_SPEC>;
    impl Rxdsf {
        #[doc = "RXDXn input is enabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "RXDXn input is disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Brme_SPEC;
    pub type Brme = crate::EnumBitfieldStruct<u8, Brme_SPEC>;
    impl Brme {
        #[doc = "Measurement of bit rate is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Measurement of bit rate is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr1_SPEC;
impl crate::sealed::RegSpec for Cr1_SPEC {
    type DataType = u8;
}
#[doc = "Control Register 1"]
pub type Cr1 = crate::RegValueT<Cr1_SPEC>;

impl Cr1 {
    #[doc = "Break Field Enable"]
    #[inline(always)]
    pub fn bfe(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, cr1::Bfe, Cr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,cr1::Bfe, Cr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Control Field 0 Reception Enable"]
    #[inline(always)]
    pub fn cf0re(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, cr1::Cf0Re, Cr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,cr1::Cf0Re, Cr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Control Field 1 Data Register Select"]
    #[inline(always)]
    pub fn cf1ds(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, cr1::Cf1Ds, Cr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,cr1::Cf1Ds, Cr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Priority Interrupt Bit Enable"]
    #[inline(always)]
    pub fn pibe(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, cr1::Pibe, Cr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,cr1::Pibe, Cr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Priority Interrupt Bit Select"]
    #[inline(always)]
    pub fn pibs(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, cr1::Pibs, Cr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,cr1::Pibs, Cr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cr1 {
    #[inline(always)]
    fn default() -> Cr1 {
        <crate::RegValueT<Cr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bfe_SPEC;
    pub type Bfe = crate::EnumBitfieldStruct<u8, Bfe_SPEC>;
    impl Bfe {
        #[doc = "Break Field detection is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Break Field detection is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf0Re_SPEC;
    pub type Cf0Re = crate::EnumBitfieldStruct<u8, Cf0Re_SPEC>;
    impl Cf0Re {
        #[doc = "Reception of Control Field 0 is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Reception of Control Field 0 is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf1Ds_SPEC;
    pub type Cf1Ds = crate::EnumBitfieldStruct<u8, Cf1Ds_SPEC>;
    impl Cf1Ds {
        #[doc = "Selects comparison with the value in PCF1DR"]
        pub const _00: Self = Self::new(0);
        #[doc = "Selects comparison with the value in SCF1DR"]
        pub const _01: Self = Self::new(1);
        #[doc = "Selects comparison with the values in PCF1DR and SCF1DR"]
        pub const _10: Self = Self::new(2);
        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pibe_SPEC;
    pub type Pibe = crate::EnumBitfieldStruct<u8, Pibe_SPEC>;
    impl Pibe {
        #[doc = "The priority interrupt bit is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "The priority interrupt bit is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pibs_SPEC;
    pub type Pibs = crate::EnumBitfieldStruct<u8, Pibs_SPEC>;
    impl Pibs {
        #[doc = "0th bit of Control Field 1"]
        pub const _000: Self = Self::new(0);
        #[doc = "1st bit of Control Field 1"]
        pub const _001: Self = Self::new(1);
        #[doc = "2nd bit of Control Field 1"]
        pub const _010: Self = Self::new(2);
        #[doc = "3rd bit of Control Field 1"]
        pub const _011: Self = Self::new(3);
        #[doc = "4th bit of Control Field 1"]
        pub const _100: Self = Self::new(4);
        #[doc = "5th bit of Control Field 1"]
        pub const _101: Self = Self::new(5);
        #[doc = "6th bit of Control Field 1"]
        pub const _110: Self = Self::new(6);
        #[doc = "7th bit of Control Field 1"]
        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr2_SPEC;
impl crate::sealed::RegSpec for Cr2_SPEC {
    type DataType = u8;
}
#[doc = "Control Register 2"]
pub type Cr2 = crate::RegValueT<Cr2_SPEC>;

impl Cr2 {
    #[doc = "RXDXn Signal Digital Filter Clock Select"]
    #[inline(always)]
    pub fn dfcs(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, cr2::Dfcs, Cr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,cr2::Dfcs, Cr2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bus Collision Detection Clock Select"]
    #[inline(always)]
    pub fn bccs(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, cr2::Bccs, Cr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,cr2::Bccs, Cr2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "RXDXn Reception Sampling Timing Select"]
    #[inline(always)]
    pub fn rts(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, cr2::Rts, Cr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,cr2::Rts, Cr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cr2 {
    #[inline(always)]
    fn default() -> Cr2 {
        <crate::RegValueT<Cr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dfcs_SPEC;
    pub type Dfcs = crate::EnumBitfieldStruct<u8, Dfcs_SPEC>;
    impl Dfcs {
        #[doc = "Filter is disabled"]
        pub const _000: Self = Self::new(0);
        #[doc = "Filter clock is SCI base clock"]
        pub const _001: Self = Self::new(1);
        #[doc = "Filter clock is PCLK/8"]
        pub const _010: Self = Self::new(2);
        #[doc = "Filter clock is PCLK/16"]
        pub const _011: Self = Self::new(3);
        #[doc = "Filter clock is PCLK/32"]
        pub const _100: Self = Self::new(4);
        #[doc = "Filter clock is PCLK/64"]
        pub const _101: Self = Self::new(5);
        #[doc = "Filter clock is PCLK/128"]
        pub const _110: Self = Self::new(6);
        #[doc = "Setting prohibited"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bccs_SPEC;
    pub type Bccs = crate::EnumBitfieldStruct<u8, Bccs_SPEC>;
    impl Bccs {
        #[doc = "SCI base clock"]
        pub const _00: Self = Self::new(0);
        #[doc = "SCI base clock frequency divided by 2"]
        pub const _01: Self = Self::new(1);
        #[doc = "SCI base clock frequency divided by 4"]
        pub const _10: Self = Self::new(2);
        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rts_SPEC;
    pub type Rts = crate::EnumBitfieldStruct<u8, Rts_SPEC>;
    impl Rts {
        #[doc = "Rising edge of the 8th cycle of SCI base clock"]
        pub const _00: Self = Self::new(0);
        #[doc = "Rising edge of the 10th cycle of SCI base clock"]
        pub const _01: Self = Self::new(1);
        #[doc = "Rising edge of the 12th cycle of SCI base clock"]
        pub const _10: Self = Self::new(2);
        #[doc = "Rising edge of the 14th cycle of SCI base clock"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr3_SPEC;
impl crate::sealed::RegSpec for Cr3_SPEC {
    type DataType = u8;
}
#[doc = "Control Register 3"]
pub type Cr3 = crate::RegValueT<Cr3_SPEC>;

impl Cr3 {
    #[doc = "Start Frame Detection Start"]
    #[inline(always)]
    pub fn sdst(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, cr3::Sdst, Cr3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,cr3::Sdst, Cr3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cr3 {
    #[inline(always)]
    fn default() -> Cr3 {
        <crate::RegValueT<Cr3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cr3 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdst_SPEC;
    pub type Sdst = crate::EnumBitfieldStruct<u8, Sdst_SPEC>;
    impl Sdst {
        #[doc = "Detection of start frame is not performed"]
        pub const _0: Self = Self::new(0);
        #[doc = "Detection of start frame is performed"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr_SPEC;
impl crate::sealed::RegSpec for Pcr_SPEC {
    type DataType = u8;
}
#[doc = "Port Control Register"]
pub type Pcr = crate::RegValueT<Pcr_SPEC>;

impl Pcr {
    #[doc = "TXDXn Signal Polarity Select"]
    #[inline(always)]
    pub fn txdxps(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, pcr::Txdxps, Pcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,pcr::Txdxps, Pcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "RXDXn Signal Polarity Select"]
    #[inline(always)]
    pub fn rxdxps(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, pcr::Rxdxps, Pcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,pcr::Rxdxps, Pcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TXDXn/RXDXn Pin Multiplexing Select"]
    #[inline(always)]
    pub fn sharps(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, pcr::Sharps, Pcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,pcr::Sharps, Pcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pcr {
    #[inline(always)]
    fn default() -> Pcr {
        <crate::RegValueT<Pcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Txdxps_SPEC;
    pub type Txdxps = crate::EnumBitfieldStruct<u8, Txdxps_SPEC>;
    impl Txdxps {
        #[doc = "The polarity of TXDXn signal is not inverted for output"]
        pub const _0: Self = Self::new(0);
        #[doc = "The polarity of TXDXn signal is inverted for output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxdxps_SPEC;
    pub type Rxdxps = crate::EnumBitfieldStruct<u8, Rxdxps_SPEC>;
    impl Rxdxps {
        #[doc = "The polarity of RXDXn signal is not inverted for input"]
        pub const _0: Self = Self::new(0);
        #[doc = "The polarity of RXDXn signal is inverted for input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sharps_SPEC;
    pub type Sharps = crate::EnumBitfieldStruct<u8, Sharps_SPEC>;
    impl Sharps {
        #[doc = "The TXDXn and RXDXn pins are independent"]
        pub const _0: Self = Self::new(0);
        #[doc = "The TXDXn and RXDXn signals are multiplexed on the same pin"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr_SPEC;
impl crate::sealed::RegSpec for Icr_SPEC {
    type DataType = u8;
}
#[doc = "Interrupt Control Register"]
pub type Icr = crate::RegValueT<Icr_SPEC>;

impl Icr {
    #[doc = "Break Field Low Width Detected Interrupt Enable"]
    #[inline(always)]
    pub fn bfdie(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, icr::Bfdie, Icr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,icr::Bfdie, Icr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Control Field 0 Match Detected Interrupt Enable"]
    #[inline(always)]
    pub fn cf0mie(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, icr::Cf0Mie, Icr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,icr::Cf0Mie, Icr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Control Field 1 Match Detected Interrupt Enable"]
    #[inline(always)]
    pub fn cf1mie(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, icr::Cf1Mie, Icr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,icr::Cf1Mie, Icr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Priority Interrupt Bit Detected Interrupt Enable"]
    #[inline(always)]
    pub fn pibdie(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, icr::Pibdie, Icr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,icr::Pibdie, Icr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bus Collision Detected Interrupt Enable"]
    #[inline(always)]
    pub fn bcdie(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, icr::Bcdie, Icr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,icr::Bcdie, Icr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Valid Edge Detected Interrupt Enable"]
    #[inline(always)]
    pub fn aedie(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, icr::Aedie, Icr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x1,1,0,icr::Aedie, Icr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Icr {
    #[inline(always)]
    fn default() -> Icr {
        <crate::RegValueT<Icr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bfdie_SPEC;
    pub type Bfdie = crate::EnumBitfieldStruct<u8, Bfdie_SPEC>;
    impl Bfdie {
        #[doc = "Interrupts on detection of the low width for a Break Field are disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupts on detection of the low width for a Break Field are enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf0Mie_SPEC;
    pub type Cf0Mie = crate::EnumBitfieldStruct<u8, Cf0Mie_SPEC>;
    impl Cf0Mie {
        #[doc = "Interrupts on detection of a match with Control Field 0 are disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupts on detection of a match with Control Field 0 are enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf1Mie_SPEC;
    pub type Cf1Mie = crate::EnumBitfieldStruct<u8, Cf1Mie_SPEC>;
    impl Cf1Mie {
        #[doc = "Interrupts on detection of a match with Control Field 1 are disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupts on detection of a match with Control Field 1 are enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pibdie_SPEC;
    pub type Pibdie = crate::EnumBitfieldStruct<u8, Pibdie_SPEC>;
    impl Pibdie {
        #[doc = "Interrupts on detection of the priority interrupt bit are disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupts on detection of the priority interrupt bit are enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bcdie_SPEC;
    pub type Bcdie = crate::EnumBitfieldStruct<u8, Bcdie_SPEC>;
    impl Bcdie {
        #[doc = "Interrupts on detection of a bus collision are disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupts on detection of a bus collision are enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aedie_SPEC;
    pub type Aedie = crate::EnumBitfieldStruct<u8, Aedie_SPEC>;
    impl Aedie {
        #[doc = "Interrupts on detection of a valid edge are disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Interrupts on detection of a valid edge are enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Str_SPEC;
impl crate::sealed::RegSpec for Str_SPEC {
    type DataType = u8;
}
#[doc = "Status Register"]
pub type Str = crate::RegValueT<Str_SPEC>;

impl Str {
    #[doc = "Break Field Low Width Detection Flag"]
    #[inline(always)]
    pub fn bfdf(self) -> crate::common::RegisterFieldBool<0, 1, 0, Str_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Str_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Control Field 0 Match Flag"]
    #[inline(always)]
    pub fn cf0mf(self) -> crate::common::RegisterFieldBool<1, 1, 0, Str_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Str_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Control Field 1 Match Flag"]
    #[inline(always)]
    pub fn cf1mf(self) -> crate::common::RegisterFieldBool<2, 1, 0, Str_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Str_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Priority Interrupt Bit Detection Flag"]
    #[inline(always)]
    pub fn pibdf(self) -> crate::common::RegisterFieldBool<3, 1, 0, Str_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Str_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Bus Collision Detected Flag"]
    #[inline(always)]
    pub fn bcdf(self) -> crate::common::RegisterFieldBool<4, 1, 0, Str_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Str_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Valid Edge Detection Flag"]
    #[inline(always)]
    pub fn aedf(self) -> crate::common::RegisterFieldBool<5, 1, 0, Str_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Str_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Str {
    #[inline(always)]
    fn default() -> Str {
        <crate::RegValueT<Str_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stcr_SPEC;
impl crate::sealed::RegSpec for Stcr_SPEC {
    type DataType = u8;
}
#[doc = "Status Clear Register"]
pub type Stcr = crate::RegValueT<Stcr_SPEC>;

impl Stcr {
    #[doc = "BFDF Clear"]
    #[inline(always)]
    pub fn bfdcl(self) -> crate::common::RegisterFieldBool<0, 1, 0, Stcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Stcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CF0MF Clear"]
    #[inline(always)]
    pub fn cf0mcl(self) -> crate::common::RegisterFieldBool<1, 1, 0, Stcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Stcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CF1MF Clear"]
    #[inline(always)]
    pub fn cf1mcl(self) -> crate::common::RegisterFieldBool<2, 1, 0, Stcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Stcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "PIBDF Clear"]
    #[inline(always)]
    pub fn pibdcl(self) -> crate::common::RegisterFieldBool<3, 1, 0, Stcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Stcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BCDF Clear"]
    #[inline(always)]
    pub fn bcdcl(self) -> crate::common::RegisterFieldBool<4, 1, 0, Stcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Stcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "AEDF Clear"]
    #[inline(always)]
    pub fn aedcl(self) -> crate::common::RegisterFieldBool<5, 1, 0, Stcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Stcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Stcr {
    #[inline(always)]
    fn default() -> Stcr {
        <crate::RegValueT<Stcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cf0Dr_SPEC;
impl crate::sealed::RegSpec for Cf0Dr_SPEC {
    type DataType = u8;
}
#[doc = "Control Field 0 Data Register"]
pub type Cf0Dr = crate::RegValueT<Cf0Dr_SPEC>;

impl NoBitfieldReg<Cf0Dr_SPEC> for Cf0Dr {}
impl ::core::default::Default for Cf0Dr {
    #[inline(always)]
    fn default() -> Cf0Dr {
        <crate::RegValueT<Cf0Dr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cf0Cr_SPEC;
impl crate::sealed::RegSpec for Cf0Cr_SPEC {
    type DataType = u8;
}
#[doc = "Control Field 0 Compare Enable Register"]
pub type Cf0Cr = crate::RegValueT<Cf0Cr_SPEC>;

impl Cf0Cr {
    #[doc = "Control Field 0 Bit 0 Compare Enable"]
    #[inline(always)]
    pub fn cf0ce0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, cf0cr::Cf0Ce0, Cf0Cr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,cf0cr::Cf0Ce0, Cf0Cr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Control Field 1 Bit 0 Compare Enable"]
    #[inline(always)]
    pub fn cf0ce1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, cf0cr::Cf0Ce1, Cf0Cr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,cf0cr::Cf0Ce1, Cf0Cr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Control Field 2 Bit 0 Compare Enable"]
    #[inline(always)]
    pub fn cf0ce2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, cf0cr::Cf0Ce2, Cf0Cr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,cf0cr::Cf0Ce2, Cf0Cr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Control Field 3 Bit 0 Compare Enable"]
    #[inline(always)]
    pub fn cf0ce3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, cf0cr::Cf0Ce3, Cf0Cr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,cf0cr::Cf0Ce3, Cf0Cr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Control Field 4 Bit 0 Compare Enable"]
    #[inline(always)]
    pub fn cf0ce4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, cf0cr::Cf0Ce4, Cf0Cr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,cf0cr::Cf0Ce4, Cf0Cr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Control Field 5 Bit 0 Compare Enable"]
    #[inline(always)]
    pub fn cf0ce5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, cf0cr::Cf0Ce5, Cf0Cr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,cf0cr::Cf0Ce5, Cf0Cr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Control Field 6 Bit 0 Compare Enable"]
    #[inline(always)]
    pub fn cf0ce6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, cf0cr::Cf0Ce6, Cf0Cr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,cf0cr::Cf0Ce6, Cf0Cr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Control Field 7 Bit 0 Compare Enable"]
    #[inline(always)]
    pub fn cf0ce7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, cf0cr::Cf0Ce7, Cf0Cr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,cf0cr::Cf0Ce7, Cf0Cr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cf0Cr {
    #[inline(always)]
    fn default() -> Cf0Cr {
        <crate::RegValueT<Cf0Cr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cf0cr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf0Ce0_SPEC;
    pub type Cf0Ce0 = crate::EnumBitfieldStruct<u8, Cf0Ce0_SPEC>;
    impl Cf0Ce0 {
        #[doc = "Comparison with bit 0 of Control Field 0 disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Comparison with bit 0 of Control Field 0 enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf0Ce1_SPEC;
    pub type Cf0Ce1 = crate::EnumBitfieldStruct<u8, Cf0Ce1_SPEC>;
    impl Cf0Ce1 {
        #[doc = "Comparison with bit 1 of Control Field 0 disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Comparison with bit 1 of Control Field 0 enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf0Ce2_SPEC;
    pub type Cf0Ce2 = crate::EnumBitfieldStruct<u8, Cf0Ce2_SPEC>;
    impl Cf0Ce2 {
        #[doc = "Comparison with bit 2 of Control Field 0 disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Comparison with bit 2 of Control Field 0 enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf0Ce3_SPEC;
    pub type Cf0Ce3 = crate::EnumBitfieldStruct<u8, Cf0Ce3_SPEC>;
    impl Cf0Ce3 {
        #[doc = "Comparison with bit 3 of Control Field 0 disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Comparison with bit 3 of Control Field 0 enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf0Ce4_SPEC;
    pub type Cf0Ce4 = crate::EnumBitfieldStruct<u8, Cf0Ce4_SPEC>;
    impl Cf0Ce4 {
        #[doc = "Comparison with bit 4 of Control Field 0 disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Comparison with bit 4 of Control Field 0 enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf0Ce5_SPEC;
    pub type Cf0Ce5 = crate::EnumBitfieldStruct<u8, Cf0Ce5_SPEC>;
    impl Cf0Ce5 {
        #[doc = "Comparison with bit 5 of Control Field 0 disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Comparison with bit 5 of Control Field 0 enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf0Ce6_SPEC;
    pub type Cf0Ce6 = crate::EnumBitfieldStruct<u8, Cf0Ce6_SPEC>;
    impl Cf0Ce6 {
        #[doc = "Comparison with bit 6 of Control Field 0 disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Comparison with bit 6 of Control Field 0 enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf0Ce7_SPEC;
    pub type Cf0Ce7 = crate::EnumBitfieldStruct<u8, Cf0Ce7_SPEC>;
    impl Cf0Ce7 {
        #[doc = "Comparison with bit 7 of Control Field 0 is disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "Comparison with bit 7 of Control Field 0 enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cf0Rr_SPEC;
impl crate::sealed::RegSpec for Cf0Rr_SPEC {
    type DataType = u8;
}
#[doc = "Control Field 0 Receive Data Register"]
pub type Cf0Rr = crate::RegValueT<Cf0Rr_SPEC>;

impl NoBitfieldReg<Cf0Rr_SPEC> for Cf0Rr {}
impl ::core::default::Default for Cf0Rr {
    #[inline(always)]
    fn default() -> Cf0Rr {
        <crate::RegValueT<Cf0Rr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcf1Dr_SPEC;
impl crate::sealed::RegSpec for Pcf1Dr_SPEC {
    type DataType = u8;
}
#[doc = "Primary Control Field 1 Data Register"]
pub type Pcf1Dr = crate::RegValueT<Pcf1Dr_SPEC>;

impl NoBitfieldReg<Pcf1Dr_SPEC> for Pcf1Dr {}
impl ::core::default::Default for Pcf1Dr {
    #[inline(always)]
    fn default() -> Pcf1Dr {
        <crate::RegValueT<Pcf1Dr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scf1Dr_SPEC;
impl crate::sealed::RegSpec for Scf1Dr_SPEC {
    type DataType = u8;
}
#[doc = "Secondary Control Field 1 Data Register"]
pub type Scf1Dr = crate::RegValueT<Scf1Dr_SPEC>;

impl NoBitfieldReg<Scf1Dr_SPEC> for Scf1Dr {}
impl ::core::default::Default for Scf1Dr {
    #[inline(always)]
    fn default() -> Scf1Dr {
        <crate::RegValueT<Scf1Dr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cf1Cr_SPEC;
impl crate::sealed::RegSpec for Cf1Cr_SPEC {
    type DataType = u8;
}
#[doc = "Control Field 1 Compare Enable Register"]
pub type Cf1Cr = crate::RegValueT<Cf1Cr_SPEC>;

impl Cf1Cr {
    #[doc = "Control Field 1 Bit 0 Compare Enable"]
    #[inline(always)]
    pub fn cf1ce0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, cf1cr::Cf1Ce0, Cf1Cr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,cf1cr::Cf1Ce0, Cf1Cr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Control Field 1 Bit 1 Compare Enable"]
    #[inline(always)]
    pub fn cf1ce1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, cf1cr::Cf1Ce1, Cf1Cr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,cf1cr::Cf1Ce1, Cf1Cr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Control Field 1 Bit 2 Compare Enable"]
    #[inline(always)]
    pub fn cf1ce2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, cf1cr::Cf1Ce2, Cf1Cr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,cf1cr::Cf1Ce2, Cf1Cr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Control Field 1 Bit 3 Compare Enable"]
    #[inline(always)]
    pub fn cf1ce3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, cf1cr::Cf1Ce3, Cf1Cr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,cf1cr::Cf1Ce3, Cf1Cr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Control Field 1 Bit 4 Compare Enable"]
    #[inline(always)]
    pub fn cf1ce4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, cf1cr::Cf1Ce4, Cf1Cr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,cf1cr::Cf1Ce4, Cf1Cr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Control Field 1 Bit 5 Compare Enable"]
    #[inline(always)]
    pub fn cf1ce5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, cf1cr::Cf1Ce5, Cf1Cr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,cf1cr::Cf1Ce5, Cf1Cr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Control Field 1 Bit 6 Compare Enable"]
    #[inline(always)]
    pub fn cf1ce6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, cf1cr::Cf1Ce6, Cf1Cr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,cf1cr::Cf1Ce6, Cf1Cr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Control Field 1 Bit 7 Compare Enable"]
    #[inline(always)]
    pub fn cf1ce7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, cf1cr::Cf1Ce7, Cf1Cr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,cf1cr::Cf1Ce7, Cf1Cr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cf1Cr {
    #[inline(always)]
    fn default() -> Cf1Cr {
        <crate::RegValueT<Cf1Cr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cf1cr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf1Ce0_SPEC;
    pub type Cf1Ce0 = crate::EnumBitfieldStruct<u8, Cf1Ce0_SPEC>;
    impl Cf1Ce0 {
        #[doc = "Comparison with bit 0 of Control Field 1 disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Comparison with bit 0 of Control Field 1 enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf1Ce1_SPEC;
    pub type Cf1Ce1 = crate::EnumBitfieldStruct<u8, Cf1Ce1_SPEC>;
    impl Cf1Ce1 {
        #[doc = "Comparison with bit 1 of Control Field 1 disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Comparison with bit 1 of Control Field 1 enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf1Ce2_SPEC;
    pub type Cf1Ce2 = crate::EnumBitfieldStruct<u8, Cf1Ce2_SPEC>;
    impl Cf1Ce2 {
        #[doc = "Comparison with bit 2 of Control Field 1 disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Comparison with bit 2 of Control Field 1 is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf1Ce3_SPEC;
    pub type Cf1Ce3 = crate::EnumBitfieldStruct<u8, Cf1Ce3_SPEC>;
    impl Cf1Ce3 {
        #[doc = "Comparison with bit 3 of Control Field 1 disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Comparison with bit 3 of Control Field 1 enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf1Ce4_SPEC;
    pub type Cf1Ce4 = crate::EnumBitfieldStruct<u8, Cf1Ce4_SPEC>;
    impl Cf1Ce4 {
        #[doc = "Comparison with bit 4 of Control Field 1 disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Comparison with bit 4 of Control Field 1 enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf1Ce5_SPEC;
    pub type Cf1Ce5 = crate::EnumBitfieldStruct<u8, Cf1Ce5_SPEC>;
    impl Cf1Ce5 {
        #[doc = "Comparison with bit 5 of Control Field 1 disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Comparison with bit 5 of Control Field 1 enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf1Ce6_SPEC;
    pub type Cf1Ce6 = crate::EnumBitfieldStruct<u8, Cf1Ce6_SPEC>;
    impl Cf1Ce6 {
        #[doc = "Comparison with bit 6 of Control Field 1 disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Comparison with bit 6 of Control Field 1 enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf1Ce7_SPEC;
    pub type Cf1Ce7 = crate::EnumBitfieldStruct<u8, Cf1Ce7_SPEC>;
    impl Cf1Ce7 {
        #[doc = "Comparison with bit 7 of Control Field 1 disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Comparison with bit 7 of Control Field 1 enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cf1Rr_SPEC;
impl crate::sealed::RegSpec for Cf1Rr_SPEC {
    type DataType = u8;
}
#[doc = "Control Field 1 Receive Data Register"]
pub type Cf1Rr = crate::RegValueT<Cf1Rr_SPEC>;

impl NoBitfieldReg<Cf1Rr_SPEC> for Cf1Rr {}
impl ::core::default::Default for Cf1Rr {
    #[inline(always)]
    fn default() -> Cf1Rr {
        <crate::RegValueT<Cf1Rr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcr_SPEC;
impl crate::sealed::RegSpec for Tcr_SPEC {
    type DataType = u8;
}
#[doc = "Timer Control Register"]
pub type Tcr = crate::RegValueT<Tcr_SPEC>;

impl Tcr {
    #[doc = "Timer Count Start"]
    #[inline(always)]
    pub fn tcst(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, tcr::Tcst, Tcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,tcr::Tcst, Tcr_SPEC,crate::common::RW>::from_register(self,0)
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
    pub struct Tcst_SPEC;
    pub type Tcst = crate::EnumBitfieldStruct<u8, Tcst_SPEC>;
    impl Tcst {
        #[doc = "Stops the timer counting"]
        pub const _0: Self = Self::new(0);
        #[doc = "Starts the timer counting"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmr_SPEC;
impl crate::sealed::RegSpec for Tmr_SPEC {
    type DataType = u8;
}
#[doc = "Timer Mode Register"]
pub type Tmr = crate::RegValueT<Tmr_SPEC>;

impl Tmr {
    #[doc = "Timer Operating Mode Select"]
    #[inline(always)]
    pub fn toms(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, tmr::Toms, Tmr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,tmr::Toms, Tmr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Counter Write Control"]
    #[inline(always)]
    pub fn twrc(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, tmr::Twrc, Tmr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,tmr::Twrc, Tmr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer Count Clock Source Select"]
    #[inline(always)]
    pub fn tcss(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, tmr::Tcss, Tmr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,tmr::Tcss, Tmr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tmr {
    #[inline(always)]
    fn default() -> Tmr {
        <crate::RegValueT<Tmr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tmr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Toms_SPEC;
    pub type Toms = crate::EnumBitfieldStruct<u8, Toms_SPEC>;
    impl Toms {
        #[doc = "Timer mode"]
        pub const _00: Self = Self::new(0);
        #[doc = "Break Field low width determination mode"]
        pub const _01: Self = Self::new(1);
        #[doc = "Break Field low width output mode"]
        pub const _10: Self = Self::new(2);
        #[doc = "Setting prohibited"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Twrc_SPEC;
    pub type Twrc = crate::EnumBitfieldStruct<u8, Twrc_SPEC>;
    impl Twrc {
        #[doc = "Data is written to the reload register and counter"]
        pub const _0: Self = Self::new(0);
        #[doc = "Data is written to the reload register only"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcss_SPEC;
    pub type Tcss = crate::EnumBitfieldStruct<u8, Tcss_SPEC>;
    impl Tcss {
        #[doc = "PCLK"]
        pub const _000: Self = Self::new(0);
        #[doc = "PCLK/2"]
        pub const _001: Self = Self::new(1);
        #[doc = "PCLK/4"]
        pub const _010: Self = Self::new(2);
        #[doc = "PCLK/8"]
        pub const _011: Self = Self::new(3);
        #[doc = "PCLK/16"]
        pub const _100: Self = Self::new(4);
        #[doc = "PCLK/32"]
        pub const _101: Self = Self::new(5);
        #[doc = "PCLK/64"]
        pub const _110: Self = Self::new(6);
        #[doc = "PCLK/128"]
        pub const _111: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tpre_SPEC;
impl crate::sealed::RegSpec for Tpre_SPEC {
    type DataType = u8;
}
#[doc = "Timer Prescaler Register"]
pub type Tpre = crate::RegValueT<Tpre_SPEC>;

impl NoBitfieldReg<Tpre_SPEC> for Tpre {}
impl ::core::default::Default for Tpre {
    #[inline(always)]
    fn default() -> Tpre {
        <crate::RegValueT<Tpre_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcnt_SPEC;
impl crate::sealed::RegSpec for Tcnt_SPEC {
    type DataType = u8;
}
#[doc = "Timer Count Register"]
pub type Tcnt = crate::RegValueT<Tcnt_SPEC>;

impl NoBitfieldReg<Tcnt_SPEC> for Tcnt {}
impl ::core::default::Default for Tcnt {
    #[inline(always)]
    fn default() -> Tcnt {
        <crate::RegValueT<Tcnt_SPEC> as RegisterValue<_>>::new(255)
    }
}
