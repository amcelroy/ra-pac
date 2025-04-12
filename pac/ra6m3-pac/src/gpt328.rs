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
#[doc = r"General PWM Timer 8 (32-bit Enhanced)"]
unsafe impl ::core::marker::Send for super::Gpt328 {}
unsafe impl ::core::marker::Sync for super::Gpt328 {}
impl super::Gpt328 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "General PWM Timer Write-Protection Register"]
    #[inline(always)]
    pub const fn gtwp(&self) -> &'static crate::common::Reg<self::Gtwp_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtwp_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "General PWM Timer Software Start Register"]
    #[inline(always)]
    pub const fn gtstr(&self) -> &'static crate::common::Reg<self::Gtstr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtstr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "General PWM Timer Software Stop Register"]
    #[inline(always)]
    pub const fn gtstp(&self) -> &'static crate::common::Reg<self::Gtstp_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtstp_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "General PWM Timer Software Clear Register"]
    #[inline(always)]
    pub const fn gtclr(&self) -> &'static crate::common::Reg<self::Gtclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Gtclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "General PWM Timer Start Source Select Register"]
    #[inline(always)]
    pub const fn gtssr(&self) -> &'static crate::common::Reg<self::Gtssr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtssr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "General PWM Timer Stop Source Select Register"]
    #[inline(always)]
    pub const fn gtpsr(&self) -> &'static crate::common::Reg<self::Gtpsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtpsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "General PWM Timer Clear Source Select Register"]
    #[inline(always)]
    pub const fn gtcsr(&self) -> &'static crate::common::Reg<self::Gtcsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtcsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "General PWM Timer Up Count Source Select Register"]
    #[inline(always)]
    pub const fn gtupsr(
        &self,
    ) -> &'static crate::common::Reg<self::Gtupsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtupsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "General PWM Timer Down Count Source Select Register"]
    #[inline(always)]
    pub const fn gtdnsr(
        &self,
    ) -> &'static crate::common::Reg<self::Gtdnsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtdnsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "General PWM Timer Input Capture Source Select Register A"]
    #[inline(always)]
    pub const fn gticasr(
        &self,
    ) -> &'static crate::common::Reg<self::Gticasr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gticasr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "General PWM Timer Input Capture Source Select Register B"]
    #[inline(always)]
    pub const fn gticbsr(
        &self,
    ) -> &'static crate::common::Reg<self::Gticbsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gticbsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "General PWM Timer Control Register"]
    #[inline(always)]
    pub const fn gtcr(&self) -> &'static crate::common::Reg<self::Gtcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "General PWM Timer Count Direction and Duty Setting Register"]
    #[inline(always)]
    pub const fn gtuddtyc(
        &self,
    ) -> &'static crate::common::Reg<self::Gtuddtyc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtuddtyc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "General PWM Timer I/O Control Register"]
    #[inline(always)]
    pub const fn gtior(&self) -> &'static crate::common::Reg<self::Gtior_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtior_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "General PWM Timer Interrupt Output Setting Register"]
    #[inline(always)]
    pub const fn gtintad(
        &self,
    ) -> &'static crate::common::Reg<self::Gtintad_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtintad_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "General PWM Timer Status Register"]
    #[inline(always)]
    pub const fn gtst(&self) -> &'static crate::common::Reg<self::Gtst_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtst_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[doc = "General PWM Timer Buffer Enable Register"]
    #[inline(always)]
    pub const fn gtber(&self) -> &'static crate::common::Reg<self::Gtber_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtber_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "General PWM Timer Counter"]
    #[inline(always)]
    pub const fn gtcnt(&self) -> &'static crate::common::Reg<self::Gtcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[doc = "General PWM Timer Compare Capture Register A"]
    #[inline(always)]
    pub const fn gtccra(
        &self,
    ) -> &'static crate::common::Reg<self::Gtccra_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtccra_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[doc = "General PWM Timer Compare Capture Register B"]
    #[inline(always)]
    pub const fn gtccrb(
        &self,
    ) -> &'static crate::common::Reg<self::Gtccrb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtccrb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[doc = "General PWM Timer Compare Capture Register C"]
    #[inline(always)]
    pub const fn gtccrc(
        &self,
    ) -> &'static crate::common::Reg<self::Gtccrc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtccrc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[doc = "General PWM Timer Compare Capture Register E"]
    #[inline(always)]
    pub const fn gtccre(
        &self,
    ) -> &'static crate::common::Reg<self::Gtccre_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtccre_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[doc = "General PWM Timer Compare Capture Register D"]
    #[inline(always)]
    pub const fn gtccrd(
        &self,
    ) -> &'static crate::common::Reg<self::Gtccrd_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtccrd_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[doc = "General PWM Timer Compare Capture Register F"]
    #[inline(always)]
    pub const fn gtccrf(
        &self,
    ) -> &'static crate::common::Reg<self::Gtccrf_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtccrf_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[doc = "General PWM Timer Cycle Setting Register"]
    #[inline(always)]
    pub const fn gtpr(&self) -> &'static crate::common::Reg<self::Gtpr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtpr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[doc = "General PWM Timer Cycle Setting Buffer Register"]
    #[inline(always)]
    pub const fn gtpbr(&self) -> &'static crate::common::Reg<self::Gtpbr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtpbr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[doc = "General PWM Timer Dead Time Control Register"]
    #[inline(always)]
    pub const fn gtdtcr(
        &self,
    ) -> &'static crate::common::Reg<self::Gtdtcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtdtcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(136usize),
            )
        }
    }

    #[doc = "General PWM Timer Dead Time Value Register U"]
    #[inline(always)]
    pub const fn gtdvu(&self) -> &'static crate::common::Reg<self::Gtdvu_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Gtdvu_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(140usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtwp_SPEC;
impl crate::sealed::RegSpec for Gtwp_SPEC {
    type DataType = u32;
}
#[doc = "General PWM Timer Write-Protection Register"]
pub type Gtwp = crate::RegValueT<Gtwp_SPEC>;

impl Gtwp {
    #[doc = "GTWP Key Code"]
    #[inline(always)]
    pub fn prkey(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, gtwp::Prkey, Gtwp_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,gtwp::Prkey, Gtwp_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Register Write Disable"]
    #[inline(always)]
    pub fn wp(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, gtwp::Wp, Gtwp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,gtwp::Wp, Gtwp_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gtwp {
    #[inline(always)]
    fn default() -> Gtwp {
        <crate::RegValueT<Gtwp_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtwp {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prkey_SPEC;
    pub type Prkey = crate::EnumBitfieldStruct<u8, Prkey_SPEC>;
    impl Prkey {
        #[doc = "Written to these bits, the WP bits write is permitted."]
        pub const _0_X_A_5: Self = Self::new(165);
        #[doc = "The WP bits write is not permitted."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wp_SPEC;
    pub type Wp = crate::EnumBitfieldStruct<u8, Wp_SPEC>;
    impl Wp {
        #[doc = "Enable writes to the register"]
        pub const _0: Self = Self::new(0);
        #[doc = "Disable writes to the register"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtstr_SPEC;
impl crate::sealed::RegSpec for Gtstr_SPEC {
    type DataType = u32;
}
#[doc = "General PWM Timer Software Start Register"]
pub type Gtstr = crate::RegValueT<Gtstr_SPEC>;

impl Gtstr {
    #[doc = "Channel 13 GTCNT Count StartRead data shows each channel\'s counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt13(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, gtstr::Cstrt13, Gtstr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,gtstr::Cstrt13, Gtstr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel 12 GTCNT Count StartRead data shows each channel\'s counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt12(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, gtstr::Cstrt12, Gtstr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,gtstr::Cstrt12, Gtstr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel 11 GTCNT Count StartRead data shows each channel\'s counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, gtstr::Cstrt11, Gtstr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,gtstr::Cstrt11, Gtstr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel 10 GTCNT Count StartRead data shows each channel\'s counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, gtstr::Cstrt10, Gtstr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,gtstr::Cstrt10, Gtstr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel 9 GTCNT Count StartRead data shows each channel\'s counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, gtstr::Cstrt9, Gtstr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,gtstr::Cstrt9, Gtstr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel 8 GTCNT Count StartRead data shows each channel\'s counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, gtstr::Cstrt8, Gtstr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,gtstr::Cstrt8, Gtstr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel 7 GTCNT Count StartRead data shows each channel\'s counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, gtstr::Cstrt7, Gtstr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,gtstr::Cstrt7, Gtstr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel 6 GTCNT Count StartRead data shows each channel\'s counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, gtstr::Cstrt6, Gtstr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,gtstr::Cstrt6, Gtstr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel 5 GTCNT Count StartRead data shows each channel\'s counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, gtstr::Cstrt5, Gtstr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,gtstr::Cstrt5, Gtstr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel 4 GTCNT Count StartRead data shows each channel\'s counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, gtstr::Cstrt4, Gtstr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,gtstr::Cstrt4, Gtstr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel 3 GTCNT Count StartRead data shows each channel\'s counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, gtstr::Cstrt3, Gtstr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,gtstr::Cstrt3, Gtstr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel 2 GTCNT Count StartRead data shows each channel\'s counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, gtstr::Cstrt2, Gtstr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,gtstr::Cstrt2, Gtstr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel 1 GTCNT Count StartRead data shows each channel\'s counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, gtstr::Cstrt1, Gtstr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,gtstr::Cstrt1, Gtstr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel 0 GTCNT Count StartRead data shows each channel\'s counter status (GTCR.CST bit). 0 means counter stop. 1 means counter running."]
    #[inline(always)]
    pub fn cstrt0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, gtstr::Cstrt0, Gtstr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,gtstr::Cstrt0, Gtstr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gtstr {
    #[inline(always)]
    fn default() -> Gtstr {
        <crate::RegValueT<Gtstr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtstr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt13_SPEC;
    pub type Cstrt13 = crate::EnumBitfieldStruct<u8, Cstrt13_SPEC>;
    impl Cstrt13 {
        #[doc = "No effect (write) / counter stop (read)"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT3213.GTCNT counter starts (write) / Counter running (read)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt12_SPEC;
    pub type Cstrt12 = crate::EnumBitfieldStruct<u8, Cstrt12_SPEC>;
    impl Cstrt12 {
        #[doc = "No effect (write) / counter stop (read)"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT3212.GTCNT counter starts (write) / Counter running (read)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt11_SPEC;
    pub type Cstrt11 = crate::EnumBitfieldStruct<u8, Cstrt11_SPEC>;
    impl Cstrt11 {
        #[doc = "No effect (write) / counter stop (read)"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT3211.GTCNT counter starts (write) / Counter running (read)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt10_SPEC;
    pub type Cstrt10 = crate::EnumBitfieldStruct<u8, Cstrt10_SPEC>;
    impl Cstrt10 {
        #[doc = "No effect (write) / counter stop (read)"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT3210.GTCNT counter starts (write) / Counter running (read)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt9_SPEC;
    pub type Cstrt9 = crate::EnumBitfieldStruct<u8, Cstrt9_SPEC>;
    impl Cstrt9 {
        #[doc = "No effect (write) / counter stop (read)"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT329.GTCNT counter starts (write) / Counter running (read)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt8_SPEC;
    pub type Cstrt8 = crate::EnumBitfieldStruct<u8, Cstrt8_SPEC>;
    impl Cstrt8 {
        #[doc = "No effect (write) / counter stop (read)"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT328.GTCNT counter starts (write) / Counter running (read)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt7_SPEC;
    pub type Cstrt7 = crate::EnumBitfieldStruct<u8, Cstrt7_SPEC>;
    impl Cstrt7 {
        #[doc = "No effect (write) / counter stop (read)"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT32E7.GTCNT counter starts (write) / Counter running (read)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt6_SPEC;
    pub type Cstrt6 = crate::EnumBitfieldStruct<u8, Cstrt6_SPEC>;
    impl Cstrt6 {
        #[doc = "No effect (write) / counter stop (read)"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT32E6.GTCNT counter starts (write) / Counter running (read)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt5_SPEC;
    pub type Cstrt5 = crate::EnumBitfieldStruct<u8, Cstrt5_SPEC>;
    impl Cstrt5 {
        #[doc = "No effect (write) / counter stop (read)"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT32E5.GTCNT counter starts (write) / Counter running (read)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt4_SPEC;
    pub type Cstrt4 = crate::EnumBitfieldStruct<u8, Cstrt4_SPEC>;
    impl Cstrt4 {
        #[doc = "No effect (write) / counter stop (read)"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT32E4.GTCNT counter starts (write) / Counter running (read)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt3_SPEC;
    pub type Cstrt3 = crate::EnumBitfieldStruct<u8, Cstrt3_SPEC>;
    impl Cstrt3 {
        #[doc = "No effect (write) / counter stop (read)"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT32EH3.GTCNT counter starts (write) / Counter running (read)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt2_SPEC;
    pub type Cstrt2 = crate::EnumBitfieldStruct<u8, Cstrt2_SPEC>;
    impl Cstrt2 {
        #[doc = "No effect (write) / counter stop (read)"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT32EH2.GTCNT counter starts (write) / Counter running (read)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt1_SPEC;
    pub type Cstrt1 = crate::EnumBitfieldStruct<u8, Cstrt1_SPEC>;
    impl Cstrt1 {
        #[doc = "No effect (write) / counter stop (read)"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT32EH1.GTCNT counter starts (write) / Counter running (read)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt0_SPEC;
    pub type Cstrt0 = crate::EnumBitfieldStruct<u8, Cstrt0_SPEC>;
    impl Cstrt0 {
        #[doc = "No effect (write) / counter stop (read)"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT32EH0.GTCNT counter starts (write) / Counter running (read)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtstp_SPEC;
impl crate::sealed::RegSpec for Gtstp_SPEC {
    type DataType = u32;
}
#[doc = "General PWM Timer Software Stop Register"]
pub type Gtstp = crate::RegValueT<Gtstp_SPEC>;

impl Gtstp {
    #[doc = "Channel 13 GTCNT Count StopRead data shows each channel\'s counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop13(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, gtstp::Cstop13, Gtstp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,gtstp::Cstop13, Gtstp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel 12 GTCNT Count StopRead data shows each channel\'s counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop12(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, gtstp::Cstop12, Gtstp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,gtstp::Cstop12, Gtstp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel 11 GTCNT Count StopRead data shows each channel\'s counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, gtstp::Cstop11, Gtstp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,gtstp::Cstop11, Gtstp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel 10 GTCNT Count StopRead data shows each channel\'s counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, gtstp::Cstop10, Gtstp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,gtstp::Cstop10, Gtstp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel 9 GTCNT Count StopRead data shows each channel\'s counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, gtstp::Cstop9, Gtstp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,gtstp::Cstop9, Gtstp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel 8 GTCNT Count StopRead data shows each channel\'s counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, gtstp::Cstop8, Gtstp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,gtstp::Cstop8, Gtstp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel 7 GTCNT Count StopRead data shows each channel\'s counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, gtstp::Cstop7, Gtstp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,gtstp::Cstop7, Gtstp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel 6 GTCNT Count StopRead data shows each channel\'s counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, gtstp::Cstop6, Gtstp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,gtstp::Cstop6, Gtstp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel 5 GTCNT Count StopRead data shows each channel\'s counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, gtstp::Cstop5, Gtstp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,gtstp::Cstop5, Gtstp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel 4 GTCNT Count StopRead data shows each channel\'s counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, gtstp::Cstop4, Gtstp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,gtstp::Cstop4, Gtstp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel 3 GTCNT Count StopRead data shows each channel\'s counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, gtstp::Cstop3, Gtstp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,gtstp::Cstop3, Gtstp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel 2 GTCNT Count StopRead data shows each channel\'s counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, gtstp::Cstop2, Gtstp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,gtstp::Cstop2, Gtstp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel 1 GTCNT Count StopRead data shows each channel\'s counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, gtstp::Cstop1, Gtstp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,gtstp::Cstop1, Gtstp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel 0 GTCNT Count StopRead data shows each channel\'s counter status (GTCR.CST bit). 0 means counter runnning. 1 means counter stop."]
    #[inline(always)]
    pub fn cstop0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, gtstp::Cstop0, Gtstp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,gtstp::Cstop0, Gtstp_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gtstp {
    #[inline(always)]
    fn default() -> Gtstp {
        <crate::RegValueT<Gtstp_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod gtstp {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop13_SPEC;
    pub type Cstop13 = crate::EnumBitfieldStruct<u8, Cstop13_SPEC>;
    impl Cstop13 {
        #[doc = "No effect (write) / counter running (read)"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT3213.GTCNT counter stops (write) / Counter stop (read)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop12_SPEC;
    pub type Cstop12 = crate::EnumBitfieldStruct<u8, Cstop12_SPEC>;
    impl Cstop12 {
        #[doc = "No effect (write) / counter running (read)"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT3212.GTCNT counter stops (write) / Counter stop (read)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop11_SPEC;
    pub type Cstop11 = crate::EnumBitfieldStruct<u8, Cstop11_SPEC>;
    impl Cstop11 {
        #[doc = "No effect (write) / counter running (read)"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT3211.GTCNT counter stops (write) / Counter stop (read)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop10_SPEC;
    pub type Cstop10 = crate::EnumBitfieldStruct<u8, Cstop10_SPEC>;
    impl Cstop10 {
        #[doc = "No effect (write) / counter running (read)"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT3210.GTCNT counter stops (write) / Counter stop (read)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop9_SPEC;
    pub type Cstop9 = crate::EnumBitfieldStruct<u8, Cstop9_SPEC>;
    impl Cstop9 {
        #[doc = "No effect (write) / counter running (read)"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT329.GTCNT counter stops (write) / Counter stop (read)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop8_SPEC;
    pub type Cstop8 = crate::EnumBitfieldStruct<u8, Cstop8_SPEC>;
    impl Cstop8 {
        #[doc = "No effect (write) / counter running (read)"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT328.GTCNT counter stops (write) / Counter stop (read)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop7_SPEC;
    pub type Cstop7 = crate::EnumBitfieldStruct<u8, Cstop7_SPEC>;
    impl Cstop7 {
        #[doc = "No effect (write) / counter running (read)"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT32E7.GTCNT counter stops (write) / Counter stop (read)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop6_SPEC;
    pub type Cstop6 = crate::EnumBitfieldStruct<u8, Cstop6_SPEC>;
    impl Cstop6 {
        #[doc = "No effect (write) / counter running (read)"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT32E6.GTCNT counter stops (write) / Counter stop (read)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop5_SPEC;
    pub type Cstop5 = crate::EnumBitfieldStruct<u8, Cstop5_SPEC>;
    impl Cstop5 {
        #[doc = "No effect (write) / counter running (read)"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT32E5.GTCNT counter stops (write) / Counter stop (read)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop4_SPEC;
    pub type Cstop4 = crate::EnumBitfieldStruct<u8, Cstop4_SPEC>;
    impl Cstop4 {
        #[doc = "No effect (write) / counter running (read)"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT32E4.GTCNT counter stops (write) / Counter stop (read)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop3_SPEC;
    pub type Cstop3 = crate::EnumBitfieldStruct<u8, Cstop3_SPEC>;
    impl Cstop3 {
        #[doc = "No effect (write) / counter running (read)"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT32EH3.GTCNT counter stops (write) / Counter stop (read)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop2_SPEC;
    pub type Cstop2 = crate::EnumBitfieldStruct<u8, Cstop2_SPEC>;
    impl Cstop2 {
        #[doc = "No effect (write) / counter running (read)"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT32EH2.GTCNT counter stops (write) / Counter stop (read)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop1_SPEC;
    pub type Cstop1 = crate::EnumBitfieldStruct<u8, Cstop1_SPEC>;
    impl Cstop1 {
        #[doc = "No effect (write) / counter running (read)"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT32EH1.GTCNT counter stops (write) / Counter stop (read)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop0_SPEC;
    pub type Cstop0 = crate::EnumBitfieldStruct<u8, Cstop0_SPEC>;
    impl Cstop0 {
        #[doc = "No effect (write) / counter running (read)"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT32EH0.GTCNT counter stops (write) / Counter stop (read)"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtclr_SPEC;
impl crate::sealed::RegSpec for Gtclr_SPEC {
    type DataType = u32;
}
#[doc = "General PWM Timer Software Clear Register"]
pub type Gtclr = crate::RegValueT<Gtclr_SPEC>;

impl Gtclr {
    #[doc = "Channel 13 GTCNT Count Clear"]
    #[inline(always)]
    pub fn cclr13(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, gtclr::Cclr13, Gtclr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<13,0x1,1,0,gtclr::Cclr13, Gtclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Channel 12 GTCNT Count Clear"]
    #[inline(always)]
    pub fn cclr12(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, gtclr::Cclr12, Gtclr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<12,0x1,1,0,gtclr::Cclr12, Gtclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Channel 11 GTCNT Count Clear"]
    #[inline(always)]
    pub fn cclr11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, gtclr::Cclr11, Gtclr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<11,0x1,1,0,gtclr::Cclr11, Gtclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Channel 10 GTCNT Count Clear"]
    #[inline(always)]
    pub fn cclr10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, gtclr::Cclr10, Gtclr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<10,0x1,1,0,gtclr::Cclr10, Gtclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Channel 9 GTCNT Count Clear"]
    #[inline(always)]
    pub fn cclr9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, gtclr::Cclr9, Gtclr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<9,0x1,1,0,gtclr::Cclr9, Gtclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Channel 8 GTCNT Count Clear"]
    #[inline(always)]
    pub fn cclr8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, gtclr::Cclr8, Gtclr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0x1,1,0,gtclr::Cclr8, Gtclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Channel 7 GTCNT Count Clear"]
    #[inline(always)]
    pub fn cclr7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, gtclr::Cclr7, Gtclr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<7,0x1,1,0,gtclr::Cclr7, Gtclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Channel 6 GTCNT Count Clear"]
    #[inline(always)]
    pub fn cclr6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, gtclr::Cclr6, Gtclr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<6,0x1,1,0,gtclr::Cclr6, Gtclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Channel 5 GTCNT Count Clear"]
    #[inline(always)]
    pub fn cclr5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, gtclr::Cclr5, Gtclr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<5,0x1,1,0,gtclr::Cclr5, Gtclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Channel 4 GTCNT Count Clear"]
    #[inline(always)]
    pub fn cclr4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, gtclr::Cclr4, Gtclr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<4,0x1,1,0,gtclr::Cclr4, Gtclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Channel 3 GTCNT Count Clear"]
    #[inline(always)]
    pub fn cclr3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, gtclr::Cclr3, Gtclr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<3,0x1,1,0,gtclr::Cclr3, Gtclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Channel 2 GTCNT Count Clear"]
    #[inline(always)]
    pub fn cclr2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, gtclr::Cclr2, Gtclr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<2,0x1,1,0,gtclr::Cclr2, Gtclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Channel 1 GTCNT Count Clear"]
    #[inline(always)]
    pub fn cclr1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, gtclr::Cclr1, Gtclr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<1,0x1,1,0,gtclr::Cclr1, Gtclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Channel 0 GTCNT Count Clear"]
    #[inline(always)]
    pub fn cclr0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, gtclr::Cclr0, Gtclr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0x1,1,0,gtclr::Cclr0, Gtclr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Gtclr {
    #[inline(always)]
    fn default() -> Gtclr {
        <crate::RegValueT<Gtclr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtclr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr13_SPEC;
    pub type Cclr13 = crate::EnumBitfieldStruct<u8, Cclr13_SPEC>;
    impl Cclr13 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT3213.GTCNT counter clears"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr12_SPEC;
    pub type Cclr12 = crate::EnumBitfieldStruct<u8, Cclr12_SPEC>;
    impl Cclr12 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT3212.GTCNT counter clears"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr11_SPEC;
    pub type Cclr11 = crate::EnumBitfieldStruct<u8, Cclr11_SPEC>;
    impl Cclr11 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT3211.GTCNT counter clears"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr10_SPEC;
    pub type Cclr10 = crate::EnumBitfieldStruct<u8, Cclr10_SPEC>;
    impl Cclr10 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT3210.GTCNT counter clears"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr9_SPEC;
    pub type Cclr9 = crate::EnumBitfieldStruct<u8, Cclr9_SPEC>;
    impl Cclr9 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT329.GTCNT counter clears"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr8_SPEC;
    pub type Cclr8 = crate::EnumBitfieldStruct<u8, Cclr8_SPEC>;
    impl Cclr8 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT328.GTCNT counter clears"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr7_SPEC;
    pub type Cclr7 = crate::EnumBitfieldStruct<u8, Cclr7_SPEC>;
    impl Cclr7 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT32E7.GTCNT counter clears"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr6_SPEC;
    pub type Cclr6 = crate::EnumBitfieldStruct<u8, Cclr6_SPEC>;
    impl Cclr6 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT32E6.GTCNT counter clears"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr5_SPEC;
    pub type Cclr5 = crate::EnumBitfieldStruct<u8, Cclr5_SPEC>;
    impl Cclr5 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT32E5.GTCNT counter clears"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr4_SPEC;
    pub type Cclr4 = crate::EnumBitfieldStruct<u8, Cclr4_SPEC>;
    impl Cclr4 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT32E4.GTCNT counter clears"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr3_SPEC;
    pub type Cclr3 = crate::EnumBitfieldStruct<u8, Cclr3_SPEC>;
    impl Cclr3 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT32EH3.GTCNT counter clears"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr2_SPEC;
    pub type Cclr2 = crate::EnumBitfieldStruct<u8, Cclr2_SPEC>;
    impl Cclr2 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT32EH2.GTCNT counter clears"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr1_SPEC;
    pub type Cclr1 = crate::EnumBitfieldStruct<u8, Cclr1_SPEC>;
    impl Cclr1 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT32EH1.GTCNT counter clears"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr0_SPEC;
    pub type Cclr0 = crate::EnumBitfieldStruct<u8, Cclr0_SPEC>;
    impl Cclr0 {
        #[doc = "No effect"]
        pub const _0: Self = Self::new(0);
        #[doc = "GPT32EH0.GTCNT counter clears"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtssr_SPEC;
impl crate::sealed::RegSpec for Gtssr_SPEC {
    type DataType = u32;
}
#[doc = "General PWM Timer Start Source Select Register"]
pub type Gtssr = crate::RegValueT<Gtssr_SPEC>;

impl Gtssr {
    #[doc = "Software Source Counter Start Enable"]
    #[inline(always)]
    pub fn cstrt(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, gtssr::Cstrt, Gtssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,gtssr::Cstrt, Gtssr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELC_GPTH Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselch(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, gtssr::Sselch, Gtssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,gtssr::Sselch, Gtssr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELC_GPTG Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselcg(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, gtssr::Sselcg, Gtssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,gtssr::Sselcg, Gtssr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELC_GPTF Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselcf(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, gtssr::Sselcf, Gtssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,gtssr::Sselcf, Gtssr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELC_GPTE Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselce(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, gtssr::Sselce, Gtssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,gtssr::Sselce, Gtssr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELC_GPTD Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselcd(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, gtssr::Sselcd, Gtssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,gtssr::Sselcd, Gtssr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELC_GPTC Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselcc(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, gtssr::Sselcc, Gtssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,gtssr::Sselcc, Gtssr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELC_GPTB Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselcb(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, gtssr::Sselcb, Gtssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,gtssr::Sselcb, Gtssr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELC_GPTA Event Source Counter Start Enable"]
    #[inline(always)]
    pub fn sselca(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, gtssr::Sselca, Gtssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,gtssr::Sselca, Gtssr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscbfah(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, gtssr::Sscbfah, Gtssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,gtssr::Sscbfah, Gtssr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscbfal(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, gtssr::Sscbfal, Gtssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,gtssr::Sscbfal, Gtssr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscbrah(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, gtssr::Sscbrah, Gtssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,gtssr::Sscbrah, Gtssr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscbral(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, gtssr::Sscbral, Gtssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,gtssr::Sscbral, Gtssr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscafbh(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, gtssr::Sscafbh, Gtssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,gtssr::Sscafbh, Gtssr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscafbl(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, gtssr::Sscafbl, Gtssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,gtssr::Sscafbl, Gtssr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscarbh(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, gtssr::Sscarbh, Gtssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,gtssr::Sscarbh, Gtssr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Start Enable"]
    #[inline(always)]
    pub fn sscarbl(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, gtssr::Sscarbl, Gtssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,gtssr::Sscarbl, Gtssr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGD Pin Falling Input Source Counter Start Enable"]
    #[inline(always)]
    pub fn ssgtrgdf(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, gtssr::Ssgtrgdf, Gtssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,gtssr::Ssgtrgdf, Gtssr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGD Pin Rising Input Source Counter Start Enable"]
    #[inline(always)]
    pub fn ssgtrgdr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, gtssr::Ssgtrgdr, Gtssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,gtssr::Ssgtrgdr, Gtssr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGC Pin Falling Input Source Counter Start Enable"]
    #[inline(always)]
    pub fn ssgtrgcf(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, gtssr::Ssgtrgcf, Gtssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,gtssr::Ssgtrgcf, Gtssr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGC Pin Rising Input Source Counter Start Enable"]
    #[inline(always)]
    pub fn ssgtrgcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, gtssr::Ssgtrgcr, Gtssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,gtssr::Ssgtrgcr, Gtssr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGB Pin Falling Input Source Counter Start Enable"]
    #[inline(always)]
    pub fn ssgtrgbf(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, gtssr::Ssgtrgbf, Gtssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,gtssr::Ssgtrgbf, Gtssr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGB Pin Rising Input Source Counter Start Enable"]
    #[inline(always)]
    pub fn ssgtrgbr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, gtssr::Ssgtrgbr, Gtssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,gtssr::Ssgtrgbr, Gtssr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGA Pin Falling Input Source Counter Start Enable"]
    #[inline(always)]
    pub fn ssgtrgaf(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, gtssr::Ssgtrgaf, Gtssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,gtssr::Ssgtrgaf, Gtssr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGA Pin Rising Input Source Counter Start Enable"]
    #[inline(always)]
    pub fn ssgtrgar(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, gtssr::Ssgtrgar, Gtssr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,gtssr::Ssgtrgar, Gtssr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gtssr {
    #[inline(always)]
    fn default() -> Gtssr {
        <crate::RegValueT<Gtssr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtssr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstrt_SPEC;
    pub type Cstrt = crate::EnumBitfieldStruct<u8, Cstrt_SPEC>;
    impl Cstrt {
        #[doc = "Disable counter start by the GTSTR register"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter start by the GTSTR register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sselch_SPEC;
    pub type Sselch = crate::EnumBitfieldStruct<u8, Sselch_SPEC>;
    impl Sselch {
        #[doc = "Disable counter start on ELC_GPTH input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter start on ELC_GPTH input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sselcg_SPEC;
    pub type Sselcg = crate::EnumBitfieldStruct<u8, Sselcg_SPEC>;
    impl Sselcg {
        #[doc = "Disable counter start on ELC_GPTG input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter start on ELC_GPTG input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sselcf_SPEC;
    pub type Sselcf = crate::EnumBitfieldStruct<u8, Sselcf_SPEC>;
    impl Sselcf {
        #[doc = "Disable counter start on ELC_GPTF input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter start on ELC_GPTF input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sselce_SPEC;
    pub type Sselce = crate::EnumBitfieldStruct<u8, Sselce_SPEC>;
    impl Sselce {
        #[doc = "Disable counter start on ELC_GPTE input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter start on ELC_GPTE input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sselcd_SPEC;
    pub type Sselcd = crate::EnumBitfieldStruct<u8, Sselcd_SPEC>;
    impl Sselcd {
        #[doc = "Disable counter start on ELC_GPTD input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter start on ELC_GPTD input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sselcc_SPEC;
    pub type Sselcc = crate::EnumBitfieldStruct<u8, Sselcc_SPEC>;
    impl Sselcc {
        #[doc = "Disable counter start on ELC_GPTC input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter start on ELC_GPTC input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sselcb_SPEC;
    pub type Sselcb = crate::EnumBitfieldStruct<u8, Sselcb_SPEC>;
    impl Sselcb {
        #[doc = "Disable counter start on ELC_GPTB input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter start on ELC_GPTB input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sselca_SPEC;
    pub type Sselca = crate::EnumBitfieldStruct<u8, Sselca_SPEC>;
    impl Sselca {
        #[doc = "Disable counter start on ELC_GPTA input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter start on ELC_GPTA input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sscbfah_SPEC;
    pub type Sscbfah = crate::EnumBitfieldStruct<u8, Sscbfah_SPEC>;
    impl Sscbfah {
        #[doc = "Disable counter start on the falling edge of GTIOCB input when GTIOCA input is 1"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter start on the falling edge of GTIOCB input when GTIOCA input is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sscbfal_SPEC;
    pub type Sscbfal = crate::EnumBitfieldStruct<u8, Sscbfal_SPEC>;
    impl Sscbfal {
        #[doc = "Disable counter start on the falling edge of GTIOCB input when GTIOCA input is 0"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter start on the falling edge of GTIOCB input when GTIOCA input is 0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sscbrah_SPEC;
    pub type Sscbrah = crate::EnumBitfieldStruct<u8, Sscbrah_SPEC>;
    impl Sscbrah {
        #[doc = "Disable counter start on the rising edge of GTIOCB input when GTIOCA input is 1"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter start on the rising edge of GTIOCB input when GTIOCA input is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sscbral_SPEC;
    pub type Sscbral = crate::EnumBitfieldStruct<u8, Sscbral_SPEC>;
    impl Sscbral {
        #[doc = "Disable counter start on the rising edge of GTIOCB input when GTIOCA input is 0"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter start on the rising edge of GTIOCB input when GTIOCA input is 0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sscafbh_SPEC;
    pub type Sscafbh = crate::EnumBitfieldStruct<u8, Sscafbh_SPEC>;
    impl Sscafbh {
        #[doc = "Disable counter start on the falling edge of GTIOCA input when GTIOCB input is 1"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter start on the falling edge of GTIOCA input when GTIOCB input is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sscafbl_SPEC;
    pub type Sscafbl = crate::EnumBitfieldStruct<u8, Sscafbl_SPEC>;
    impl Sscafbl {
        #[doc = "Disable counter start on the falling edge of GTIOCA input when GTIOCB input is 0"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter start on the falling edge of GTIOCA input when GTIOCB input is 0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sscarbh_SPEC;
    pub type Sscarbh = crate::EnumBitfieldStruct<u8, Sscarbh_SPEC>;
    impl Sscarbh {
        #[doc = "Disable counter start on the rising edge of GTIOCA input when GTIOCB input is 1"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter start on the rising edge of GTIOCA input when GTIOCB input is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sscarbl_SPEC;
    pub type Sscarbl = crate::EnumBitfieldStruct<u8, Sscarbl_SPEC>;
    impl Sscarbl {
        #[doc = "Disable counter start on the rising edge of GTIOCA input when GTIOCB input is 0"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter start on the rising edge of GTIOCA input when GTIOCB input is 0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssgtrgdf_SPEC;
    pub type Ssgtrgdf = crate::EnumBitfieldStruct<u8, Ssgtrgdf_SPEC>;
    impl Ssgtrgdf {
        #[doc = "Disable counter start on the falling edge of GTETRGD input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter start on the falling edge of GTETRGD input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssgtrgdr_SPEC;
    pub type Ssgtrgdr = crate::EnumBitfieldStruct<u8, Ssgtrgdr_SPEC>;
    impl Ssgtrgdr {
        #[doc = "Disable counter start on the rising edge of GTETRGD input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter start on the rising edge of GTETRGD input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssgtrgcf_SPEC;
    pub type Ssgtrgcf = crate::EnumBitfieldStruct<u8, Ssgtrgcf_SPEC>;
    impl Ssgtrgcf {
        #[doc = "Disable counter start on the falling edge of GTETRGC input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter start on the falling edge of GTETRGC input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssgtrgcr_SPEC;
    pub type Ssgtrgcr = crate::EnumBitfieldStruct<u8, Ssgtrgcr_SPEC>;
    impl Ssgtrgcr {
        #[doc = "Disable counter start on the rising edge of GTETRGC input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter start on the rising edge of GTETRGC input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssgtrgbf_SPEC;
    pub type Ssgtrgbf = crate::EnumBitfieldStruct<u8, Ssgtrgbf_SPEC>;
    impl Ssgtrgbf {
        #[doc = "Disable counter start on the falling edge of GTETRGB input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter start on the falling edge of GTETRGB input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssgtrgbr_SPEC;
    pub type Ssgtrgbr = crate::EnumBitfieldStruct<u8, Ssgtrgbr_SPEC>;
    impl Ssgtrgbr {
        #[doc = "Disable counter start on the rising edge of GTETRGB input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter start on the rising edge of GTETRGB input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssgtrgaf_SPEC;
    pub type Ssgtrgaf = crate::EnumBitfieldStruct<u8, Ssgtrgaf_SPEC>;
    impl Ssgtrgaf {
        #[doc = "Disable counter start on the falling edge of GTETRGA input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter start on the falling edge of GTETRGA input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssgtrgar_SPEC;
    pub type Ssgtrgar = crate::EnumBitfieldStruct<u8, Ssgtrgar_SPEC>;
    impl Ssgtrgar {
        #[doc = "Disable counter start on the rising edge of GTETRGA input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter start on the rising edge of GTETRGA input."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtpsr_SPEC;
impl crate::sealed::RegSpec for Gtpsr_SPEC {
    type DataType = u32;
}
#[doc = "General PWM Timer Stop Source Select Register"]
pub type Gtpsr = crate::RegValueT<Gtpsr_SPEC>;

impl Gtpsr {
    #[doc = "Software Source Counter Stop Enable"]
    #[inline(always)]
    pub fn cstop(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, gtpsr::Cstop, Gtpsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,gtpsr::Cstop, Gtpsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELC_GPTH Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselch(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, gtpsr::Pselch, Gtpsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,gtpsr::Pselch, Gtpsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELC_GPTG Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselcg(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, gtpsr::Pselcg, Gtpsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,gtpsr::Pselcg, Gtpsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELC_GPTF Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselcf(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, gtpsr::Pselcf, Gtpsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,gtpsr::Pselcf, Gtpsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELC_GPTE Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselce(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, gtpsr::Pselce, Gtpsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,gtpsr::Pselce, Gtpsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELC_GPTD Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselcd(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, gtpsr::Pselcd, Gtpsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,gtpsr::Pselcd, Gtpsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELC_GPTC Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselcc(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, gtpsr::Pselcc, Gtpsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,gtpsr::Pselcc, Gtpsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELC_GPTB Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselcb(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, gtpsr::Pselcb, Gtpsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,gtpsr::Pselcb, Gtpsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELC_GPTA Event Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pselca(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, gtpsr::Pselca, Gtpsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,gtpsr::Pselca, Gtpsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscbfah(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, gtpsr::Pscbfah, Gtpsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,gtpsr::Pscbfah, Gtpsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscbfal(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, gtpsr::Pscbfal, Gtpsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,gtpsr::Pscbfal, Gtpsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscbrah(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, gtpsr::Pscbrah, Gtpsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,gtpsr::Pscbrah, Gtpsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscbral(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, gtpsr::Pscbral, Gtpsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,gtpsr::Pscbral, Gtpsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscafbh(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, gtpsr::Pscafbh, Gtpsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,gtpsr::Pscafbh, Gtpsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscafbl(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, gtpsr::Pscafbl, Gtpsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,gtpsr::Pscafbl, Gtpsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscarbh(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, gtpsr::Pscarbh, Gtpsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,gtpsr::Pscarbh, Gtpsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Stop Enable"]
    #[inline(always)]
    pub fn pscarbl(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, gtpsr::Pscarbl, Gtpsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,gtpsr::Pscarbl, Gtpsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGD Pin Falling Input Source Counter Stop Enable"]
    #[inline(always)]
    pub fn psgtrgdf(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, gtpsr::Psgtrgdf, Gtpsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,gtpsr::Psgtrgdf, Gtpsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGD Pin Rising Input Source Counter Stop Enable"]
    #[inline(always)]
    pub fn psgtrgdr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, gtpsr::Psgtrgdr, Gtpsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,gtpsr::Psgtrgdr, Gtpsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGC Pin Falling Input Source Counter Stop Enable"]
    #[inline(always)]
    pub fn psgtrgcf(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, gtpsr::Psgtrgcf, Gtpsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,gtpsr::Psgtrgcf, Gtpsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGC Pin Rising Input Source Counter Stop Enable"]
    #[inline(always)]
    pub fn psgtrgcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, gtpsr::Psgtrgcr, Gtpsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,gtpsr::Psgtrgcr, Gtpsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGB Pin Falling Input Source Counter Stop Enable"]
    #[inline(always)]
    pub fn psgtrgbf(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, gtpsr::Psgtrgbf, Gtpsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,gtpsr::Psgtrgbf, Gtpsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGB Pin Rising Input Source Counter Stop Enable"]
    #[inline(always)]
    pub fn psgtrgbr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, gtpsr::Psgtrgbr, Gtpsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,gtpsr::Psgtrgbr, Gtpsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGA Pin Falling Input Source Counter Stop Enable"]
    #[inline(always)]
    pub fn psgtrgaf(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, gtpsr::Psgtrgaf, Gtpsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,gtpsr::Psgtrgaf, Gtpsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGA Pin Rising Input Source Counter Stop Enable"]
    #[inline(always)]
    pub fn psgtrgar(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, gtpsr::Psgtrgar, Gtpsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,gtpsr::Psgtrgar, Gtpsr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gtpsr {
    #[inline(always)]
    fn default() -> Gtpsr {
        <crate::RegValueT<Gtpsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtpsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cstop_SPEC;
    pub type Cstop = crate::EnumBitfieldStruct<u8, Cstop_SPEC>;
    impl Cstop {
        #[doc = "Disable counter stop by the GTSTP register"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter stop by the GTSTP register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pselch_SPEC;
    pub type Pselch = crate::EnumBitfieldStruct<u8, Pselch_SPEC>;
    impl Pselch {
        #[doc = "Disable counter stop on ELC_GPTH input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter stop on ELCH event inpu"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pselcg_SPEC;
    pub type Pselcg = crate::EnumBitfieldStruct<u8, Pselcg_SPEC>;
    impl Pselcg {
        #[doc = "Disable counter stop on ELC_GPTG input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter stop on ELC_GPTG input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pselcf_SPEC;
    pub type Pselcf = crate::EnumBitfieldStruct<u8, Pselcf_SPEC>;
    impl Pselcf {
        #[doc = "Disable counter stop on ELC_GPTF input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter stop on ELC_GPTF input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pselce_SPEC;
    pub type Pselce = crate::EnumBitfieldStruct<u8, Pselce_SPEC>;
    impl Pselce {
        #[doc = "Disable counter stop on ELC_GPTE input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter stop on ELC_GPTE input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pselcd_SPEC;
    pub type Pselcd = crate::EnumBitfieldStruct<u8, Pselcd_SPEC>;
    impl Pselcd {
        #[doc = "Disable counter stop on ELC_GPTD input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter stop on ELC_GPTD input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pselcc_SPEC;
    pub type Pselcc = crate::EnumBitfieldStruct<u8, Pselcc_SPEC>;
    impl Pselcc {
        #[doc = "Disable counter stop on ELC_GPTC input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter stop on ELC_GPTC input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pselcb_SPEC;
    pub type Pselcb = crate::EnumBitfieldStruct<u8, Pselcb_SPEC>;
    impl Pselcb {
        #[doc = "Disable counter stop on ELC_GPTB input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter stop on ELC_GPTB input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pselca_SPEC;
    pub type Pselca = crate::EnumBitfieldStruct<u8, Pselca_SPEC>;
    impl Pselca {
        #[doc = "Disable counter stop on ELC_GPTA input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter stop on ELC_GPTA input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pscbfah_SPEC;
    pub type Pscbfah = crate::EnumBitfieldStruct<u8, Pscbfah_SPEC>;
    impl Pscbfah {
        #[doc = "Disable counter stop on the falling edge of GTIOCB input when GTIOCA input is 1"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter stop on the falling edge of GTIOCB input when GTIOCA input is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pscbfal_SPEC;
    pub type Pscbfal = crate::EnumBitfieldStruct<u8, Pscbfal_SPEC>;
    impl Pscbfal {
        #[doc = "Disable counter stop on the falling edge of GTIOCB input when GTIOCA input is 0"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter stop on the falling edge of GTIOCB input when GTIOCA input is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pscbrah_SPEC;
    pub type Pscbrah = crate::EnumBitfieldStruct<u8, Pscbrah_SPEC>;
    impl Pscbrah {
        #[doc = "Disable counter stop on the rising edge of GTIOCB input when GTIOCA input is 1"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter stop on the rising edge of GTIOCB input when GTIOCA input is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pscbral_SPEC;
    pub type Pscbral = crate::EnumBitfieldStruct<u8, Pscbral_SPEC>;
    impl Pscbral {
        #[doc = "Disable counter stop on the rising edge of GTIOCB input when GTIOCA input is 0"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter stop on the rising edge of GTIOCB input when GTIOCA input is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pscafbh_SPEC;
    pub type Pscafbh = crate::EnumBitfieldStruct<u8, Pscafbh_SPEC>;
    impl Pscafbh {
        #[doc = "Disable counter stop on the falling edge of GTIOCA input when GTIOCB input is 1"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter stop on the falling edge of GTIOCA input when GTIOCB input is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pscafbl_SPEC;
    pub type Pscafbl = crate::EnumBitfieldStruct<u8, Pscafbl_SPEC>;
    impl Pscafbl {
        #[doc = "Disable counter stop on the falling edge of GTIOCA input when GTIOCB input is 0"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter stop on the falling edge of GTIOCA input when GTIOCB input is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pscarbh_SPEC;
    pub type Pscarbh = crate::EnumBitfieldStruct<u8, Pscarbh_SPEC>;
    impl Pscarbh {
        #[doc = "Disable counter stop on the rising edge of GTIOCA input when GTIOCB input is 1"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter stop on the rising edge of GTIOCA input when GTIOCB input is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pscarbl_SPEC;
    pub type Pscarbl = crate::EnumBitfieldStruct<u8, Pscarbl_SPEC>;
    impl Pscarbl {
        #[doc = "Disable counter stop on the rising edge of GTIOCA input when GTIOCB input is 0"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter stop on the rising edge of GTIOCA input when GTIOCB input is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psgtrgdf_SPEC;
    pub type Psgtrgdf = crate::EnumBitfieldStruct<u8, Psgtrgdf_SPEC>;
    impl Psgtrgdf {
        #[doc = "Disable counter stop on the falling edge of GTETRGD input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter stop on the falling edge of GTETRGD input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psgtrgdr_SPEC;
    pub type Psgtrgdr = crate::EnumBitfieldStruct<u8, Psgtrgdr_SPEC>;
    impl Psgtrgdr {
        #[doc = "Disable counter stop on the rising edge of GTETRGD input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter stop on the rising edge of GTETRGD input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psgtrgcf_SPEC;
    pub type Psgtrgcf = crate::EnumBitfieldStruct<u8, Psgtrgcf_SPEC>;
    impl Psgtrgcf {
        #[doc = "Disable counter stop on the falling edge of GTETRGC input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter stop on the falling edge of GTETRGC input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psgtrgcr_SPEC;
    pub type Psgtrgcr = crate::EnumBitfieldStruct<u8, Psgtrgcr_SPEC>;
    impl Psgtrgcr {
        #[doc = "Disable counter stop on the rising edge of GTETRGC input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter stop on the rising edge of GTETRGC input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psgtrgbf_SPEC;
    pub type Psgtrgbf = crate::EnumBitfieldStruct<u8, Psgtrgbf_SPEC>;
    impl Psgtrgbf {
        #[doc = "Disable counter stop on the falling edge of GTETRGB input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter stop on the falling edge of GTETRGB input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psgtrgbr_SPEC;
    pub type Psgtrgbr = crate::EnumBitfieldStruct<u8, Psgtrgbr_SPEC>;
    impl Psgtrgbr {
        #[doc = "Disable counter stop on the rising edge of GTETRGB input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter stop on the rising edge of GTETRGB input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psgtrgaf_SPEC;
    pub type Psgtrgaf = crate::EnumBitfieldStruct<u8, Psgtrgaf_SPEC>;
    impl Psgtrgaf {
        #[doc = "Disable counter stop on the falling edge of GTETRGA input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter stop on the falling edge of GTETRGA input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psgtrgar_SPEC;
    pub type Psgtrgar = crate::EnumBitfieldStruct<u8, Psgtrgar_SPEC>;
    impl Psgtrgar {
        #[doc = "Disable counter stop on the rising edge of GTETRGA input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter stop on the rising edge of GTETRGA input"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtcsr_SPEC;
impl crate::sealed::RegSpec for Gtcsr_SPEC {
    type DataType = u32;
}
#[doc = "General PWM Timer Clear Source Select Register"]
pub type Gtcsr = crate::RegValueT<Gtcsr_SPEC>;

impl Gtcsr {
    #[doc = "Software Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cclr(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, gtcsr::Cclr, Gtcsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,gtcsr::Cclr, Gtcsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELC_GPTH Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselch(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, gtcsr::Cselch, Gtcsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,gtcsr::Cselch, Gtcsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELC_GPTG Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselcg(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, gtcsr::Cselcg, Gtcsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,gtcsr::Cselcg, Gtcsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELC_GPTF Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselcf(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, gtcsr::Cselcf, Gtcsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,gtcsr::Cselcf, Gtcsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELC_GPTE Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselce(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, gtcsr::Cselce, Gtcsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,gtcsr::Cselce, Gtcsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELC_GPTD Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselcd(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, gtcsr::Cselcd, Gtcsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,gtcsr::Cselcd, Gtcsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELC_GPTC Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselcc(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, gtcsr::Cselcc, Gtcsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,gtcsr::Cselcc, Gtcsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELC_GPTB Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselcb(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, gtcsr::Cselcb, Gtcsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,gtcsr::Cselcb, Gtcsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELC_GPTA Event Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cselca(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, gtcsr::Cselca, Gtcsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,gtcsr::Cselca, Gtcsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscbfah(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, gtcsr::Cscbfah, Gtcsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,gtcsr::Cscbfah, Gtcsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscbfal(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, gtcsr::Cscbfal, Gtcsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,gtcsr::Cscbfal, Gtcsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscbrah(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, gtcsr::Cscbrah, Gtcsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,gtcsr::Cscbrah, Gtcsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscbral(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, gtcsr::Cscbral, Gtcsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,gtcsr::Cscbral, Gtcsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscafbh(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, gtcsr::Cscafbh, Gtcsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,gtcsr::Cscafbh, Gtcsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscafbl(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, gtcsr::Cscafbl, Gtcsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,gtcsr::Cscafbl, Gtcsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscarbh(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, gtcsr::Cscarbh, Gtcsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,gtcsr::Cscarbh, Gtcsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Clear Enable"]
    #[inline(always)]
    pub fn cscarbl(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, gtcsr::Cscarbl, Gtcsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,gtcsr::Cscarbl, Gtcsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGD Pin Falling Input Source Counter Clear Enable"]
    #[inline(always)]
    pub fn csgtrgdf(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, gtcsr::Csgtrgdf, Gtcsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,gtcsr::Csgtrgdf, Gtcsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGD Pin Rising Input Source Counter Clear Enable"]
    #[inline(always)]
    pub fn csgtrgdr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, gtcsr::Csgtrgdr, Gtcsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,gtcsr::Csgtrgdr, Gtcsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGC Pin Falling Input Source Counter Clear Enable"]
    #[inline(always)]
    pub fn csgtrgcf(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, gtcsr::Csgtrgcf, Gtcsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,gtcsr::Csgtrgcf, Gtcsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGC Pin Rising Input Source Counter Clear Enable"]
    #[inline(always)]
    pub fn csgtrgcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, gtcsr::Csgtrgcr, Gtcsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,gtcsr::Csgtrgcr, Gtcsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGB Pin Falling Input Source Counter Clear Enable"]
    #[inline(always)]
    pub fn csgtrgbf(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, gtcsr::Csgtrgbf, Gtcsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,gtcsr::Csgtrgbf, Gtcsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGB Pin Rising Input Source Counter Clear Enable"]
    #[inline(always)]
    pub fn csgtrgbr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, gtcsr::Csgtrgbr, Gtcsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,gtcsr::Csgtrgbr, Gtcsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGA Pin Falling Input Source Counter Clear Enable"]
    #[inline(always)]
    pub fn csgtrgaf(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, gtcsr::Csgtrgaf, Gtcsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,gtcsr::Csgtrgaf, Gtcsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGA Pin Rising Input Source Counter Clear Enable"]
    #[inline(always)]
    pub fn csgtrgar(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, gtcsr::Csgtrgar, Gtcsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,gtcsr::Csgtrgar, Gtcsr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gtcsr {
    #[inline(always)]
    fn default() -> Gtcsr {
        <crate::RegValueT<Gtcsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtcsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cclr_SPEC;
    pub type Cclr = crate::EnumBitfieldStruct<u8, Cclr_SPEC>;
    impl Cclr {
        #[doc = "Disable counter clear by the GTCLR register"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter clear by the GTCLR register"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cselch_SPEC;
    pub type Cselch = crate::EnumBitfieldStruct<u8, Cselch_SPEC>;
    impl Cselch {
        #[doc = "Disable counter clear on ELC_GPTH input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter clear on ELC_GPTH input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cselcg_SPEC;
    pub type Cselcg = crate::EnumBitfieldStruct<u8, Cselcg_SPEC>;
    impl Cselcg {
        #[doc = "Disable counter clear on ELC_GPTG input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter clear on ELC_GPTG input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cselcf_SPEC;
    pub type Cselcf = crate::EnumBitfieldStruct<u8, Cselcf_SPEC>;
    impl Cselcf {
        #[doc = "Disable counter clear on ELC_GPTF input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter clear on ELC_GPTF input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cselce_SPEC;
    pub type Cselce = crate::EnumBitfieldStruct<u8, Cselce_SPEC>;
    impl Cselce {
        #[doc = "Disable counter clear on ELC_GPTE input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter clear on ELC_GPTE input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cselcd_SPEC;
    pub type Cselcd = crate::EnumBitfieldStruct<u8, Cselcd_SPEC>;
    impl Cselcd {
        #[doc = "Disable counter clear on ELC_GPTD input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter clear on ELC_GPTD input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cselcc_SPEC;
    pub type Cselcc = crate::EnumBitfieldStruct<u8, Cselcc_SPEC>;
    impl Cselcc {
        #[doc = "Disable counter clear on ELC_GPTC input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter clear on ELC_GPTC input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cselcb_SPEC;
    pub type Cselcb = crate::EnumBitfieldStruct<u8, Cselcb_SPEC>;
    impl Cselcb {
        #[doc = "Disable counter clear on ELC_GPTB input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter clear on ELC_GPTB input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cselca_SPEC;
    pub type Cselca = crate::EnumBitfieldStruct<u8, Cselca_SPEC>;
    impl Cselca {
        #[doc = "Disable counter clear on ELC_GPTA input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter clear on ELC_GPTA input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cscbfah_SPEC;
    pub type Cscbfah = crate::EnumBitfieldStruct<u8, Cscbfah_SPEC>;
    impl Cscbfah {
        #[doc = "Disable counter clear on the falling edge of GTIOCB input when GTIOCA input is 1"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter clear on the falling edge of GTIOCB input when GTIOCA input is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cscbfal_SPEC;
    pub type Cscbfal = crate::EnumBitfieldStruct<u8, Cscbfal_SPEC>;
    impl Cscbfal {
        #[doc = "Disable counter clear on the falling edge of GTIOCB input when GTIOCA input is 0"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter clear on the falling edge of GTIOCB input when GTIOCA input is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cscbrah_SPEC;
    pub type Cscbrah = crate::EnumBitfieldStruct<u8, Cscbrah_SPEC>;
    impl Cscbrah {
        #[doc = "Disable counter clear on the rising edge of GTIOCB input when GTIOCA input is 1"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter clear on the rising edge of GTIOCB input when GTIOCA input is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cscbral_SPEC;
    pub type Cscbral = crate::EnumBitfieldStruct<u8, Cscbral_SPEC>;
    impl Cscbral {
        #[doc = "Disable counter clear on the rising edge of GTIOCB input when GTIOCA input is 0"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter clear on the rising edge of GTIOCB input when GTIOCA input is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cscafbh_SPEC;
    pub type Cscafbh = crate::EnumBitfieldStruct<u8, Cscafbh_SPEC>;
    impl Cscafbh {
        #[doc = "Disable counter clear on the falling edge of GTIOCA input when GTIOCB input is 1"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter clear on the falling edge of GTIOCA input when GTIOCB input is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cscafbl_SPEC;
    pub type Cscafbl = crate::EnumBitfieldStruct<u8, Cscafbl_SPEC>;
    impl Cscafbl {
        #[doc = "Disable counter clear on the falling edge of GTIOCA input when GTIOCB input is 0"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter clear on the falling edge of GTIOCA input when GTIOCB input is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cscarbh_SPEC;
    pub type Cscarbh = crate::EnumBitfieldStruct<u8, Cscarbh_SPEC>;
    impl Cscarbh {
        #[doc = "Disable counter clear on the rising edge of GTIOCA input when GTIOCB input is 1"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter clear on the rising edge of GTIOCA input when GTIOCB input is 1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cscarbl_SPEC;
    pub type Cscarbl = crate::EnumBitfieldStruct<u8, Cscarbl_SPEC>;
    impl Cscarbl {
        #[doc = "Disable counter clear on the rising edge of GTIOCA input when GTIOCB input is 0"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter clear on the rising edge of GTIOCA input when GTIOCB input is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csgtrgdf_SPEC;
    pub type Csgtrgdf = crate::EnumBitfieldStruct<u8, Csgtrgdf_SPEC>;
    impl Csgtrgdf {
        #[doc = "Disable counter clear on the falling edge of GTETRGD input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter clear on the falling edge of GTETRGD input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csgtrgdr_SPEC;
    pub type Csgtrgdr = crate::EnumBitfieldStruct<u8, Csgtrgdr_SPEC>;
    impl Csgtrgdr {
        #[doc = "Disable counter clear on the rising edge of GTETRGD input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter clear on the rising edge of GTETRGD input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csgtrgcf_SPEC;
    pub type Csgtrgcf = crate::EnumBitfieldStruct<u8, Csgtrgcf_SPEC>;
    impl Csgtrgcf {
        #[doc = "Disable counter clear on the falling edge of GTETRGC input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter clear on the falling edge of GTETRGC input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csgtrgcr_SPEC;
    pub type Csgtrgcr = crate::EnumBitfieldStruct<u8, Csgtrgcr_SPEC>;
    impl Csgtrgcr {
        #[doc = "Disable counter clear on the rising edge of GTETRGC input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter clear on the rising edge of GTETRGC input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csgtrgbf_SPEC;
    pub type Csgtrgbf = crate::EnumBitfieldStruct<u8, Csgtrgbf_SPEC>;
    impl Csgtrgbf {
        #[doc = "Disable counter clear on the falling edge of GTETRGB input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter clear on the falling edge of GTETRGB input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csgtrgbr_SPEC;
    pub type Csgtrgbr = crate::EnumBitfieldStruct<u8, Csgtrgbr_SPEC>;
    impl Csgtrgbr {
        #[doc = "Disable counter clear on the rising edge of GTETRGB input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter clear on the rising edge of GTETRGB input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csgtrgaf_SPEC;
    pub type Csgtrgaf = crate::EnumBitfieldStruct<u8, Csgtrgaf_SPEC>;
    impl Csgtrgaf {
        #[doc = "Disable counter clear on the falling edge of GTETRGA input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter clear on the falling edge of GTETRGA input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Csgtrgar_SPEC;
    pub type Csgtrgar = crate::EnumBitfieldStruct<u8, Csgtrgar_SPEC>;
    impl Csgtrgar {
        #[doc = "Disable counter clear on the rising edge of GTETRGA input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter clear on the rising edge of GTETRGA input"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtupsr_SPEC;
impl crate::sealed::RegSpec for Gtupsr_SPEC {
    type DataType = u32;
}
#[doc = "General PWM Timer Up Count Source Select Register"]
pub type Gtupsr = crate::RegValueT<Gtupsr_SPEC>;

impl Gtupsr {
    #[doc = "ELC_GPTH Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uselch(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, gtupsr::Uselch, Gtupsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,gtupsr::Uselch, Gtupsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELC_GPTG Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uselcg(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, gtupsr::Uselcg, Gtupsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,gtupsr::Uselcg, Gtupsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELC_GPTF Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uselcf(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, gtupsr::Uselcf, Gtupsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,gtupsr::Uselcf, Gtupsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELC_GPTE Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uselce(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, gtupsr::Uselce, Gtupsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,gtupsr::Uselce, Gtupsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELC_GPTD Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uselcd(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, gtupsr::Uselcd, Gtupsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,gtupsr::Uselcd, Gtupsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELC_GPTC Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uselcc(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, gtupsr::Uselcc, Gtupsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,gtupsr::Uselcc, Gtupsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELC_GPTB Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uselcb(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, gtupsr::Uselcb, Gtupsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,gtupsr::Uselcb, Gtupsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELC_GPTA Event Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uselca(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, gtupsr::Uselca, Gtupsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,gtupsr::Uselca, Gtupsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uscbfah(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, gtupsr::Uscbfah, Gtupsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,gtupsr::Uscbfah, Gtupsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uscbfal(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, gtupsr::Uscbfal, Gtupsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,gtupsr::Uscbfal, Gtupsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uscbrah(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, gtupsr::Uscbrah, Gtupsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,gtupsr::Uscbrah, Gtupsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uscbral(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, gtupsr::Uscbral, Gtupsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,gtupsr::Uscbral, Gtupsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uscafbh(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, gtupsr::Uscafbh, Gtupsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,gtupsr::Uscafbh, Gtupsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uscafbl(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, gtupsr::Uscafbl, Gtupsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,gtupsr::Uscafbl, Gtupsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uscarbh(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, gtupsr::Uscarbh, Gtupsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,gtupsr::Uscarbh, Gtupsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn uscarbl(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, gtupsr::Uscarbl, Gtupsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,gtupsr::Uscarbl, Gtupsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGD Pin Falling Input Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn usgtrgdf(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, gtupsr::Usgtrgdf, Gtupsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,gtupsr::Usgtrgdf, Gtupsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGD Pin Rising Input Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn usgtrgdr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, gtupsr::Usgtrgdr, Gtupsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,gtupsr::Usgtrgdr, Gtupsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGC Pin Falling Input Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn usgtrgcf(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, gtupsr::Usgtrgcf, Gtupsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,gtupsr::Usgtrgcf, Gtupsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGC Pin Rising Input Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn usgtrgcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, gtupsr::Usgtrgcr, Gtupsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,gtupsr::Usgtrgcr, Gtupsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGB Pin Falling Input Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn usgtrgbf(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, gtupsr::Usgtrgbf, Gtupsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,gtupsr::Usgtrgbf, Gtupsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGB Pin Rising Input Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn usgtrgbr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, gtupsr::Usgtrgbr, Gtupsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,gtupsr::Usgtrgbr, Gtupsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGA Pin Falling Input Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn usgtrgaf(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, gtupsr::Usgtrgaf, Gtupsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,gtupsr::Usgtrgaf, Gtupsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGA Pin Rising Input Source Counter Count Up Enable"]
    #[inline(always)]
    pub fn usgtrgar(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, gtupsr::Usgtrgar, Gtupsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,gtupsr::Usgtrgar, Gtupsr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gtupsr {
    #[inline(always)]
    fn default() -> Gtupsr {
        <crate::RegValueT<Gtupsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtupsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uselch_SPEC;
    pub type Uselch = crate::EnumBitfieldStruct<u8, Uselch_SPEC>;
    impl Uselch {
        #[doc = "Disable counter count up on ELC_GPTH input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count up on ELC_GPTH input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uselcg_SPEC;
    pub type Uselcg = crate::EnumBitfieldStruct<u8, Uselcg_SPEC>;
    impl Uselcg {
        #[doc = "Disable counter count up on ELC_GPTG input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count up on ELC_GPTG input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uselcf_SPEC;
    pub type Uselcf = crate::EnumBitfieldStruct<u8, Uselcf_SPEC>;
    impl Uselcf {
        #[doc = "Disable counter count up on ELC_GPTF input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count up on ELC_GPTF input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uselce_SPEC;
    pub type Uselce = crate::EnumBitfieldStruct<u8, Uselce_SPEC>;
    impl Uselce {
        #[doc = "Disable counter count up on ELC_GPTE input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count up on ELC_GPTE input.put"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uselcd_SPEC;
    pub type Uselcd = crate::EnumBitfieldStruct<u8, Uselcd_SPEC>;
    impl Uselcd {
        #[doc = "Disable counter count up on ELC_GPTD input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count up on ELC_GPTD input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uselcc_SPEC;
    pub type Uselcc = crate::EnumBitfieldStruct<u8, Uselcc_SPEC>;
    impl Uselcc {
        #[doc = "Disable counter count up on ELC_GPTC input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count up on ELC_GPTC input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uselcb_SPEC;
    pub type Uselcb = crate::EnumBitfieldStruct<u8, Uselcb_SPEC>;
    impl Uselcb {
        #[doc = "Disable counter count up on ELC_GPTB input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count up on ELC_GPTB input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uselca_SPEC;
    pub type Uselca = crate::EnumBitfieldStruct<u8, Uselca_SPEC>;
    impl Uselca {
        #[doc = "Disable counter count up on ELC_GPTA input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count up on ELC_GPTA input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uscbfah_SPEC;
    pub type Uscbfah = crate::EnumBitfieldStruct<u8, Uscbfah_SPEC>;
    impl Uscbfah {
        #[doc = "Disable counter count up on the falling edge of GTIOCB input when GTIOCA input is 1"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count up on the falling edge of GTIOCB input when GTIOCA input is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uscbfal_SPEC;
    pub type Uscbfal = crate::EnumBitfieldStruct<u8, Uscbfal_SPEC>;
    impl Uscbfal {
        #[doc = "Disable counter count up on the falling edge of GTIOCB input when GTIOCA input is 0"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count up on the falling edge of GTIOCB input when GTIOCA input is 0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uscbrah_SPEC;
    pub type Uscbrah = crate::EnumBitfieldStruct<u8, Uscbrah_SPEC>;
    impl Uscbrah {
        #[doc = "Disable counter count up on the rising edge of GTIOCB input when GTIOCA input is 1"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count up on the rising edge of GTIOCB input when GTIOCA input is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uscbral_SPEC;
    pub type Uscbral = crate::EnumBitfieldStruct<u8, Uscbral_SPEC>;
    impl Uscbral {
        #[doc = "Disable counter count up on the rising edge of GTIOCB input when GTIOCA input is 0"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count up on the rising edge of GTIOCB input when GTIOCA input is 0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uscafbh_SPEC;
    pub type Uscafbh = crate::EnumBitfieldStruct<u8, Uscafbh_SPEC>;
    impl Uscafbh {
        #[doc = "Disable counter count up on the falling edge of GTIOCA input when GTIOCB input is 1"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count up on the falling edge of GTIOCA input when GTIOCB input is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uscafbl_SPEC;
    pub type Uscafbl = crate::EnumBitfieldStruct<u8, Uscafbl_SPEC>;
    impl Uscafbl {
        #[doc = "Disable counter count up on the falling edge of GTIOCA input when GTIOCB input is 0"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count up on the falling edge of GTIOCA input when GTIOCB input is 0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uscarbh_SPEC;
    pub type Uscarbh = crate::EnumBitfieldStruct<u8, Uscarbh_SPEC>;
    impl Uscarbh {
        #[doc = "Disable counter count up on the rising edge of GTIOCA input when GTIOCB input is 1"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count up on the rising edge of GTIOCA input when GTIOCB input is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Uscarbl_SPEC;
    pub type Uscarbl = crate::EnumBitfieldStruct<u8, Uscarbl_SPEC>;
    impl Uscarbl {
        #[doc = "Disable counter count up on the rising edge of GTIOCA input when GTIOCB input is 0"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count up on the rising edge of GTIOCA input when GTIOCB input is 0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usgtrgdf_SPEC;
    pub type Usgtrgdf = crate::EnumBitfieldStruct<u8, Usgtrgdf_SPEC>;
    impl Usgtrgdf {
        #[doc = "Disable counter count up on the falling edge of GTETRGD input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count up on the falling edge of GTETRGD input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usgtrgdr_SPEC;
    pub type Usgtrgdr = crate::EnumBitfieldStruct<u8, Usgtrgdr_SPEC>;
    impl Usgtrgdr {
        #[doc = "Disable counter count up on the rising edge of GTETRGD input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count up on the rising edge of GTETRGD input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usgtrgcf_SPEC;
    pub type Usgtrgcf = crate::EnumBitfieldStruct<u8, Usgtrgcf_SPEC>;
    impl Usgtrgcf {
        #[doc = "Disable counter count up on the falling edge of GTETRGC input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count up on the falling edge of GTETRGC input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usgtrgcr_SPEC;
    pub type Usgtrgcr = crate::EnumBitfieldStruct<u8, Usgtrgcr_SPEC>;
    impl Usgtrgcr {
        #[doc = "Disable counter count up on the rising edge of GTETRGC input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count up on the rising edge of GTETRGC input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usgtrgbf_SPEC;
    pub type Usgtrgbf = crate::EnumBitfieldStruct<u8, Usgtrgbf_SPEC>;
    impl Usgtrgbf {
        #[doc = "Disable counter count up on the falling edge of GTETRGB input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count up on the falling edge of GTETRGB input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usgtrgbr_SPEC;
    pub type Usgtrgbr = crate::EnumBitfieldStruct<u8, Usgtrgbr_SPEC>;
    impl Usgtrgbr {
        #[doc = "Disable counter count up on the rising edge of GTETRGB input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count up on the rising edge of GTETRGB input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usgtrgaf_SPEC;
    pub type Usgtrgaf = crate::EnumBitfieldStruct<u8, Usgtrgaf_SPEC>;
    impl Usgtrgaf {
        #[doc = "Disable counter count up on the falling edge of GTETRGA input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count up on the falling edge of GTETRGA input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usgtrgar_SPEC;
    pub type Usgtrgar = crate::EnumBitfieldStruct<u8, Usgtrgar_SPEC>;
    impl Usgtrgar {
        #[doc = "Disable counter count up on the rising edge of GTETRGA input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count up on the rising edge of GTETRGA input"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtdnsr_SPEC;
impl crate::sealed::RegSpec for Gtdnsr_SPEC {
    type DataType = u32;
}
#[doc = "General PWM Timer Down Count Source Select Register"]
pub type Gtdnsr = crate::RegValueT<Gtdnsr_SPEC>;

impl Gtdnsr {
    #[doc = "ELC_GPTH Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dselch(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, gtdnsr::Dselch, Gtdnsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,gtdnsr::Dselch, Gtdnsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELC_GPTG Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dselcg(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, gtdnsr::Dselcg, Gtdnsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,gtdnsr::Dselcg, Gtdnsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELC_GPTF Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dselcf(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, gtdnsr::Dselcf, Gtdnsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,gtdnsr::Dselcf, Gtdnsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELC_GPTE Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dselce(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, gtdnsr::Dselce, Gtdnsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,gtdnsr::Dselce, Gtdnsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELC_GPTD Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dselcd(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, gtdnsr::Dselcd, Gtdnsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,gtdnsr::Dselcd, Gtdnsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELC_GPTC Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dselcc(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, gtdnsr::Dselcc, Gtdnsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,gtdnsr::Dselcc, Gtdnsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELC_GPTB Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dselcb(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, gtdnsr::Dselcb, Gtdnsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,gtdnsr::Dselcb, Gtdnsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ELC_GPTA Event Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dselca(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, gtdnsr::Dselca, Gtdnsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,gtdnsr::Dselca, Gtdnsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscbfah(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, gtdnsr::Dscbfah, Gtdnsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,gtdnsr::Dscbfah, Gtdnsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscbfal(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, gtdnsr::Dscbfal, Gtdnsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,gtdnsr::Dscbfal, Gtdnsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscbrah(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, gtdnsr::Dscbrah, Gtdnsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,gtdnsr::Dscbrah, Gtdnsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscbral(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, gtdnsr::Dscbral, Gtdnsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,gtdnsr::Dscbral, Gtdnsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscafbh(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, gtdnsr::Dscafbh, Gtdnsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,gtdnsr::Dscafbh, Gtdnsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscafbl(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, gtdnsr::Dscafbl, Gtdnsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,gtdnsr::Dscafbl, Gtdnsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscarbh(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, gtdnsr::Dscarbh, Gtdnsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,gtdnsr::Dscarbh, Gtdnsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dscarbl(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, gtdnsr::Dscarbl, Gtdnsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,gtdnsr::Dscarbl, Gtdnsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGD Pin Falling Input Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dsgtrgdf(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, gtdnsr::Dsgtrgdf, Gtdnsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,gtdnsr::Dsgtrgdf, Gtdnsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGD Pin Rising Input Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dsgtrgdr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, gtdnsr::Dsgtrgdr, Gtdnsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,gtdnsr::Dsgtrgdr, Gtdnsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGC Pin Falling Input Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dsgtrgcf(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, gtdnsr::Dsgtrgcf, Gtdnsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,gtdnsr::Dsgtrgcf, Gtdnsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGC Pin Rising Input Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dsgtrgcr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, gtdnsr::Dsgtrgcr, Gtdnsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,gtdnsr::Dsgtrgcr, Gtdnsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGB Pin Falling Input Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dsgtrgbf(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, gtdnsr::Dsgtrgbf, Gtdnsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,gtdnsr::Dsgtrgbf, Gtdnsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGB Pin Rising Input Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dsgtrgbr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, gtdnsr::Dsgtrgbr, Gtdnsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,gtdnsr::Dsgtrgbr, Gtdnsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGA Pin Falling Input Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dsgtrgaf(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, gtdnsr::Dsgtrgaf, Gtdnsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,gtdnsr::Dsgtrgaf, Gtdnsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTETRGA Pin Rising Input Source Counter Count Down Enable"]
    #[inline(always)]
    pub fn dsgtrgar(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, gtdnsr::Dsgtrgar, Gtdnsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,gtdnsr::Dsgtrgar, Gtdnsr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gtdnsr {
    #[inline(always)]
    fn default() -> Gtdnsr {
        <crate::RegValueT<Gtdnsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtdnsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dselch_SPEC;
    pub type Dselch = crate::EnumBitfieldStruct<u8, Dselch_SPEC>;
    impl Dselch {
        #[doc = "Disable counter count down on ELC_GPTH input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count down on ELC_GPTH input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dselcg_SPEC;
    pub type Dselcg = crate::EnumBitfieldStruct<u8, Dselcg_SPEC>;
    impl Dselcg {
        #[doc = "Disable counter count down on ELC_GPTG input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count down on ELC_GPTG input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dselcf_SPEC;
    pub type Dselcf = crate::EnumBitfieldStruct<u8, Dselcf_SPEC>;
    impl Dselcf {
        #[doc = "Disable counter count down on ELC_GPTF input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count down on ELC_GPTF input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dselce_SPEC;
    pub type Dselce = crate::EnumBitfieldStruct<u8, Dselce_SPEC>;
    impl Dselce {
        #[doc = "Disable counter count down on ELC_GPTE input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count down on ELC_GPTE input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dselcd_SPEC;
    pub type Dselcd = crate::EnumBitfieldStruct<u8, Dselcd_SPEC>;
    impl Dselcd {
        #[doc = "Disable counter count down on ELC_GPTD input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count down on ELC_GPTD input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dselcc_SPEC;
    pub type Dselcc = crate::EnumBitfieldStruct<u8, Dselcc_SPEC>;
    impl Dselcc {
        #[doc = "Disable counter count down on ELC_GPTC input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count down on ELC_GPTC input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dselcb_SPEC;
    pub type Dselcb = crate::EnumBitfieldStruct<u8, Dselcb_SPEC>;
    impl Dselcb {
        #[doc = "Disable counter count down on ELC_GPTB input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count down on ELC_GPTB input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dselca_SPEC;
    pub type Dselca = crate::EnumBitfieldStruct<u8, Dselca_SPEC>;
    impl Dselca {
        #[doc = "Disable counter count down on ELC_GPTA input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count down on ELC_GPTA input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscbfah_SPEC;
    pub type Dscbfah = crate::EnumBitfieldStruct<u8, Dscbfah_SPEC>;
    impl Dscbfah {
        #[doc = "Disable counter count down on the falling edge of GTIOCB input when GTIOCA input is 1"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count down on the falling edge of GTIOCB input when GTIOCA input is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscbfal_SPEC;
    pub type Dscbfal = crate::EnumBitfieldStruct<u8, Dscbfal_SPEC>;
    impl Dscbfal {
        #[doc = "Disable counter count down on the falling edge of GTIOCB input when GTIOCA input is 0"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count down on the falling edge of GTIOCB input when GTIOCA input is 0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscbrah_SPEC;
    pub type Dscbrah = crate::EnumBitfieldStruct<u8, Dscbrah_SPEC>;
    impl Dscbrah {
        #[doc = "Disable counter count down on the rising edge of GTIOCB input when GTIOCA input is 1"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count down on the rising edge of GTIOCB input when GTIOCA input is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscbral_SPEC;
    pub type Dscbral = crate::EnumBitfieldStruct<u8, Dscbral_SPEC>;
    impl Dscbral {
        #[doc = "Disable counter count down on the rising edge of GTIOCB input when GTIOCA input is 0"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count down on the rising edge of GTIOCB input when GTIOCA input is 0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscafbh_SPEC;
    pub type Dscafbh = crate::EnumBitfieldStruct<u8, Dscafbh_SPEC>;
    impl Dscafbh {
        #[doc = "Disable counter count down on the falling edge of GTIOCA input when GTIOCB input is 1"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count down on the falling edge of GTIOCA input when GTIOCB input is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscafbl_SPEC;
    pub type Dscafbl = crate::EnumBitfieldStruct<u8, Dscafbl_SPEC>;
    impl Dscafbl {
        #[doc = "Disable counter count down on the falling edge of GTIOCA input when GTIOCB input is 0"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count down on the falling edge of GTIOCA input when GTIOCB input is 0"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscarbh_SPEC;
    pub type Dscarbh = crate::EnumBitfieldStruct<u8, Dscarbh_SPEC>;
    impl Dscarbh {
        #[doc = "Disable counter count down on the rising edge of GTIOCA input when GTIOCB input is 1"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count down on the rising edge of GTIOCA input when GTIOCB input is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dscarbl_SPEC;
    pub type Dscarbl = crate::EnumBitfieldStruct<u8, Dscarbl_SPEC>;
    impl Dscarbl {
        #[doc = "Disable counter count down on the rising edge of GTIOCA input when GTIOCB input is 0"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count down on the rising edge of GTIOCA input when GTIOCB input is 0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dsgtrgdf_SPEC;
    pub type Dsgtrgdf = crate::EnumBitfieldStruct<u8, Dsgtrgdf_SPEC>;
    impl Dsgtrgdf {
        #[doc = "Disable counter count down on the falling edge of GTETRGD input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count down on the falling edge of GTETRGD input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dsgtrgdr_SPEC;
    pub type Dsgtrgdr = crate::EnumBitfieldStruct<u8, Dsgtrgdr_SPEC>;
    impl Dsgtrgdr {
        #[doc = "Disable counter count down on the rising edge of GTETRGD input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count down on the rising edge of GTETRGD input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dsgtrgcf_SPEC;
    pub type Dsgtrgcf = crate::EnumBitfieldStruct<u8, Dsgtrgcf_SPEC>;
    impl Dsgtrgcf {
        #[doc = "Disable counter count down on the falling edge of GTETRGC input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count down on the falling edge of GTETRGC input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dsgtrgcr_SPEC;
    pub type Dsgtrgcr = crate::EnumBitfieldStruct<u8, Dsgtrgcr_SPEC>;
    impl Dsgtrgcr {
        #[doc = "Disable counter count down on the rising edge of GTETRGC input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count down on the rising edge of GTETRGC input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dsgtrgbf_SPEC;
    pub type Dsgtrgbf = crate::EnumBitfieldStruct<u8, Dsgtrgbf_SPEC>;
    impl Dsgtrgbf {
        #[doc = "Disable counter count down on the falling edge of GTETRGB input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count down on the falling edge of GTETRGB input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dsgtrgbr_SPEC;
    pub type Dsgtrgbr = crate::EnumBitfieldStruct<u8, Dsgtrgbr_SPEC>;
    impl Dsgtrgbr {
        #[doc = "Disable counter count down on the rising edge of GTETRGB input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count down on the rising edge of GTETRGB input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dsgtrgaf_SPEC;
    pub type Dsgtrgaf = crate::EnumBitfieldStruct<u8, Dsgtrgaf_SPEC>;
    impl Dsgtrgaf {
        #[doc = "Disable counter count down on the falling edge of GTETRGA input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count down on the falling edge of GTETRGA input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dsgtrgar_SPEC;
    pub type Dsgtrgar = crate::EnumBitfieldStruct<u8, Dsgtrgar_SPEC>;
    impl Dsgtrgar {
        #[doc = "Disable counter count down on the rising edge of GTETRGA input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable counter count down on the rising edge of GTETRGA input"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gticasr_SPEC;
impl crate::sealed::RegSpec for Gticasr_SPEC {
    type DataType = u32;
}
#[doc = "General PWM Timer Input Capture Source Select Register A"]
pub type Gticasr = crate::RegValueT<Gticasr_SPEC>;

impl Gticasr {
    #[doc = "ELC_GPTH Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn aselch(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, gticasr::Aselch, Gticasr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            gticasr::Aselch,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ELC_GPTG Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn aselcg(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, gticasr::Aselcg, Gticasr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            gticasr::Aselcg,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ELC_GPTF Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn aselcf(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, gticasr::Aselcf, Gticasr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            gticasr::Aselcf,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ELC_GPTE Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn aselce(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, gticasr::Aselce, Gticasr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            gticasr::Aselce,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ELC_GPTD Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn aselcd(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, gticasr::Aselcd, Gticasr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            gticasr::Aselcd,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ELC_GPTC Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn aselcc(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, gticasr::Aselcc, Gticasr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            gticasr::Aselcc,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ELC_GPTB Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn aselcb(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, gticasr::Aselcb, Gticasr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            gticasr::Aselcb,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ELC_GPTA Event Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn aselca(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, gticasr::Aselca, Gticasr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            gticasr::Aselca,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascbfah(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        gticasr::Ascbfah,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            gticasr::Ascbfah,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascbfal(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        gticasr::Ascbfal,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            gticasr::Ascbfal,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascbrah(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        gticasr::Ascbrah,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            gticasr::Ascbrah,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascbral(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        gticasr::Ascbral,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            gticasr::Ascbral,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascafbh(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        gticasr::Ascafbh,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            gticasr::Ascafbh,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascafbl(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        gticasr::Ascafbl,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            gticasr::Ascafbl,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascarbh(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, gticasr::Ascarbh, Gticasr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            gticasr::Ascarbh,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn ascarbl(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, gticasr::Ascarbl, Gticasr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gticasr::Ascarbl,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "GTETRGD Pin Falling Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn asgtrgdf(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        gticasr::Asgtrgdf,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            gticasr::Asgtrgdf,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "GTETRGD Pin Rising Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn asgtrgdr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        gticasr::Asgtrgdr,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            gticasr::Asgtrgdr,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "GTETRGC Pin Falling Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn asgtrgcf(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        gticasr::Asgtrgcf,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            gticasr::Asgtrgcf,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "GTETRGC Pin Rising Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn asgtrgcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gticasr::Asgtrgcr,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gticasr::Asgtrgcr,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "GTETRGB Pin Falling Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn asgtrgbf(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gticasr::Asgtrgbf,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gticasr::Asgtrgbf,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "GTETRGB Pin Rising Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn asgtrgbr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gticasr::Asgtrgbr,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gticasr::Asgtrgbr,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "GTETRGA Pin Falling Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn asgtrgaf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gticasr::Asgtrgaf,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gticasr::Asgtrgaf,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "GTETRGA Pin Rising Input Source GTCCRA Input Capture Enable"]
    #[inline(always)]
    pub fn asgtrgar(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gticasr::Asgtrgar,
        Gticasr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gticasr::Asgtrgar,
            Gticasr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gticasr {
    #[inline(always)]
    fn default() -> Gticasr {
        <crate::RegValueT<Gticasr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gticasr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aselch_SPEC;
    pub type Aselch = crate::EnumBitfieldStruct<u8, Aselch_SPEC>;
    impl Aselch {
        #[doc = "Disable GTCCRA input capture on ELC_GPTH input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRA input capture on ELC_GPTH input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aselcg_SPEC;
    pub type Aselcg = crate::EnumBitfieldStruct<u8, Aselcg_SPEC>;
    impl Aselcg {
        #[doc = "Disable GTCCRA input capture on ELC_GPTG input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRA input capture on ELC_GPTG input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aselcf_SPEC;
    pub type Aselcf = crate::EnumBitfieldStruct<u8, Aselcf_SPEC>;
    impl Aselcf {
        #[doc = "Disable GTCCRA input capture on ELC_GPTF input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRA input capture on ELC_GPTF input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aselce_SPEC;
    pub type Aselce = crate::EnumBitfieldStruct<u8, Aselce_SPEC>;
    impl Aselce {
        #[doc = "Disable GTCCRA input capture on ELC_GPTE input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRA input capture on ELC_GPTE input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aselcd_SPEC;
    pub type Aselcd = crate::EnumBitfieldStruct<u8, Aselcd_SPEC>;
    impl Aselcd {
        #[doc = "Disable GTCCRA input capture on ELC_GPTD input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRA input capture on ELC_GPTD input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aselcc_SPEC;
    pub type Aselcc = crate::EnumBitfieldStruct<u8, Aselcc_SPEC>;
    impl Aselcc {
        #[doc = "Disable GTCCRA input capture on ELC_GPTC input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRA input capture on ELC_GPTC input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aselcb_SPEC;
    pub type Aselcb = crate::EnumBitfieldStruct<u8, Aselcb_SPEC>;
    impl Aselcb {
        #[doc = "Disable GTCCRA input capture on ELC_GPTB input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRA input capture on ELC_GPTB input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aselca_SPEC;
    pub type Aselca = crate::EnumBitfieldStruct<u8, Aselca_SPEC>;
    impl Aselca {
        #[doc = "Disable GTCCRA input capture on ELC_GPTA input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRA input capture on ELC_GPTA input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ascbfah_SPEC;
    pub type Ascbfah = crate::EnumBitfieldStruct<u8, Ascbfah_SPEC>;
    impl Ascbfah {
        #[doc = "Disable GTCCRA input capture on the falling edge of GTIOCB input when GTIOCA input is 1"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRA input capture on the falling edge of GTIOCB input when GTIOCA input is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ascbfal_SPEC;
    pub type Ascbfal = crate::EnumBitfieldStruct<u8, Ascbfal_SPEC>;
    impl Ascbfal {
        #[doc = "Disable GTCCRA input capture on the falling edge of GTIOCB input when GTIOCA input is 0"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRA input capture on the falling edge of GTIOCB input when GTIOCA input is 0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ascbrah_SPEC;
    pub type Ascbrah = crate::EnumBitfieldStruct<u8, Ascbrah_SPEC>;
    impl Ascbrah {
        #[doc = "Disable GTCCRA input capture on the rising edge of GTIOCB input when GTIOCA input is 1"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRA input capture on the rising edge of GTIOCB input when GTIOCA input is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ascbral_SPEC;
    pub type Ascbral = crate::EnumBitfieldStruct<u8, Ascbral_SPEC>;
    impl Ascbral {
        #[doc = "Disable GTCCRA input capture on the rising edge of GTIOCB input when GTIOCA input is 0"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRA input capture on the rising edge of GTIOCB input when GTIOCA input is 0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ascafbh_SPEC;
    pub type Ascafbh = crate::EnumBitfieldStruct<u8, Ascafbh_SPEC>;
    impl Ascafbh {
        #[doc = "Disable GTCCRA input capture on the falling edge of GTIOCA input when GTIOCB input is 1"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRA input capture on the falling edge of GTIOCA input when GTIOCB input is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ascafbl_SPEC;
    pub type Ascafbl = crate::EnumBitfieldStruct<u8, Ascafbl_SPEC>;
    impl Ascafbl {
        #[doc = "Disable GTCCRA input capture on the falling edge of GTIOCA input when GTIOCB input is 0"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRA input capture on the falling edge of GTIOCA input when GTIOCB input is 0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ascarbh_SPEC;
    pub type Ascarbh = crate::EnumBitfieldStruct<u8, Ascarbh_SPEC>;
    impl Ascarbh {
        #[doc = "Disable GTCCRA input capture on the rising edge of GTIOCA input when GTIOCB input is 1"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRA input capture on the rising edge of GTIOCA input when GTIOCB input is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ascarbl_SPEC;
    pub type Ascarbl = crate::EnumBitfieldStruct<u8, Ascarbl_SPEC>;
    impl Ascarbl {
        #[doc = "Disable GTCCRA input capture on the rising edge of GTIOCA input when GTIOCB input is 0"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRA input capture on the rising edge of GTIOCA input when GTIOCB input is 0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asgtrgdf_SPEC;
    pub type Asgtrgdf = crate::EnumBitfieldStruct<u8, Asgtrgdf_SPEC>;
    impl Asgtrgdf {
        #[doc = "Disable GTCCRA input capture on the falling edge of GTETRGD input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRA input capture on the falling edge of GTETRGD input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asgtrgdr_SPEC;
    pub type Asgtrgdr = crate::EnumBitfieldStruct<u8, Asgtrgdr_SPEC>;
    impl Asgtrgdr {
        #[doc = "Disable GTCCRA input capture on the rising edge of GTETRGD input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRA input capture on the rising edge of GTETRGD input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asgtrgcf_SPEC;
    pub type Asgtrgcf = crate::EnumBitfieldStruct<u8, Asgtrgcf_SPEC>;
    impl Asgtrgcf {
        #[doc = "Disable GTCCRA input capture on the falling edge of GTETRGC input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRA input capture on the falling edge of GTETRGC input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asgtrgcr_SPEC;
    pub type Asgtrgcr = crate::EnumBitfieldStruct<u8, Asgtrgcr_SPEC>;
    impl Asgtrgcr {
        #[doc = "Disable GTCCRA input capture on the rising edge of GTETRGC input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRA input capture on the rising edge of GTETRGC input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asgtrgbf_SPEC;
    pub type Asgtrgbf = crate::EnumBitfieldStruct<u8, Asgtrgbf_SPEC>;
    impl Asgtrgbf {
        #[doc = "Disable GTCCRA input capture on the falling edge of GTETRGB input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRA input capture on the falling edge of GTETRGB input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asgtrgbr_SPEC;
    pub type Asgtrgbr = crate::EnumBitfieldStruct<u8, Asgtrgbr_SPEC>;
    impl Asgtrgbr {
        #[doc = "Disable GTCCRA input capture on the rising edge of GTETRGB input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRA input capture on the rising edge of GTETRGB input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asgtrgaf_SPEC;
    pub type Asgtrgaf = crate::EnumBitfieldStruct<u8, Asgtrgaf_SPEC>;
    impl Asgtrgaf {
        #[doc = "Disable GTCCRA input capture on the falling edge of GTETRGA input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRA input capture on the falling edge of GTETRGA input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asgtrgar_SPEC;
    pub type Asgtrgar = crate::EnumBitfieldStruct<u8, Asgtrgar_SPEC>;
    impl Asgtrgar {
        #[doc = "Disable GTCCRA input capture on the rising edge of GTETRGA input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRA input capture on the rising edge of GTETRGA input."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gticbsr_SPEC;
impl crate::sealed::RegSpec for Gticbsr_SPEC {
    type DataType = u32;
}
#[doc = "General PWM Timer Input Capture Source Select Register B"]
pub type Gticbsr = crate::RegValueT<Gticbsr_SPEC>;

impl Gticbsr {
    #[doc = "ELC_GPTH Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselch(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, gticbsr::Bselch, Gticbsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            gticbsr::Bselch,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ELC_GPTG Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselcg(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, gticbsr::Bselcg, Gticbsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            gticbsr::Bselcg,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ELC_GPTF Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselcf(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, gticbsr::Bselcf, Gticbsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            gticbsr::Bselcf,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ELC_GPTE Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselce(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, gticbsr::Bselce, Gticbsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            gticbsr::Bselce,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ELC_GPTD Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselcd(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, gticbsr::Bselcd, Gticbsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            gticbsr::Bselcd,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ELC_GPTC Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselcc(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, gticbsr::Bselcc, Gticbsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            gticbsr::Bselcc,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ELC_GPTB Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselcb(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, gticbsr::Bselcb, Gticbsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            gticbsr::Bselcb,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ELC_GPTA Event Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bselca(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, gticbsr::Bselca, Gticbsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            gticbsr::Bselca,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value High Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscbfah(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        gticbsr::Bscbfah,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            gticbsr::Bscbfah,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "GTIOCB Pin Falling Input during GTIOCA Value Low Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscbfal(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        gticbsr::Bscbfal,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            gticbsr::Bscbfal,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value High Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscbrah(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        gticbsr::Bscbrah,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            gticbsr::Bscbrah,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "GTIOCB Pin Rising Input during GTIOCA Value Low Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscbral(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        gticbsr::Bscbral,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            gticbsr::Bscbral,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value High Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscafbh(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        gticbsr::Bscafbh,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            gticbsr::Bscafbh,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "GTIOCA Pin Falling Input during GTIOCB Value Low Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscafbl(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        gticbsr::Bscafbl,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            gticbsr::Bscafbl,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value High Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscarbh(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, gticbsr::Bscarbh, Gticbsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            gticbsr::Bscarbh,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "GTIOCA Pin Rising Input during GTIOCB Value Low Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bscarbl(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, gticbsr::Bscarbl, Gticbsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            gticbsr::Bscarbl,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "GTETRGD Pin Falling Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bsgtrgdf(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        gticbsr::Bsgtrgdf,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            gticbsr::Bsgtrgdf,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "GTETRGD Pin Rising Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bsgtrgdr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        gticbsr::Bsgtrgdr,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            gticbsr::Bsgtrgdr,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "GTETRGC Pin Falling Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bsgtrgcf(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        gticbsr::Bsgtrgcf,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            gticbsr::Bsgtrgcf,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "GTETRGC Pin Rising Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bsgtrgcr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        gticbsr::Bsgtrgcr,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            gticbsr::Bsgtrgcr,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "GTETRGB Pin Falling Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bsgtrgbf(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        gticbsr::Bsgtrgbf,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            gticbsr::Bsgtrgbf,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "GTETRGB Pin Rising Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bsgtrgbr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        gticbsr::Bsgtrgbr,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            gticbsr::Bsgtrgbr,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "GTETRGA Pin Falling Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bsgtrgaf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        gticbsr::Bsgtrgaf,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            gticbsr::Bsgtrgaf,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "GTETRGA Pin Rising Input Source GTCCRB Input Capture Enable"]
    #[inline(always)]
    pub fn bsgtrgar(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        gticbsr::Bsgtrgar,
        Gticbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            gticbsr::Bsgtrgar,
            Gticbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Gticbsr {
    #[inline(always)]
    fn default() -> Gticbsr {
        <crate::RegValueT<Gticbsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gticbsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bselch_SPEC;
    pub type Bselch = crate::EnumBitfieldStruct<u8, Bselch_SPEC>;
    impl Bselch {
        #[doc = "Disable GTCCRB input capture on ELC_GPTH input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRB input capture on ELC_GPTH input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bselcg_SPEC;
    pub type Bselcg = crate::EnumBitfieldStruct<u8, Bselcg_SPEC>;
    impl Bselcg {
        #[doc = "Disable GTCCRB input capture on ELC_GPTG input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRB input capture on ELC_GPTG input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bselcf_SPEC;
    pub type Bselcf = crate::EnumBitfieldStruct<u8, Bselcf_SPEC>;
    impl Bselcf {
        #[doc = "Disable GTCCRB input capture on ELC_GPTF input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRB input capture on ELC_GPTF input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bselce_SPEC;
    pub type Bselce = crate::EnumBitfieldStruct<u8, Bselce_SPEC>;
    impl Bselce {
        #[doc = "Disable GTCCRB input capture on ELC_GPTE input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRB input capture on ELC_GPTE input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bselcd_SPEC;
    pub type Bselcd = crate::EnumBitfieldStruct<u8, Bselcd_SPEC>;
    impl Bselcd {
        #[doc = "Disable GTCCRB input capture on ELC_GPTD input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRB input capture on ELC_GPTD input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bselcc_SPEC;
    pub type Bselcc = crate::EnumBitfieldStruct<u8, Bselcc_SPEC>;
    impl Bselcc {
        #[doc = "Disable GTCCRB input capture on ELC_GPTC input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRB input capture on ELC_GPTC input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bselcb_SPEC;
    pub type Bselcb = crate::EnumBitfieldStruct<u8, Bselcb_SPEC>;
    impl Bselcb {
        #[doc = "Disable GTCCRB input capture on ELC_GPTB input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRB input capture on ELC_GPTB input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bselca_SPEC;
    pub type Bselca = crate::EnumBitfieldStruct<u8, Bselca_SPEC>;
    impl Bselca {
        #[doc = "Disable GTCCRB input capture on ELC_GPTA input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRB input capture on ELC_GPTA input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bscbfah_SPEC;
    pub type Bscbfah = crate::EnumBitfieldStruct<u8, Bscbfah_SPEC>;
    impl Bscbfah {
        #[doc = "Disable GTCCRB input capture on the falling edge of GTIOCB input when GTIOCA input is 1"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRB input capture on the falling edge of GTIOCB input when GTIOCA input is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bscbfal_SPEC;
    pub type Bscbfal = crate::EnumBitfieldStruct<u8, Bscbfal_SPEC>;
    impl Bscbfal {
        #[doc = "Disable GTCCRB input capture on the falling edge of GTIOCB input when GTIOCA input is 0"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRB input capture on the falling edge of GTIOCB input when GTIOCA input is 0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bscbrah_SPEC;
    pub type Bscbrah = crate::EnumBitfieldStruct<u8, Bscbrah_SPEC>;
    impl Bscbrah {
        #[doc = "Disable GTCCRB input capture on the rising edge of GTIOCB input when GTIOCA input is 1"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRB input capture on the rising edge of GTIOCB input when GTIOCA input is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bscbral_SPEC;
    pub type Bscbral = crate::EnumBitfieldStruct<u8, Bscbral_SPEC>;
    impl Bscbral {
        #[doc = "Disable GTCCRB input capture on the rising edge of GTIOCB input when GTIOCA input is 0"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRB input capture on the rising edge of GTIOCB input when GTIOCA input is 0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bscafbh_SPEC;
    pub type Bscafbh = crate::EnumBitfieldStruct<u8, Bscafbh_SPEC>;
    impl Bscafbh {
        #[doc = "Disable GTCCRB input capture on the falling edge of GTIOCA input when GTIOCB input is 1"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRB input capture on the falling edge of GTIOCA input when GTIOCB input is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bscafbl_SPEC;
    pub type Bscafbl = crate::EnumBitfieldStruct<u8, Bscafbl_SPEC>;
    impl Bscafbl {
        #[doc = "Disable GTCCRB input capture on the falling edge of GTIOCA input when GTIOCB input is 0"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRB input capture on the falling edge of GTIOCA input when GTIOCB input is 0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bscarbh_SPEC;
    pub type Bscarbh = crate::EnumBitfieldStruct<u8, Bscarbh_SPEC>;
    impl Bscarbh {
        #[doc = "Disable GTCCRB input capture on the rising edge of GTIOCA input when GTIOCB input is 1"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRB input capture on the rising edge of GTIOCA input when GTIOCB input is 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bscarbl_SPEC;
    pub type Bscarbl = crate::EnumBitfieldStruct<u8, Bscarbl_SPEC>;
    impl Bscarbl {
        #[doc = "Disable GTCCRB input capture on the rising edge of GTIOCA input when GTIOCB input is 0"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRB input capture on the rising edge of GTIOCA input when GTIOCB input is 0."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsgtrgdf_SPEC;
    pub type Bsgtrgdf = crate::EnumBitfieldStruct<u8, Bsgtrgdf_SPEC>;
    impl Bsgtrgdf {
        #[doc = "Disable GTCCRB input capture on the falling edge of GTETRGD input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRB input capture on the falling edge of GTETRGD input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsgtrgdr_SPEC;
    pub type Bsgtrgdr = crate::EnumBitfieldStruct<u8, Bsgtrgdr_SPEC>;
    impl Bsgtrgdr {
        #[doc = "Disable GTCCRB input capture on the rising edge of GTETRGD input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRB input capture on the rising edge of GTETRGD input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsgtrgcf_SPEC;
    pub type Bsgtrgcf = crate::EnumBitfieldStruct<u8, Bsgtrgcf_SPEC>;
    impl Bsgtrgcf {
        #[doc = "Disable GTCCRB input capture on the falling edge of GTETRGC input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRB input capture on the falling edge of GTETRGC input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsgtrgcr_SPEC;
    pub type Bsgtrgcr = crate::EnumBitfieldStruct<u8, Bsgtrgcr_SPEC>;
    impl Bsgtrgcr {
        #[doc = "Disable GTCCRB input capture on the rising edge of GTETRGC input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRB input capture on the rising edge of GTETRGC input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsgtrgbf_SPEC;
    pub type Bsgtrgbf = crate::EnumBitfieldStruct<u8, Bsgtrgbf_SPEC>;
    impl Bsgtrgbf {
        #[doc = "Disable GTCCRB input capture on the falling edge of GTETRGB input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRB input capture on the falling edge of GTETRGB input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsgtrgbr_SPEC;
    pub type Bsgtrgbr = crate::EnumBitfieldStruct<u8, Bsgtrgbr_SPEC>;
    impl Bsgtrgbr {
        #[doc = "Disable GTCCRB input capture on the rising edge of GTETRGB input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRB input capture on the rising edge of GTETRGB input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsgtrgaf_SPEC;
    pub type Bsgtrgaf = crate::EnumBitfieldStruct<u8, Bsgtrgaf_SPEC>;
    impl Bsgtrgaf {
        #[doc = "Disable GTCCRB input capture on the falling edge of GTETRGA input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRB input capture on the falling edge of GTETRGA input."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bsgtrgar_SPEC;
    pub type Bsgtrgar = crate::EnumBitfieldStruct<u8, Bsgtrgar_SPEC>;
    impl Bsgtrgar {
        #[doc = "Disable GTCCRB input capture on the rising edge of GTETRGA input"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable GTCCRB input capture on the rising edge of GTETRGA input."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtcr_SPEC;
impl crate::sealed::RegSpec for Gtcr_SPEC {
    type DataType = u32;
}
#[doc = "General PWM Timer Control Register"]
pub type Gtcr = crate::RegValueT<Gtcr_SPEC>;

impl Gtcr {
    #[doc = "Timer Prescaler Select"]
    #[inline(always)]
    pub fn tpcs(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, gtcr::Tpcs, Gtcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x7,1,0,gtcr::Tpcs, Gtcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Mode Select"]
    #[inline(always)]
    pub fn md(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, gtcr::Md, Gtcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,gtcr::Md, Gtcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Count Start"]
    #[inline(always)]
    pub fn cst(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, gtcr::Cst, Gtcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,gtcr::Cst, Gtcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gtcr {
    #[inline(always)]
    fn default() -> Gtcr {
        <crate::RegValueT<Gtcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tpcs_SPEC;
    pub type Tpcs = crate::EnumBitfieldStruct<u8, Tpcs_SPEC>;
    impl Tpcs {
        #[doc = "PCLK/1"]
        pub const _000: Self = Self::new(0);
        #[doc = "PCLK/4"]
        pub const _001: Self = Self::new(1);
        #[doc = "PCLK/16"]
        pub const _010: Self = Self::new(2);
        #[doc = "PCLK/64"]
        pub const _011: Self = Self::new(3);
        #[doc = "PCLK/256"]
        pub const _100: Self = Self::new(4);
        #[doc = "PCLK/1024"]
        pub const _101: Self = Self::new(5);
        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Md_SPEC;
    pub type Md = crate::EnumBitfieldStruct<u8, Md_SPEC>;
    impl Md {
        #[doc = "Saw-wave PWM mode (single buffer or double buffer possible)"]
        pub const _000: Self = Self::new(0);
        #[doc = "Saw-wave one-shot pulse mode (fixed buffer operation)"]
        pub const _001: Self = Self::new(1);
        #[doc = "Setting prohibited"]
        pub const _010: Self = Self::new(2);
        #[doc = "Setting prohibited"]
        pub const _011: Self = Self::new(3);
        #[doc = "Triangle-wave PWM mode 1 (32-bit transfer at crest) (single buffer or double buffer possible)"]
        pub const _100: Self = Self::new(4);
        #[doc = "Triangle-wave PWM mode 2 (32-bit transfer at crest and trough) (single buffer or double buffer possible)"]
        pub const _101: Self = Self::new(5);
        #[doc = "Triangle-wave PWM mode 3 (64-bit transfer at trough) fixed buffer operation)"]
        pub const _110: Self = Self::new(6);
        #[doc = "Setting prohibited"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cst_SPEC;
    pub type Cst = crate::EnumBitfieldStruct<u8, Cst_SPEC>;
    impl Cst {
        #[doc = "Count operation is stopped"]
        pub const _0: Self = Self::new(0);
        #[doc = "Count operation is performed"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtuddtyc_SPEC;
impl crate::sealed::RegSpec for Gtuddtyc_SPEC {
    type DataType = u32;
}
#[doc = "General PWM Timer Count Direction and Duty Setting Register"]
pub type Gtuddtyc = crate::RegValueT<Gtuddtyc_SPEC>;

impl Gtuddtyc {
    #[doc = "GTIOCB Output Value Selecting after Releasing  0 percent/100 percent Duty Setting"]
    #[inline(always)]
    pub fn obdtyr(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        gtuddtyc::Obdtyr,
        Gtuddtyc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            gtuddtyc::Obdtyr,
            Gtuddtyc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Forcible GTIOCB Output Duty Setting"]
    #[inline(always)]
    pub fn obdtyf(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        gtuddtyc::Obdtyf,
        Gtuddtyc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            gtuddtyc::Obdtyf,
            Gtuddtyc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "GTIOCB Output Duty Setting"]
    #[inline(always)]
    pub fn obdty(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x3,
        1,
        0,
        gtuddtyc::Obdty,
        Gtuddtyc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x3,
            1,
            0,
            gtuddtyc::Obdty,
            Gtuddtyc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "GTIOCA Output Value Selecting after Releasing  0 percent/100 percent Duty Setting"]
    #[inline(always)]
    pub fn oadtyr(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        gtuddtyc::Oadtyr,
        Gtuddtyc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            gtuddtyc::Oadtyr,
            Gtuddtyc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Forcible GTIOCA Output Duty Setting"]
    #[inline(always)]
    pub fn oadtyf(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        gtuddtyc::Oadtyf,
        Gtuddtyc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            gtuddtyc::Oadtyf,
            Gtuddtyc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "GTIOCA Output Duty Setting"]
    #[inline(always)]
    pub fn oadty(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3,
        1,
        0,
        gtuddtyc::Oadty,
        Gtuddtyc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            gtuddtyc::Oadty,
            Gtuddtyc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Forcible Count Direction Setting"]
    #[inline(always)]
    pub fn udf(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, gtuddtyc::Udf, Gtuddtyc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,gtuddtyc::Udf, Gtuddtyc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Count Direction Setting"]
    #[inline(always)]
    pub fn ud(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, gtuddtyc::Ud, Gtuddtyc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,gtuddtyc::Ud, Gtuddtyc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gtuddtyc {
    #[inline(always)]
    fn default() -> Gtuddtyc {
        <crate::RegValueT<Gtuddtyc_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod gtuddtyc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Obdtyr_SPEC;
    pub type Obdtyr = crate::EnumBitfieldStruct<u8, Obdtyr_SPEC>;
    impl Obdtyr {
        #[doc = "Apply output value set in 0 percent/100 percent duty to GTIOB\\[3:2\\] function after releasing 0percent/100percent duty setting."]
        pub const _0: Self = Self::new(0);
        #[doc = "Apply masked compare match output value to GTIOB\\[3:2\\] function after releasing 0percent/100percent duty setting."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Obdtyf_SPEC;
    pub type Obdtyf = crate::EnumBitfieldStruct<u8, Obdtyf_SPEC>;
    impl Obdtyf {
        #[doc = "Do not force setting"]
        pub const _0: Self = Self::new(0);
        #[doc = "Force setting"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Obdty_SPEC;
    pub type Obdty = crate::EnumBitfieldStruct<u8, Obdty_SPEC>;
    impl Obdty {
        #[doc = "GTIOCB pin duty is depend on compare match"]
        pub const _00: Self = Self::new(0);
        #[doc = "GTIOCB pin duty is depend on compare match"]
        pub const _01: Self = Self::new(1);
        #[doc = "GTIOCB pin duty 0percent"]
        pub const _10: Self = Self::new(2);
        #[doc = "GTIOCB pin duty 100percent"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oadtyr_SPEC;
    pub type Oadtyr = crate::EnumBitfieldStruct<u8, Oadtyr_SPEC>;
    impl Oadtyr {
        #[doc = "Apply output value set in 0 percent/100 percent duty to GTIOA\\[3:2\\] function after releasing 0 percent/100 percent duty setting."]
        pub const _0: Self = Self::new(0);
        #[doc = "Apply masked compare match output value to GTIOA\\[3:2\\] function after releasing 0 percent/100 percent duty setting."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oadtyf_SPEC;
    pub type Oadtyf = crate::EnumBitfieldStruct<u8, Oadtyf_SPEC>;
    impl Oadtyf {
        #[doc = "Do not force setting"]
        pub const _0: Self = Self::new(0);
        #[doc = "Force setting"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oadty_SPEC;
    pub type Oadty = crate::EnumBitfieldStruct<u8, Oadty_SPEC>;
    impl Oadty {
        #[doc = "GTIOCA pin duty is depend on compare match"]
        pub const _00: Self = Self::new(0);
        #[doc = "GTIOCA pin duty is depend on compare match"]
        pub const _01: Self = Self::new(1);
        #[doc = "GTIOCA pin duty 0 percent"]
        pub const _10: Self = Self::new(2);
        #[doc = "GTIOCA pin duty 100 percent"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Udf_SPEC;
    pub type Udf = crate::EnumBitfieldStruct<u8, Udf_SPEC>;
    impl Udf {
        #[doc = "Do not force setting"]
        pub const _0: Self = Self::new(0);
        #[doc = "Force setting"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ud_SPEC;
    pub type Ud = crate::EnumBitfieldStruct<u8, Ud_SPEC>;
    impl Ud {
        #[doc = "Count down on GTCNT"]
        pub const _0: Self = Self::new(0);
        #[doc = "Counts up on GTCNT"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtior_SPEC;
impl crate::sealed::RegSpec for Gtior_SPEC {
    type DataType = u32;
}
#[doc = "General PWM Timer I/O Control Register"]
pub type Gtior = crate::RegValueT<Gtior_SPEC>;

impl Gtior {
    #[doc = "Noise Filter B Sampling Clock Select"]
    #[inline(always)]
    pub fn nfcsb(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, gtior::Nfcsb, Gtior_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x3,1,0,gtior::Nfcsb, Gtior_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Noise Filter B Enable"]
    #[inline(always)]
    pub fn nfben(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, gtior::Nfben, Gtior_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,gtior::Nfben, Gtior_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCB Pin Disable Value Setting"]
    #[inline(always)]
    pub fn obdf(
        self,
    ) -> crate::common::RegisterField<25, 0x3, 1, 0, gtior::Obdf, Gtior_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x3,1,0,gtior::Obdf, Gtior_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCB Pin Output Enable"]
    #[inline(always)]
    pub fn obe(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, gtior::Obe, Gtior_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,gtior::Obe, Gtior_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCB Pin Output Setting at the Start/Stop Count"]
    #[inline(always)]
    pub fn obhld(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, gtior::Obhld, Gtior_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,gtior::Obhld, Gtior_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCB Pin Output Value Setting at the Count Stop"]
    #[inline(always)]
    pub fn obdflt(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, gtior::Obdflt, Gtior_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,gtior::Obdflt, Gtior_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCB Pin Function Select"]
    #[inline(always)]
    pub fn gtiob(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, gtior::Gtiob, Gtior_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1f,1,0,gtior::Gtiob, Gtior_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Noise Filter A Sampling Clock Select"]
    #[inline(always)]
    pub fn nfcsa(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, gtior::Nfcsa, Gtior_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,gtior::Nfcsa, Gtior_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Noise Filter A Enable"]
    #[inline(always)]
    pub fn nfaen(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, gtior::Nfaen, Gtior_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,gtior::Nfaen, Gtior_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCA Pin Disable Value Setting"]
    #[inline(always)]
    pub fn oadf(
        self,
    ) -> crate::common::RegisterField<9, 0x3, 1, 0, gtior::Oadf, Gtior_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x3,1,0,gtior::Oadf, Gtior_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCA Pin Output Enable"]
    #[inline(always)]
    pub fn oae(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, gtior::Oae, Gtior_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1,1,0,gtior::Oae, Gtior_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCA Pin Output Setting at the Start/Stop Count"]
    #[inline(always)]
    pub fn oahld(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, gtior::Oahld, Gtior_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,gtior::Oahld, Gtior_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCA Pin Output Value Setting at the Count Stop"]
    #[inline(always)]
    pub fn oadflt(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, gtior::Oadflt, Gtior_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,gtior::Oadflt, Gtior_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTIOCA Pin Function Select"]
    #[inline(always)]
    pub fn gtioa(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, gtior::Gtioa, Gtior_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,gtior::Gtioa, Gtior_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gtior {
    #[inline(always)]
    fn default() -> Gtior {
        <crate::RegValueT<Gtior_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtior {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfcsb_SPEC;
    pub type Nfcsb = crate::EnumBitfieldStruct<u8, Nfcsb_SPEC>;
    impl Nfcsb {
        #[doc = "PCLK/1"]
        pub const _00: Self = Self::new(0);
        #[doc = "PCLK/4"]
        pub const _01: Self = Self::new(1);
        #[doc = "PCLK/16"]
        pub const _10: Self = Self::new(2);
        #[doc = "PCLK/64"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfben_SPEC;
    pub type Nfben = crate::EnumBitfieldStruct<u8, Nfben_SPEC>;
    impl Nfben {
        #[doc = "Disable noise filter for GTIOCB pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable noise filter for GTIOCB pin"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Obdf_SPEC;
    pub type Obdf = crate::EnumBitfieldStruct<u8, Obdf_SPEC>;
    impl Obdf {
        #[doc = "Prohibit output disable"]
        pub const _00: Self = Self::new(0);
        #[doc = "Set GTIOCB pin to Hi-Z on output disable"]
        pub const _01: Self = Self::new(1);
        #[doc = "Set GTIOCB pin to 0 on output disable"]
        pub const _10: Self = Self::new(2);
        #[doc = "Set GTIOCB pin to 1 on output disable."]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Obe_SPEC;
    pub type Obe = crate::EnumBitfieldStruct<u8, Obe_SPEC>;
    impl Obe {
        #[doc = "Disable output"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable output"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Obhld_SPEC;
    pub type Obhld = crate::EnumBitfieldStruct<u8, Obhld_SPEC>;
    impl Obhld {
        #[doc = "Set GTIOCB pin output level on counting start and stop based on the register setting"]
        pub const _0: Self = Self::new(0);
        #[doc = "Retain GTIOCB pin output level on counting start and stop"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Obdflt_SPEC;
    pub type Obdflt = crate::EnumBitfieldStruct<u8, Obdflt_SPEC>;
    impl Obdflt {
        #[doc = "Output low on GTIOCB pin when counting stops"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output high on GTIOCB pin when counting stops"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gtiob_SPEC;
    pub type Gtiob = crate::EnumBitfieldStruct<u8, Gtiob_SPEC>;
    impl Gtiob {
        #[doc = "Initial output is Low. Output retained at cycle end. Output retained at GTCCRB compare match."]
        pub const _00000: Self = Self::new(0);
        #[doc = "Initial output is Low. Output retained at cycle end. Low output at GTCCRB compare match."]
        pub const _00001: Self = Self::new(1);
        #[doc = "Initial output is Low. Output retained at cycle end. High output at GTCCRB compare match."]
        pub const _00010: Self = Self::new(2);
        #[doc = "Initial output is Low. Output retained at cycle end. Output toggled at GTCCRB compare match."]
        pub const _00011: Self = Self::new(3);
        #[doc = "Initial output is Low. Low output at cycle end. Output retained at GTCCRB compare match."]
        pub const _00100: Self = Self::new(4);
        #[doc = "Initial output is Low. Low output at cycle end. Low output at GTCCRB compare match."]
        pub const _00101: Self = Self::new(5);
        #[doc = "Initial output is Low. Low output at cycle end. High output at GTCCRB compare match."]
        pub const _00110: Self = Self::new(6);
        #[doc = "Initial output is Low. Low output at cycle end. Output toggled at GTCCRB compare match."]
        pub const _00111: Self = Self::new(7);
        #[doc = "Initial output is Low. High output at cycle end. Output retained at GTCCRB compare match."]
        pub const _01000: Self = Self::new(8);
        #[doc = "Initial output is Low. High output at cycle end. Low output at GTCCRB compare match."]
        pub const _01001: Self = Self::new(9);
        #[doc = "Initial output is Low. High output at cycle end. High output at GTCCRB compare match."]
        pub const _01010: Self = Self::new(10);
        #[doc = "Initial output is Low. High output at cycle end. Output toggled at GTCCRB compare match."]
        pub const _01011: Self = Self::new(11);
        #[doc = "Initial output is Low. Output toggled at cycle end. Output retained at GTCCRB compare match."]
        pub const _01100: Self = Self::new(12);
        #[doc = "Initial output is Low. Output toggled at cycle end. Low output at GTCCRB compare match."]
        pub const _01101: Self = Self::new(13);
        #[doc = "Initial output is Low. Output toggled at cycle end. High output at GTCCRB compare match."]
        pub const _01110: Self = Self::new(14);
        #[doc = "Initial output is Low. Output toggled at cycle end. Output toggled at GTCCRB compare match."]
        pub const _01111: Self = Self::new(15);
        #[doc = "Initial output is High. Output retained at cycle end. Output retained at GTCCRB compare match."]
        pub const _10000: Self = Self::new(16);
        #[doc = "Initial output is High. Output retained at cycle end. Low output at GTCCRB compare match."]
        pub const _10001: Self = Self::new(17);
        #[doc = "Initial output is High. Output retained at cycle end. High output at GTCCRB compare match."]
        pub const _10010: Self = Self::new(18);
        #[doc = "Initial output is High. Output retained at cycle end. Output toggled at GTCCRB compare match."]
        pub const _10011: Self = Self::new(19);
        #[doc = "Initial output is High. Low output at cycle end. Output retained at GTCCRB compare match."]
        pub const _10100: Self = Self::new(20);
        #[doc = "Initial output is High. Low output at cycle end. Low output at GTCCRB compare match."]
        pub const _10101: Self = Self::new(21);
        #[doc = "Initial output is High. Low output at cycle end. High output at GTCCRB compare match."]
        pub const _10110: Self = Self::new(22);
        #[doc = "Initial output is High. Low output at cycle end. Output toggled at GTCCRB compare match."]
        pub const _10111: Self = Self::new(23);
        #[doc = "Initial output is High. High output at cycle end. Output retained at GTCCRB compare match."]
        pub const _11000: Self = Self::new(24);
        #[doc = "Initial output is High. High output at cycle end. Low output at GTCCRB compare match."]
        pub const _11001: Self = Self::new(25);
        #[doc = "Initial output is High. High output at cycle end. High output at GTCCRB compare match."]
        pub const _11010: Self = Self::new(26);
        #[doc = "Initial output is High. High output at cycle end. Output toggled at GTCCRB compare match."]
        pub const _11011: Self = Self::new(27);
        #[doc = "Initial output is High. Output toggled at cycle end. Output retained at GTCCRB compare match."]
        pub const _11100: Self = Self::new(28);
        #[doc = "Initial output is High. Output toggled at cycle end. Low output at GTCCRB compare match."]
        pub const _11101: Self = Self::new(29);
        #[doc = "Initial output is High. Output toggled at cycle end. High output at GTCCRB compare match."]
        pub const _11110: Self = Self::new(30);
        #[doc = "Initial output is High. Output toggled at cycle end. Output toggled at GTCCRB compare match."]
        pub const _11111: Self = Self::new(31);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfcsa_SPEC;
    pub type Nfcsa = crate::EnumBitfieldStruct<u8, Nfcsa_SPEC>;
    impl Nfcsa {
        #[doc = "PCLK/1"]
        pub const _00: Self = Self::new(0);
        #[doc = "PCLK/4"]
        pub const _01: Self = Self::new(1);
        #[doc = "PCLK/16"]
        pub const _10: Self = Self::new(2);
        #[doc = "PCLK/64"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfaen_SPEC;
    pub type Nfaen = crate::EnumBitfieldStruct<u8, Nfaen_SPEC>;
    impl Nfaen {
        #[doc = "Disable noise filter for GTIOCA pin"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable noise filter for GTIOCA pin."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oadf_SPEC;
    pub type Oadf = crate::EnumBitfieldStruct<u8, Oadf_SPEC>;
    impl Oadf {
        #[doc = "Prohibit output disable"]
        pub const _00: Self = Self::new(0);
        #[doc = "Set GTIOCA pin to Hi-Z on output disable"]
        pub const _01: Self = Self::new(1);
        #[doc = "Set GTIOCA pin to 0 on output disable"]
        pub const _10: Self = Self::new(2);
        #[doc = "Set GTIOCA pin to 1 on output disable."]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oae_SPEC;
    pub type Oae = crate::EnumBitfieldStruct<u8, Oae_SPEC>;
    impl Oae {
        #[doc = "Disable output"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable output."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oahld_SPEC;
    pub type Oahld = crate::EnumBitfieldStruct<u8, Oahld_SPEC>;
    impl Oahld {
        #[doc = "Set GTIOCA pin output level on counting start and stop based on the register setting."]
        pub const _0: Self = Self::new(0);
        #[doc = "Retain GTIOCA pin output level on counting start and stop"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oadflt_SPEC;
    pub type Oadflt = crate::EnumBitfieldStruct<u8, Oadflt_SPEC>;
    impl Oadflt {
        #[doc = "Output low on GTIOCA pin when counting stops"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output high on GTIOCA pin when counting stops."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gtioa_SPEC;
    pub type Gtioa = crate::EnumBitfieldStruct<u8, Gtioa_SPEC>;
    impl Gtioa {
        #[doc = "Initial output is Low. Output retained at cycle end. Output retained at GTCCRA compare match."]
        pub const _00000: Self = Self::new(0);
        #[doc = "Initial output is Low. Output retained at cycle end. Low output at GTCCRA compare match."]
        pub const _00001: Self = Self::new(1);
        #[doc = "Initial output is Low. Output retained at cycle end. High output at GTCCRA compare match."]
        pub const _00010: Self = Self::new(2);
        #[doc = "Initial output is Low. Output retained at cycle end. Output toggled at GTCCRA compare match."]
        pub const _00011: Self = Self::new(3);
        #[doc = "Initial output is Low. Low output at cycle end. Output retained at GTCCRA compare match."]
        pub const _00100: Self = Self::new(4);
        #[doc = "Initial output is Low. Low output at cycle end. Low output at GTCCRA compare match."]
        pub const _00101: Self = Self::new(5);
        #[doc = "Initial output is Low. Low output at cycle end. High output at GTCCRA compare match."]
        pub const _00110: Self = Self::new(6);
        #[doc = "Initial output is Low. Low output at cycle end. Output toggled at GTCCRA compare match."]
        pub const _00111: Self = Self::new(7);
        #[doc = "Initial output is Low. High output at cycle end. Output retained at GTCCRA compare match."]
        pub const _01000: Self = Self::new(8);
        #[doc = "Initial output is Low. High output at cycle end. Low output at GTCCRA compare match."]
        pub const _01001: Self = Self::new(9);
        #[doc = "Initial output is Low. High output at cycle end. High output at GTCCRA compare match."]
        pub const _01010: Self = Self::new(10);
        #[doc = "Initial output is Low. High output at cycle end. Output toggled at GTCCRA compare match."]
        pub const _01011: Self = Self::new(11);
        #[doc = "Initial output is Low. Output toggled at cycle end. Output retained at GTCCRA compare match."]
        pub const _01100: Self = Self::new(12);
        #[doc = "Initial output is Low. Output toggled at cycle end. Low output at GTCCRA compare match."]
        pub const _01101: Self = Self::new(13);
        #[doc = "Initial output is Low. Output toggled at cycle end. High output at GTCCRA compare match."]
        pub const _01110: Self = Self::new(14);
        #[doc = "Initial output is Low. Output toggled at cycle end. Output toggled at GTCCRA compare match."]
        pub const _01111: Self = Self::new(15);
        #[doc = "Initial output is High. Output retained at cycle end. Output retained at GTCCRA compare match."]
        pub const _10000: Self = Self::new(16);
        #[doc = "Initial output is High. Output retained at cycle end. Low output at GTCCRA compare match."]
        pub const _10001: Self = Self::new(17);
        #[doc = "Initial output is High. Output retained at cycle end. High output at GTCCRA compare match."]
        pub const _10010: Self = Self::new(18);
        #[doc = "Initial output is High. Output retained at cycle end. Output toggled at GTCCRA compare match."]
        pub const _10011: Self = Self::new(19);
        #[doc = "Initial output is High. Low output at cycle end. Output retained at GTCCRA compare match."]
        pub const _10100: Self = Self::new(20);
        #[doc = "Initial output is High. Low output at cycle end. Low output at GTCCRA compare match."]
        pub const _10101: Self = Self::new(21);
        #[doc = "Initial output is High. Low output at cycle end. High output at GTCCRA compare match."]
        pub const _10110: Self = Self::new(22);
        #[doc = "Initial output is High. Low output at cycle end. Output toggled at GTCCRA compare match."]
        pub const _10111: Self = Self::new(23);
        #[doc = "Initial output is High. High output at cycle end. Output retained at GTCCRA compare match."]
        pub const _11000: Self = Self::new(24);
        #[doc = "Initial output is High. High output at cycle end. Low output at GTCCRA compare match."]
        pub const _11001: Self = Self::new(25);
        #[doc = "Initial output is High. High output at cycle end. High output at GTCCRA compare match."]
        pub const _11010: Self = Self::new(26);
        #[doc = "Initial output is High. High output at cycle end. Output toggled at GTCCRA compare match."]
        pub const _11011: Self = Self::new(27);
        #[doc = "Initial output is High. Output toggled at cycle end. Output retained at GTCCRA compare match."]
        pub const _11100: Self = Self::new(28);
        #[doc = "Initial output is High. Output toggled at cycle end. Low output at GTCCRA compare match."]
        pub const _11101: Self = Self::new(29);
        #[doc = "Initial output is High. Output toggled at cycle end. High output at GTCCRA compare match."]
        pub const _11110: Self = Self::new(30);
        #[doc = "Initial output is High. Output toggled at cycle end. Output toggled at GTCCRA compare match."]
        pub const _11111: Self = Self::new(31);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtintad_SPEC;
impl crate::sealed::RegSpec for Gtintad_SPEC {
    type DataType = u32;
}
#[doc = "General PWM Timer Interrupt Output Setting Register"]
pub type Gtintad = crate::RegValueT<Gtintad_SPEC>;

impl Gtintad {
    #[doc = "Same Time Output Level Low Disable Request Enable"]
    #[inline(always)]
    pub fn grpabl(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, gtintad::Grpabl, Gtintad_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            gtintad::Grpabl,
            Gtintad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Same Time Output Level High Disable Request Enable"]
    #[inline(always)]
    pub fn grpabh(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, gtintad::Grpabh, Gtintad_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            gtintad::Grpabh,
            Gtintad_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Output Disable Source Select"]
    #[inline(always)]
    pub fn grp(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, gtintad::Grp, Gtintad_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x3,1,0,gtintad::Grp, Gtintad_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gtintad {
    #[inline(always)]
    fn default() -> Gtintad {
        <crate::RegValueT<Gtintad_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtintad {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Grpabl_SPEC;
    pub type Grpabl = crate::EnumBitfieldStruct<u8, Grpabl_SPEC>;
    impl Grpabl {
        #[doc = "Disable same time output level low disable request"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable same time output level low disable request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Grpabh_SPEC;
    pub type Grpabh = crate::EnumBitfieldStruct<u8, Grpabh_SPEC>;
    impl Grpabh {
        #[doc = "Disable same time output level high disable request"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable same time output level high disable request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Grp_SPEC;
    pub type Grp = crate::EnumBitfieldStruct<u8, Grp_SPEC>;
    impl Grp {
        #[doc = "Select Group A output disable request"]
        pub const _00: Self = Self::new(0);
        #[doc = "Select Group B output disable request"]
        pub const _01: Self = Self::new(1);
        #[doc = "Select Group C output disable request"]
        pub const _10: Self = Self::new(2);
        #[doc = "Select Group D output disable request."]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtst_SPEC;
impl crate::sealed::RegSpec for Gtst_SPEC {
    type DataType = u32;
}
#[doc = "General PWM Timer Status Register"]
pub type Gtst = crate::RegValueT<Gtst_SPEC>;

impl Gtst {
    #[doc = "Same Time Output Level Low Disable Request Enable"]
    #[inline(always)]
    pub fn oablf(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, gtst::Oablf, Gtst_SPEC, crate::common::R> {
        crate::common::RegisterField::<30,0x1,1,0,gtst::Oablf, Gtst_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Same Time Output Level High Disable Request Enable"]
    #[inline(always)]
    pub fn oabhf(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, gtst::Oabhf, Gtst_SPEC, crate::common::R> {
        crate::common::RegisterField::<29,0x1,1,0,gtst::Oabhf, Gtst_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Output Disable Flag"]
    #[inline(always)]
    pub fn odf(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, gtst::Odf, Gtst_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x1,1,0,gtst::Odf, Gtst_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Count Direction Flag"]
    #[inline(always)]
    pub fn tucf(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, gtst::Tucf, Gtst_SPEC, crate::common::R> {
        crate::common::RegisterField::<15,0x1,1,0,gtst::Tucf, Gtst_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Underflow Flag"]
    #[inline(always)]
    pub fn tcfpu(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, gtst::Tcfpu, Gtst_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x1,1,0,gtst::Tcfpu, Gtst_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Overflow Flag"]
    #[inline(always)]
    pub fn tcpfo(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, gtst::Tcpfo, Gtst_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x1,1,0,gtst::Tcpfo, Gtst_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Compare Match Flag F"]
    #[inline(always)]
    pub fn tcff(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, gtst::Tcff, Gtst_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x1,1,0,gtst::Tcff, Gtst_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Compare Match Flag E"]
    #[inline(always)]
    pub fn tcfe(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, gtst::Tcfe, Gtst_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,gtst::Tcfe, Gtst_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Compare Match Flag D"]
    #[inline(always)]
    pub fn tcfd(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, gtst::Tcfd, Gtst_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,gtst::Tcfd, Gtst_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Compare Match Flag C"]
    #[inline(always)]
    pub fn tcfc(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, gtst::Tcfc, Gtst_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,gtst::Tcfc, Gtst_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Capture/Compare Match Flag B"]
    #[inline(always)]
    pub fn tcfb(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, gtst::Tcfb, Gtst_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,gtst::Tcfb, Gtst_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Capture/Compare Match Flag A"]
    #[inline(always)]
    pub fn tcfa(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, gtst::Tcfa, Gtst_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,gtst::Tcfa, Gtst_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gtst {
    #[inline(always)]
    fn default() -> Gtst {
        <crate::RegValueT<Gtst_SPEC> as RegisterValue<_>>::new(32768)
    }
}
pub mod gtst {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oablf_SPEC;
    pub type Oablf = crate::EnumBitfieldStruct<u8, Oablf_SPEC>;
    impl Oablf {
        #[doc = "GTIOCA pin and GTIOCB pin don\'t output 0 at the same time."]
        pub const _0: Self = Self::new(0);
        #[doc = "GTIOCA pin and GTIOCB pin output 0 at the same time."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Oabhf_SPEC;
    pub type Oabhf = crate::EnumBitfieldStruct<u8, Oabhf_SPEC>;
    impl Oabhf {
        #[doc = "GTIOCA pin and GTIOCB pin don\'t output 1 at the same time."]
        pub const _0: Self = Self::new(0);
        #[doc = "GTIOCA pin and GTIOCB pin output 1 at the same time."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Odf_SPEC;
    pub type Odf = crate::EnumBitfieldStruct<u8, Odf_SPEC>;
    impl Odf {
        #[doc = "No output disable request is generated."]
        pub const _0: Self = Self::new(0);
        #[doc = "An output disable request is generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tucf_SPEC;
    pub type Tucf = crate::EnumBitfieldStruct<u8, Tucf_SPEC>;
    impl Tucf {
        #[doc = "GTCNT counter is counting down"]
        pub const _0: Self = Self::new(0);
        #[doc = "GTCNT counter is counting up."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcfpu_SPEC;
    pub type Tcfpu = crate::EnumBitfieldStruct<u8, Tcfpu_SPEC>;
    impl Tcfpu {
        #[doc = "No underflow (trough) has occurred."]
        pub const _0: Self = Self::new(0);
        #[doc = "An underflow (trough) has occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcpfo_SPEC;
    pub type Tcpfo = crate::EnumBitfieldStruct<u8, Tcpfo_SPEC>;
    impl Tcpfo {
        #[doc = "No overflow (crest) has occurred."]
        pub const _0: Self = Self::new(0);
        #[doc = "An overflow (crest) has occurred."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcff_SPEC;
    pub type Tcff = crate::EnumBitfieldStruct<u8, Tcff_SPEC>;
    impl Tcff {
        #[doc = "No compare match of GTCCRF is generated."]
        pub const _0: Self = Self::new(0);
        #[doc = "A compare match of GTCCRF is generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcfe_SPEC;
    pub type Tcfe = crate::EnumBitfieldStruct<u8, Tcfe_SPEC>;
    impl Tcfe {
        #[doc = "No compare match of GTCCRE is generated."]
        pub const _0: Self = Self::new(0);
        #[doc = "A compare match of GTCCRE is generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcfd_SPEC;
    pub type Tcfd = crate::EnumBitfieldStruct<u8, Tcfd_SPEC>;
    impl Tcfd {
        #[doc = "No compare match of GTCCRD is generated."]
        pub const _0: Self = Self::new(0);
        #[doc = "A compare match of GTCCRD is generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcfc_SPEC;
    pub type Tcfc = crate::EnumBitfieldStruct<u8, Tcfc_SPEC>;
    impl Tcfc {
        #[doc = "No compare match of GTCCRC is generated."]
        pub const _0: Self = Self::new(0);
        #[doc = "A compare match of GTCCRC is generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcfb_SPEC;
    pub type Tcfb = crate::EnumBitfieldStruct<u8, Tcfb_SPEC>;
    impl Tcfb {
        #[doc = "No input capture/compare match of GTCCRB is generated."]
        pub const _0: Self = Self::new(0);
        #[doc = "An input capture/compare match of GTCCRB is generated."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcfa_SPEC;
    pub type Tcfa = crate::EnumBitfieldStruct<u8, Tcfa_SPEC>;
    impl Tcfa {
        #[doc = "No input capture/compare match of GTCCRA is generated."]
        pub const _0: Self = Self::new(0);
        #[doc = "An input capture/compare match of GTCCRA is generated."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtber_SPEC;
impl crate::sealed::RegSpec for Gtber_SPEC {
    type DataType = u32;
}
#[doc = "General PWM Timer Buffer Enable Register"]
pub type Gtber = crate::RegValueT<Gtber_SPEC>;

impl Gtber {
    #[doc = "GTCCRA and GTCCRB Forcible Buffer OperationThis bit is read as 0."]
    #[inline(always)]
    pub fn ccrswt(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, gtber::Ccrswt, Gtber_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<22,0x1,1,0,gtber::Ccrswt, Gtber_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "GTPR Buffer Operation"]
    #[inline(always)]
    pub fn pr(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, gtber::Pr, Gtber_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3,1,0,gtber::Pr, Gtber_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTCCRB Buffer Operation"]
    #[inline(always)]
    pub fn ccrb(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, gtber::Ccrb, Gtber_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x3,1,0,gtber::Ccrb, Gtber_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTCCRA Buffer Operation"]
    #[inline(always)]
    pub fn ccra(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, gtber::Ccra, Gtber_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3,1,0,gtber::Ccra, Gtber_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTPR Buffer Operation Disable"]
    #[inline(always)]
    pub fn bd1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, gtber::Bd1, Gtber_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,gtber::Bd1, Gtber_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTCCR Buffer Operation Disable"]
    #[inline(always)]
    pub fn bd2(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, gtber::Bd2, Gtber_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,gtber::Bd2, Gtber_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gtber {
    #[inline(always)]
    fn default() -> Gtber {
        <crate::RegValueT<Gtber_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtber {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccrswt_SPEC;
    pub type Ccrswt = crate::EnumBitfieldStruct<u8, Ccrswt_SPEC>;
    impl Ccrswt {
        #[doc = "no effect"]
        pub const _0: Self = Self::new(0);
        #[doc = "Forcibly performs buffer transfer of GTCCRA and GTCCRB. This bit automatically returns to 0 after the writing of 1."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pr_SPEC;
    pub type Pr = crate::EnumBitfieldStruct<u8, Pr_SPEC>;
    impl Pr {
        #[doc = "Buffer operation is not performed"]
        pub const _00: Self = Self::new(0);
        #[doc = "Single buffer operation (GTPBR --> GTPR)"]
        pub const _01: Self = Self::new(1);
        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccrb_SPEC;
    pub type Ccrb = crate::EnumBitfieldStruct<u8, Ccrb_SPEC>;
    impl Ccrb {
        #[doc = "Buffer operation is not performed"]
        pub const _00: Self = Self::new(0);
        #[doc = "Single buffer operation (GTCCRB <--> GTCCRE)"]
        pub const _01: Self = Self::new(1);
        #[doc = "Double buffer operation (GTCCRB <--> GTCCRE <--> GTCCRF)"]
        pub const _10: Self = Self::new(2);
        #[doc = "Double buffer operation (GTCCRB <--> GTCCRE <--> GTCCRF)"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ccra_SPEC;
    pub type Ccra = crate::EnumBitfieldStruct<u8, Ccra_SPEC>;
    impl Ccra {
        #[doc = "Buffer operation is not performed"]
        pub const _00: Self = Self::new(0);
        #[doc = "Single buffer operation (GTCCRA <--> GTCCRC)"]
        pub const _01: Self = Self::new(1);
        #[doc = "Double buffer operation (GTCCRA <--> GTCCRC <--> GTCCRD)"]
        pub const _10: Self = Self::new(2);
        #[doc = "Double buffer operation (GTCCRA <--> GTCCRC <--> GTCCRD)"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bd1_SPEC;
    pub type Bd1 = crate::EnumBitfieldStruct<u8, Bd1_SPEC>;
    impl Bd1 {
        #[doc = "Enable buffer operation"]
        pub const _0: Self = Self::new(0);
        #[doc = "Disable buffer operation."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bd2_SPEC;
    pub type Bd2 = crate::EnumBitfieldStruct<u8, Bd2_SPEC>;
    impl Bd2 {
        #[doc = "Enable buffer operation"]
        pub const _0: Self = Self::new(0);
        #[doc = "Disable buffer operation."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtcnt_SPEC;
impl crate::sealed::RegSpec for Gtcnt_SPEC {
    type DataType = u32;
}
#[doc = "General PWM Timer Counter"]
pub type Gtcnt = crate::RegValueT<Gtcnt_SPEC>;

impl Gtcnt {
    #[doc = "Counter"]
    #[inline(always)]
    pub fn gtcnt(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Gtcnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Gtcnt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gtcnt {
    #[inline(always)]
    fn default() -> Gtcnt {
        <crate::RegValueT<Gtcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtccra_SPEC;
impl crate::sealed::RegSpec for Gtccra_SPEC {
    type DataType = u32;
}
#[doc = "General PWM Timer Compare Capture Register A"]
pub type Gtccra = crate::RegValueT<Gtccra_SPEC>;

impl Gtccra {
    #[doc = "Compare Capture Register A"]
    #[inline(always)]
    pub fn gtccra(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Gtccra_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Gtccra_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gtccra {
    #[inline(always)]
    fn default() -> Gtccra {
        <crate::RegValueT<Gtccra_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtccrb_SPEC;
impl crate::sealed::RegSpec for Gtccrb_SPEC {
    type DataType = u32;
}
#[doc = "General PWM Timer Compare Capture Register B"]
pub type Gtccrb = crate::RegValueT<Gtccrb_SPEC>;

impl Gtccrb {
    #[doc = "Compare Capture Register B"]
    #[inline(always)]
    pub fn gtccrb(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Gtccrb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Gtccrb_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gtccrb {
    #[inline(always)]
    fn default() -> Gtccrb {
        <crate::RegValueT<Gtccrb_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtccrc_SPEC;
impl crate::sealed::RegSpec for Gtccrc_SPEC {
    type DataType = u32;
}
#[doc = "General PWM Timer Compare Capture Register C"]
pub type Gtccrc = crate::RegValueT<Gtccrc_SPEC>;

impl Gtccrc {
    #[doc = "Compare Capture Register C"]
    #[inline(always)]
    pub fn gtccrc(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Gtccrc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Gtccrc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gtccrc {
    #[inline(always)]
    fn default() -> Gtccrc {
        <crate::RegValueT<Gtccrc_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtccre_SPEC;
impl crate::sealed::RegSpec for Gtccre_SPEC {
    type DataType = u32;
}
#[doc = "General PWM Timer Compare Capture Register E"]
pub type Gtccre = crate::RegValueT<Gtccre_SPEC>;

impl Gtccre {
    #[doc = "Compare Capture Register E"]
    #[inline(always)]
    pub fn gtccre(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Gtccre_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Gtccre_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gtccre {
    #[inline(always)]
    fn default() -> Gtccre {
        <crate::RegValueT<Gtccre_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtccrd_SPEC;
impl crate::sealed::RegSpec for Gtccrd_SPEC {
    type DataType = u32;
}
#[doc = "General PWM Timer Compare Capture Register D"]
pub type Gtccrd = crate::RegValueT<Gtccrd_SPEC>;

impl Gtccrd {
    #[doc = "Compare Capture Register D"]
    #[inline(always)]
    pub fn gtccrd(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Gtccrd_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Gtccrd_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gtccrd {
    #[inline(always)]
    fn default() -> Gtccrd {
        <crate::RegValueT<Gtccrd_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtccrf_SPEC;
impl crate::sealed::RegSpec for Gtccrf_SPEC {
    type DataType = u32;
}
#[doc = "General PWM Timer Compare Capture Register F"]
pub type Gtccrf = crate::RegValueT<Gtccrf_SPEC>;

impl Gtccrf {
    #[doc = "Compare Capture Register F"]
    #[inline(always)]
    pub fn gtccrf(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Gtccrf_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Gtccrf_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gtccrf {
    #[inline(always)]
    fn default() -> Gtccrf {
        <crate::RegValueT<Gtccrf_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtpr_SPEC;
impl crate::sealed::RegSpec for Gtpr_SPEC {
    type DataType = u32;
}
#[doc = "General PWM Timer Cycle Setting Register"]
pub type Gtpr = crate::RegValueT<Gtpr_SPEC>;

impl Gtpr {
    #[doc = "Cycle Setting Register"]
    #[inline(always)]
    pub fn gtpr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Gtpr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Gtpr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gtpr {
    #[inline(always)]
    fn default() -> Gtpr {
        <crate::RegValueT<Gtpr_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtpbr_SPEC;
impl crate::sealed::RegSpec for Gtpbr_SPEC {
    type DataType = u32;
}
#[doc = "General PWM Timer Cycle Setting Buffer Register"]
pub type Gtpbr = crate::RegValueT<Gtpbr_SPEC>;

impl Gtpbr {
    #[doc = "Cycle Setting Buffer Register"]
    #[inline(always)]
    pub fn gtpbr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Gtpbr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Gtpbr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gtpbr {
    #[inline(always)]
    fn default() -> Gtpbr {
        <crate::RegValueT<Gtpbr_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtdtcr_SPEC;
impl crate::sealed::RegSpec for Gtdtcr_SPEC {
    type DataType = u32;
}
#[doc = "General PWM Timer Dead Time Control Register"]
pub type Gtdtcr = crate::RegValueT<Gtdtcr_SPEC>;

impl Gtdtcr {
    #[doc = "Negative-Phase Waveform Setting"]
    #[inline(always)]
    pub fn tde(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, gtdtcr::Tde, Gtdtcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,gtdtcr::Tde, Gtdtcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gtdtcr {
    #[inline(always)]
    fn default() -> Gtdtcr {
        <crate::RegValueT<Gtdtcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtdtcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tde_SPEC;
    pub type Tde = crate::EnumBitfieldStruct<u8, Tde_SPEC>;
    impl Tde {
        #[doc = "Set GTCCRB without using GTDVU and GTDVD."]
        pub const _0: Self = Self::new(0);
        #[doc = "Use GTDVU and GTDVD to set the compare match value for negative-phase waveform with automatic dead time in GTCCRB."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtdvu_SPEC;
impl crate::sealed::RegSpec for Gtdvu_SPEC {
    type DataType = u32;
}
#[doc = "General PWM Timer Dead Time Value Register U"]
pub type Gtdvu = crate::RegValueT<Gtdvu_SPEC>;

impl Gtdvu {
    #[doc = "Dead Time Value Register U"]
    #[inline(always)]
    pub fn gtdvu(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Gtdvu_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Gtdvu_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Gtdvu {
    #[inline(always)]
    fn default() -> Gtdvu {
        <crate::RegValueT<Gtdvu_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
