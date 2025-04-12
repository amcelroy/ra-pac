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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:15:57 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"System Control"]
unsafe impl ::core::marker::Send for super::System {}
unsafe impl ::core::marker::Sync for super::System {}
impl super::System {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "VBATT Control Register1"]
    #[inline(always)]
    pub const fn vbtcr1(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtcr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtcr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1055usize),
            )
        }
    }

    #[doc = "VBATT Control Register2"]
    #[inline(always)]
    pub const fn vbtcr2(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtcr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtcr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1200usize),
            )
        }
    }

    #[doc = "VBATT Status Register"]
    #[inline(always)]
    pub const fn vbtsr(&self) -> &'static crate::common::Reg<self::Vbtsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1201usize),
            )
        }
    }

    #[doc = "VBATT Comparator Control Register"]
    #[inline(always)]
    pub const fn vbtcmpcr(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtcmpcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtcmpcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1202usize),
            )
        }
    }

    #[doc = "VBATT Pin Low Voltage Detect Interrupt Control Register"]
    #[inline(always)]
    pub const fn vbtlvdicr(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtlvdicr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtlvdicr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1204usize),
            )
        }
    }

    #[doc = "VBATT Wakeup function Control Register"]
    #[inline(always)]
    pub const fn vbtwctlr(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtwctlr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtwctlr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1206usize),
            )
        }
    }

    #[doc = "VBATT Wakeup I/O 0 Output Trigger Select Register"]
    #[inline(always)]
    pub const fn vbtwch0otsr(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtwch0Otsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtwch0Otsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1208usize),
            )
        }
    }

    #[doc = "VBATT Wakeup I/O 1 Output Trigger Select Register"]
    #[inline(always)]
    pub const fn vbtwch1otsr(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtwch1Otsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtwch1Otsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1209usize),
            )
        }
    }

    #[doc = "VBATT Wakeup I/O 2 Output Trigger Select Register"]
    #[inline(always)]
    pub const fn vbtwch2otsr(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtwch2Otsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtwch2Otsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1210usize),
            )
        }
    }

    #[doc = "VBATT Input Control Register"]
    #[inline(always)]
    pub const fn vbtictlr(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtictlr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtictlr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1211usize),
            )
        }
    }

    #[doc = "VBATT Output Control Register"]
    #[inline(always)]
    pub const fn vbtoctlr(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtoctlr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtoctlr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1212usize),
            )
        }
    }

    #[doc = "VBATT Wakeup Trigger source Enable Register"]
    #[inline(always)]
    pub const fn vbtwter(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtwter_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtwter_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1213usize),
            )
        }
    }

    #[doc = "VBATT Wakeup Trigger source Edge Register"]
    #[inline(always)]
    pub const fn vbtwegr(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtwegr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtwegr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1214usize),
            )
        }
    }

    #[doc = "VBATT Wakeup trigger source Flag Register"]
    #[inline(always)]
    pub const fn vbtwfr(
        &self,
    ) -> &'static crate::common::Reg<self::Vbtwfr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Vbtwfr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1215usize),
            )
        }
    }

    #[doc = "VBATT Backup Register \\[%s\\]"]
    #[inline(always)]
    pub const fn vbtbkr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Vbtbkr_SPEC, crate::common::RW>,
        512,
        0x1,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x500usize))
        }
    }

    #[doc = "System Clock Division Control Register"]
    #[inline(always)]
    pub const fn sckdivcr(
        &self,
    ) -> &'static crate::common::Reg<self::Sckdivcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sckdivcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "System Clock Source Control Register"]
    #[inline(always)]
    pub const fn sckscr(
        &self,
    ) -> &'static crate::common::Reg<self::Sckscr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sckscr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(38usize),
            )
        }
    }

    #[doc = "PLL Control Register"]
    #[inline(always)]
    pub const fn pllcr(&self) -> &'static crate::common::Reg<self::Pllcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pllcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(42usize),
            )
        }
    }

    #[doc = "PLL Clock Control Register2"]
    #[inline(always)]
    pub const fn pllccr2(
        &self,
    ) -> &'static crate::common::Reg<self::Pllccr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Pllccr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(43usize),
            )
        }
    }

    #[doc = "Memory Wait Cycle Control Register"]
    #[inline(always)]
    pub const fn memwait(
        &self,
    ) -> &'static crate::common::Reg<self::Memwait_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Memwait_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(49usize),
            )
        }
    }

    #[doc = "Main Clock Oscillator Control Register"]
    #[inline(always)]
    pub const fn mosccr(
        &self,
    ) -> &'static crate::common::Reg<self::Mosccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mosccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(50usize),
            )
        }
    }

    #[doc = "High-Speed On-Chip Oscillator Control Register"]
    #[inline(always)]
    pub const fn hococr(
        &self,
    ) -> &'static crate::common::Reg<self::Hococr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Hococr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(54usize),
            )
        }
    }

    #[doc = "Middle-Speed On-Chip Oscillator Control Register"]
    #[inline(always)]
    pub const fn mococr(
        &self,
    ) -> &'static crate::common::Reg<self::Mococr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mococr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "Oscillation Stabilization Flag Register"]
    #[inline(always)]
    pub const fn oscsf(&self) -> &'static crate::common::Reg<self::Oscsf_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Oscsf_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[doc = "Clock Out Control Register"]
    #[inline(always)]
    pub const fn ckocr(&self) -> &'static crate::common::Reg<self::Ckocr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ckocr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(62usize),
            )
        }
    }

    #[doc = "Trace Clock Control Register"]
    #[inline(always)]
    pub const fn trckcr(
        &self,
    ) -> &'static crate::common::Reg<self::Trckcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Trckcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(63usize),
            )
        }
    }

    #[doc = "Oscillation Stop Detection Control Register"]
    #[inline(always)]
    pub const fn ostdcr(
        &self,
    ) -> &'static crate::common::Reg<self::Ostdcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ostdcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "Oscillation Stop Detection Status Register"]
    #[inline(always)]
    pub const fn ostdsr(
        &self,
    ) -> &'static crate::common::Reg<self::Ostdsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ostdsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(65usize),
            )
        }
    }

    #[doc = "Segment LCD Source Clock Control Register"]
    #[inline(always)]
    pub const fn slcdsckcr(
        &self,
    ) -> &'static crate::common::Reg<self::Slcdsckcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Slcdsckcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[doc = "MOCO User Trimming Control Register"]
    #[inline(always)]
    pub const fn mocoutcr(
        &self,
    ) -> &'static crate::common::Reg<self::Mocoutcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mocoutcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(97usize),
            )
        }
    }

    #[doc = "HOCO User Trimming Control Register"]
    #[inline(always)]
    pub const fn hocoutcr(
        &self,
    ) -> &'static crate::common::Reg<self::Hocoutcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Hocoutcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(98usize),
            )
        }
    }

    #[doc = "Main Clock Oscillator Wait Control Register"]
    #[inline(always)]
    pub const fn moscwtcr(
        &self,
    ) -> &'static crate::common::Reg<self::Moscwtcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Moscwtcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(162usize),
            )
        }
    }

    #[doc = "High-Speed On-Chip Oscillator Wait Control Register"]
    #[inline(always)]
    pub const fn hocowtcr(
        &self,
    ) -> &'static crate::common::Reg<self::Hocowtcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Hocowtcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(165usize),
            )
        }
    }

    #[doc = "USB Clock Control register"]
    #[inline(always)]
    pub const fn usbckcr(
        &self,
    ) -> &'static crate::common::Reg<self::Usbckcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Usbckcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(208usize),
            )
        }
    }

    #[doc = "Main Clock Oscillator Mode Oscillation Control Register"]
    #[inline(always)]
    pub const fn momcr(&self) -> &'static crate::common::Reg<self::Momcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Momcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1043usize),
            )
        }
    }

    #[doc = "Sub-Clock Oscillator Control Register"]
    #[inline(always)]
    pub const fn sosccr(
        &self,
    ) -> &'static crate::common::Reg<self::Sosccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sosccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1152usize),
            )
        }
    }

    #[doc = "Sub Clock Oscillator Mode Control Register"]
    #[inline(always)]
    pub const fn somcr(&self) -> &'static crate::common::Reg<self::Somcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Somcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1153usize),
            )
        }
    }

    #[doc = "Low-Speed On-Chip Oscillator Control Register"]
    #[inline(always)]
    pub const fn lococr(
        &self,
    ) -> &'static crate::common::Reg<self::Lococr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lococr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1168usize),
            )
        }
    }

    #[doc = "LOCO User Trimming Control Register"]
    #[inline(always)]
    pub const fn locoutcr(
        &self,
    ) -> &'static crate::common::Reg<self::Locoutcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Locoutcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1170usize),
            )
        }
    }

    #[doc = "Standby Control Register"]
    #[inline(always)]
    pub const fn sbycr(&self) -> &'static crate::common::Reg<self::Sbycr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sbycr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Module Stop Control Register A"]
    #[inline(always)]
    pub const fn mstpcra(
        &self,
    ) -> &'static crate::common::Reg<self::Mstpcra_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mstpcra_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "Snooze Control Register"]
    #[inline(always)]
    pub const fn snzcr(&self) -> &'static crate::common::Reg<self::Snzcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Snzcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(146usize),
            )
        }
    }

    #[doc = "Snooze End Control Register"]
    #[inline(always)]
    pub const fn snzedcr(
        &self,
    ) -> &'static crate::common::Reg<self::Snzedcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Snzedcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }

    #[doc = "Snooze Request Control Register"]
    #[inline(always)]
    pub const fn snzreqcr(
        &self,
    ) -> &'static crate::common::Reg<self::Snzreqcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Snzreqcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(152usize),
            )
        }
    }

    #[doc = "Flash Operation Control Register"]
    #[inline(always)]
    pub const fn flstop(
        &self,
    ) -> &'static crate::common::Reg<self::Flstop_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Flstop_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(158usize),
            )
        }
    }

    #[doc = "Operating Power Control Register"]
    #[inline(always)]
    pub const fn opccr(&self) -> &'static crate::common::Reg<self::Opccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Opccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(160usize),
            )
        }
    }

    #[doc = "Sub Operating Power Control Register"]
    #[inline(always)]
    pub const fn sopccr(
        &self,
    ) -> &'static crate::common::Reg<self::Sopccr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Sopccr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(170usize),
            )
        }
    }

    #[doc = "System Control OCD Control Register"]
    #[inline(always)]
    pub const fn syocdcr(
        &self,
    ) -> &'static crate::common::Reg<self::Syocdcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Syocdcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1038usize),
            )
        }
    }

    #[doc = "Voltage Monitor Circuit Control Register"]
    #[inline(always)]
    pub const fn lvcmpcr(
        &self,
    ) -> &'static crate::common::Reg<self::Lvcmpcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lvcmpcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1047usize),
            )
        }
    }

    #[doc = "Voltage Detection Level Select Register"]
    #[inline(always)]
    pub const fn lvdlvlr(
        &self,
    ) -> &'static crate::common::Reg<self::Lvdlvlr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Lvdlvlr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1048usize),
            )
        }
    }

    #[doc = "Voltage Monitor %s Circuit Control Register 0"]
    #[inline(always)]
    pub const fn lvdcr0(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Lvdcr0_SPEC, crate::common::RW>,
        2,
        0x1,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x41ausize))
        }
    }

    #[doc = "Voltage Monitor %s Circuit Control Register 1"]
    #[inline(always)]
    pub const fn lvdcr1(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Lvdcr1_SPEC, crate::common::RW>,
        2,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xe0usize))
        }
    }

    #[doc = "Voltage Monitor %s Circuit Status Register"]
    #[inline(always)]
    pub const fn lvdsr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Lvdsr_SPEC, crate::common::RW>,
        2,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xe1usize))
        }
    }

    #[doc = "Protect Register"]
    #[inline(always)]
    pub const fn prcr(&self) -> &'static crate::common::Reg<self::Prcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Prcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1022usize),
            )
        }
    }

    #[doc = "Reset Status Register 0"]
    #[inline(always)]
    pub const fn rstsr0(
        &self,
    ) -> &'static crate::common::Reg<self::Rstsr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rstsr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1040usize),
            )
        }
    }

    #[doc = "Reset Status Register 2"]
    #[inline(always)]
    pub const fn rstsr2(
        &self,
    ) -> &'static crate::common::Reg<self::Rstsr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rstsr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1041usize),
            )
        }
    }

    #[doc = "Reset Status Register 1"]
    #[inline(always)]
    pub const fn rstsr1(
        &self,
    ) -> &'static crate::common::Reg<self::Rstsr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Rstsr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(192usize),
            )
        }
    }

    #[doc = "Backup Register Access Control Register"]
    #[inline(always)]
    pub const fn bkracr(
        &self,
    ) -> &'static crate::common::Reg<self::Bkracr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Bkracr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(198usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtcr1_SPEC;
impl crate::sealed::RegSpec for Vbtcr1_SPEC {
    type DataType = u8;
}
#[doc = "VBATT Control Register1"]
pub type Vbtcr1 = crate::RegValueT<Vbtcr1_SPEC>;

impl Vbtcr1 {
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, Vbtcr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8, Vbtcr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Battery Power supply Switch Stop"]
    #[inline(always)]
    pub fn bpwswstp(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, vbtcr1::Bpwswstp, Vbtcr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,vbtcr1::Bpwswstp, Vbtcr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Vbtcr1 {
    #[inline(always)]
    fn default() -> Vbtcr1 {
        <crate::RegValueT<Vbtcr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod vbtcr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bpwswstp_SPEC;
    pub type Bpwswstp = crate::EnumBitfieldStruct<u8, Bpwswstp_SPEC>;
    impl Bpwswstp {
        #[doc = "Battery Power supply Switch Enable"]
        pub const _0: Self = Self::new(0);
        #[doc = "Battery Power supply Switch stop"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtcr2_SPEC;
impl crate::sealed::RegSpec for Vbtcr2_SPEC {
    type DataType = u8;
}
#[doc = "VBATT Control Register2"]
pub type Vbtcr2 = crate::RegValueT<Vbtcr2_SPEC>;

impl Vbtcr2 {
    #[doc = "VBATT Pin Voltage Low Voltage Detect Level Select Bit"]
    #[inline(always)]
    pub fn vbtlvdlvl(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, vbtcr2::Vbtlvdlvl, Vbtcr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            vbtcr2::Vbtlvdlvl,
            Vbtcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "VBATT Pin Low Voltage Detect Enable Bit"]
    #[inline(always)]
    pub fn vbtlvden(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, vbtcr2::Vbtlvden, Vbtcr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,vbtcr2::Vbtlvden, Vbtcr2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Vbtcr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Vbtcr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Vbtcr2 {
    #[inline(always)]
    fn default() -> Vbtcr2 {
        <crate::RegValueT<Vbtcr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod vbtcr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbtlvdlvl_SPEC;
    pub type Vbtlvdlvl = crate::EnumBitfieldStruct<u8, Vbtlvdlvl_SPEC>;
    impl Vbtlvdlvl {
        #[doc = "2.7V"]
        pub const _00: Self = Self::new(0);
        #[doc = "Setting prohibited"]
        pub const _01: Self = Self::new(1);
        #[doc = "2.3V"]
        pub const _10: Self = Self::new(2);
        #[doc = "2.1V"]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbtlvden_SPEC;
    pub type Vbtlvden = crate::EnumBitfieldStruct<u8, Vbtlvden_SPEC>;
    impl Vbtlvden {
        #[doc = "VBATT pin low voltage detect disable"]
        pub const _0: Self = Self::new(0);
        #[doc = "VBATT pin low voltage detect enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtsr_SPEC;
impl crate::sealed::RegSpec for Vbtsr_SPEC {
    type DataType = u8;
}
#[doc = "VBATT Status Register"]
pub type Vbtsr = crate::RegValueT<Vbtsr_SPEC>;

impl Vbtsr {
    #[doc = "VBATT_R Valid"]
    #[inline(always)]
    pub fn vbtrvld(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, vbtsr::Vbtrvld, Vbtsr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0x1,1,0,vbtsr::Vbtrvld, Vbtsr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, Vbtsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8, Vbtsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VBATT Battery Low voltage Detect Flag"]
    #[inline(always)]
    pub fn vbtbldf(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, vbtsr::Vbtbldf, Vbtsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,vbtsr::Vbtbldf, Vbtsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VBAT_R Reset Detect Flag"]
    #[inline(always)]
    pub fn vbtrdf(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, vbtsr::Vbtrdf, Vbtsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,vbtsr::Vbtrdf, Vbtsr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Vbtsr {
    #[inline(always)]
    fn default() -> Vbtsr {
        <crate::RegValueT<Vbtsr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod vbtsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbtrvld_SPEC;
    pub type Vbtrvld = crate::EnumBitfieldStruct<u8, Vbtrvld_SPEC>;
    impl Vbtrvld {
        #[doc = "VBATT_R area not valid"]
        pub const _0: Self = Self::new(0);
        #[doc = "VBATT_R area valid"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbtbldf_SPEC;
    pub type Vbtbldf = crate::EnumBitfieldStruct<u8, Vbtbldf_SPEC>;
    impl Vbtbldf {
        #[doc = "VBATT pin low voltage not detected"]
        pub const _0: Self = Self::new(0);
        #[doc = "VBATT pin low voltage detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbtrdf_SPEC;
    pub type Vbtrdf = crate::EnumBitfieldStruct<u8, Vbtrdf_SPEC>;
    impl Vbtrdf {
        #[doc = "VBATT_R voltage power-on reset not detected"]
        pub const _0: Self = Self::new(0);
        #[doc = "VBATT_R selected voltage power-on reset detected."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtcmpcr_SPEC;
impl crate::sealed::RegSpec for Vbtcmpcr_SPEC {
    type DataType = u8;
}
#[doc = "VBATT Comparator Control Register"]
pub type Vbtcmpcr = crate::RegValueT<Vbtcmpcr_SPEC>;

impl Vbtcmpcr {
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, Vbtcmpcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8, Vbtcmpcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VBATT pin low voltage detect circuit output enable"]
    #[inline(always)]
    pub fn vbtcmpe(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        vbtcmpcr::Vbtcmpe,
        Vbtcmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            vbtcmpcr::Vbtcmpe,
            Vbtcmpcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Vbtcmpcr {
    #[inline(always)]
    fn default() -> Vbtcmpcr {
        <crate::RegValueT<Vbtcmpcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod vbtcmpcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbtcmpe_SPEC;
    pub type Vbtcmpe = crate::EnumBitfieldStruct<u8, Vbtcmpe_SPEC>;
    impl Vbtcmpe {
        #[doc = "VBATT pin low voltage detect circuit output disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "VBATT pin low voltage detect circuit output enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtlvdicr_SPEC;
impl crate::sealed::RegSpec for Vbtlvdicr_SPEC {
    type DataType = u8;
}
#[doc = "VBATT Pin Low Voltage Detect Interrupt Control Register"]
pub type Vbtlvdicr = crate::RegValueT<Vbtlvdicr_SPEC>;

impl Vbtlvdicr {
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, Vbtlvdicr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3f,1,0,u8, Vbtlvdicr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pin Low Voltage Detect Interrupt Select bit"]
    #[inline(always)]
    pub fn vbtlvdisel(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        vbtlvdicr::Vbtlvdisel,
        Vbtlvdicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            vbtlvdicr::Vbtlvdisel,
            Vbtlvdicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "VBATT Pin Low Voltage Detect Interrupt Enable bit"]
    #[inline(always)]
    pub fn vbtlvdie(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        vbtlvdicr::Vbtlvdie,
        Vbtlvdicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            vbtlvdicr::Vbtlvdie,
            Vbtlvdicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Vbtlvdicr {
    #[inline(always)]
    fn default() -> Vbtlvdicr {
        <crate::RegValueT<Vbtlvdicr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod vbtlvdicr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbtlvdisel_SPEC;
    pub type Vbtlvdisel = crate::EnumBitfieldStruct<u8, Vbtlvdisel_SPEC>;
    impl Vbtlvdisel {
        #[doc = "Non Maskable Interrupt"]
        pub const _0: Self = Self::new(0);
        #[doc = "Maskable Interrupt"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vbtlvdie_SPEC;
    pub type Vbtlvdie = crate::EnumBitfieldStruct<u8, Vbtlvdie_SPEC>;
    impl Vbtlvdie {
        #[doc = "VBATT Pin Low Voltage Detect Interrupt Disable"]
        pub const _0: Self = Self::new(0);
        #[doc = "VBATT Pin Low Voltage Detect Interrupt Enable"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtwctlr_SPEC;
impl crate::sealed::RegSpec for Vbtwctlr_SPEC {
    type DataType = u8;
}
#[doc = "VBATT Wakeup function Control Register"]
pub type Vbtwctlr = crate::RegValueT<Vbtwctlr_SPEC>;

impl Vbtwctlr {
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, Vbtwctlr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8, Vbtwctlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VBATT wakeup enable"]
    #[inline(always)]
    pub fn vwen(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, vbtwctlr::Vwen, Vbtwctlr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,vbtwctlr::Vwen, Vbtwctlr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Vbtwctlr {
    #[inline(always)]
    fn default() -> Vbtwctlr {
        <crate::RegValueT<Vbtwctlr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod vbtwctlr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vwen_SPEC;
    pub type Vwen = crate::EnumBitfieldStruct<u8, Vwen_SPEC>;
    impl Vwen {
        #[doc = "Disable Wakeup function"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable Wakeup function"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtwch0Otsr_SPEC;
impl crate::sealed::RegSpec for Vbtwch0Otsr_SPEC {
    type DataType = u8;
}
#[doc = "VBATT Wakeup I/O 0 Output Trigger Select Register"]
pub type Vbtwch0Otsr = crate::RegValueT<Vbtwch0Otsr_SPEC>;

impl Vbtwch0Otsr {
    #[doc = "VBATWIO0 Output RTC Alarm Signal Enable"]
    #[inline(always)]
    pub fn ch0vrtcate(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        vbtwch0otsr::Ch0Vrtcate,
        Vbtwch0Otsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            vbtwch0otsr::Ch0Vrtcate,
            Vbtwch0Otsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "VBATWIO0 Output RTC Periodic Signal Enable"]
    #[inline(always)]
    pub fn ch0vrtcte(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        vbtwch0otsr::Ch0Vrtcte,
        Vbtwch0Otsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            vbtwch0otsr::Ch0Vrtcte,
            Vbtwch0Otsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "VBATWIO0 Output VBATWIO2 Trigger Enable"]
    #[inline(always)]
    pub fn ch0vch2te(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        vbtwch0otsr::Ch0Vch2Te,
        Vbtwch0Otsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            vbtwch0otsr::Ch0Vch2Te,
            Vbtwch0Otsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "VBATWIO0 Output VBATWIO1 Trigger Enable"]
    #[inline(always)]
    pub fn ch0vch1te(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        vbtwch0otsr::Ch0Vch1Te,
        Vbtwch0Otsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            vbtwch0otsr::Ch0Vch1Te,
            Vbtwch0Otsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Vbtwch0Otsr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Vbtwch0Otsr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Vbtwch0Otsr {
    #[inline(always)]
    fn default() -> Vbtwch0Otsr {
        <crate::RegValueT<Vbtwch0Otsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod vbtwch0otsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ch0Vrtcate_SPEC;
    pub type Ch0Vrtcate = crate::EnumBitfieldStruct<u8, Ch0Vrtcate_SPEC>;
    impl Ch0Vrtcate {
        #[doc = "VBATT wakeup I/O 0 output trigger by the RTC alarm signal is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "VBATT wakeup I/O 0 output trigger by the RTC alarm signal is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ch0Vrtcte_SPEC;
    pub type Ch0Vrtcte = crate::EnumBitfieldStruct<u8, Ch0Vrtcte_SPEC>;
    impl Ch0Vrtcte {
        #[doc = "VBATT wakeup I/O 0 output trigger by the RTC periodic signal is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "VBATT wakeup I/O 0 output trigger by the RTC periodic signal is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ch0Vch2Te_SPEC;
    pub type Ch0Vch2Te = crate::EnumBitfieldStruct<u8, Ch0Vch2Te_SPEC>;
    impl Ch0Vch2Te {
        #[doc = "VBATT wakeup I/O 0 output trigger by the VBATWIO2 pin is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "VBATT wakeup I/O 0 output trigger by the VBATWIO2 pin is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ch0Vch1Te_SPEC;
    pub type Ch0Vch1Te = crate::EnumBitfieldStruct<u8, Ch0Vch1Te_SPEC>;
    impl Ch0Vch1Te {
        #[doc = "VBATT wakeup I/O 0 output trigger by the VBATWIO1 pin is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "VBATT wakeup I/O 0 output trigger by the VBATWIO1 pin is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtwch1Otsr_SPEC;
impl crate::sealed::RegSpec for Vbtwch1Otsr_SPEC {
    type DataType = u8;
}
#[doc = "VBATT Wakeup I/O 1 Output Trigger Select Register"]
pub type Vbtwch1Otsr = crate::RegValueT<Vbtwch1Otsr_SPEC>;

impl Vbtwch1Otsr {
    #[doc = "VBATWIO1 Output RTC Alarm Signal Enable"]
    #[inline(always)]
    pub fn ch1vrtcate(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        vbtwch1otsr::Ch1Vrtcate,
        Vbtwch1Otsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            vbtwch1otsr::Ch1Vrtcate,
            Vbtwch1Otsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "VBATWIO1 Output RTC Periodic Signal Enable"]
    #[inline(always)]
    pub fn ch1vrtcte(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        vbtwch1otsr::Ch1Vrtcte,
        Vbtwch1Otsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            vbtwch1otsr::Ch1Vrtcte,
            Vbtwch1Otsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "VBATWIO1 Output VBATWIO2 Trigger Enable"]
    #[inline(always)]
    pub fn ch1vch2te(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        vbtwch1otsr::Ch1Vch2Te,
        Vbtwch1Otsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            vbtwch1otsr::Ch1Vch2Te,
            Vbtwch1Otsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Vbtwch1Otsr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Vbtwch1Otsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VBATWIO1 Output VBATWIO0 Trigger Enable"]
    #[inline(always)]
    pub fn ch1vch0te(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        vbtwch1otsr::Ch1Vch0Te,
        Vbtwch1Otsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            vbtwch1otsr::Ch1Vch0Te,
            Vbtwch1Otsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Vbtwch1Otsr {
    #[inline(always)]
    fn default() -> Vbtwch1Otsr {
        <crate::RegValueT<Vbtwch1Otsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod vbtwch1otsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ch1Vrtcate_SPEC;
    pub type Ch1Vrtcate = crate::EnumBitfieldStruct<u8, Ch1Vrtcate_SPEC>;
    impl Ch1Vrtcate {
        #[doc = "VBATT wakeup I/O 1 output trigger by the RTC alarm signal is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "VBATT wakeup I/O 1 output trigger by the RTC alarm signal is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ch1Vrtcte_SPEC;
    pub type Ch1Vrtcte = crate::EnumBitfieldStruct<u8, Ch1Vrtcte_SPEC>;
    impl Ch1Vrtcte {
        #[doc = "VBATT wakeup I/O 1 output trigger by the RTC periodic signal is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "VBATT wakeup I/O 1 output trigger by the RTC periodic signal is enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ch1Vch2Te_SPEC;
    pub type Ch1Vch2Te = crate::EnumBitfieldStruct<u8, Ch1Vch2Te_SPEC>;
    impl Ch1Vch2Te {
        #[doc = "VBATT wakeup I/O 1 output trigger by the VBATWIO2 pin is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "VBATT wakeup I/O 1 output trigger by the VBATWIO2 pin is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ch1Vch0Te_SPEC;
    pub type Ch1Vch0Te = crate::EnumBitfieldStruct<u8, Ch1Vch0Te_SPEC>;
    impl Ch1Vch0Te {
        #[doc = "VBATT wakeup I/O 1 output trigger by the VBATWIO0 pin is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "VBATT wakeup I/O 1 output trigger by the VBATWIO0 pin is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtwch2Otsr_SPEC;
impl crate::sealed::RegSpec for Vbtwch2Otsr_SPEC {
    type DataType = u8;
}
#[doc = "VBATT Wakeup I/O 2 Output Trigger Select Register"]
pub type Vbtwch2Otsr = crate::RegValueT<Vbtwch2Otsr_SPEC>;

impl Vbtwch2Otsr {
    #[doc = "VBATWIO2 Output RTC Alarm Signal Enable"]
    #[inline(always)]
    pub fn ch2vrtcate(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        vbtwch2otsr::Ch2Vrtcate,
        Vbtwch2Otsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            vbtwch2otsr::Ch2Vrtcate,
            Vbtwch2Otsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "VBATWIO2 Output RTC Periodic Signal Enable"]
    #[inline(always)]
    pub fn ch2vrtcte(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        vbtwch2otsr::Ch2Vrtcte,
        Vbtwch2Otsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            vbtwch2otsr::Ch2Vrtcte,
            Vbtwch2Otsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Vbtwch2Otsr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Vbtwch2Otsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VBATWIO2 Output VBATWIO1 Trigger Enable"]
    #[inline(always)]
    pub fn ch2vch1te(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        vbtwch2otsr::Ch2Vch1Te,
        Vbtwch2Otsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            vbtwch2otsr::Ch2Vch1Te,
            Vbtwch2Otsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "VBATWIO2 Output VBATWIO0 Trigger Enable"]
    #[inline(always)]
    pub fn ch2vch0te(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        vbtwch2otsr::Ch2Vch0Te,
        Vbtwch2Otsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            vbtwch2otsr::Ch2Vch0Te,
            Vbtwch2Otsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Vbtwch2Otsr {
    #[inline(always)]
    fn default() -> Vbtwch2Otsr {
        <crate::RegValueT<Vbtwch2Otsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod vbtwch2otsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ch2Vrtcate_SPEC;
    pub type Ch2Vrtcate = crate::EnumBitfieldStruct<u8, Ch2Vrtcate_SPEC>;
    impl Ch2Vrtcate {
        #[doc = "VBATT wakeup I/O 2 output trigger by the RTC alarm signal is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "VBATT wakeup I/O 2 output trigger by the RTC alarm signal is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ch2Vrtcte_SPEC;
    pub type Ch2Vrtcte = crate::EnumBitfieldStruct<u8, Ch2Vrtcte_SPEC>;
    impl Ch2Vrtcte {
        #[doc = "VBATT wakeup I/O 2 output trigger by the RTC periodic signal is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "VBATT wakeup I/O 2 output trigger by the RTC periodic signal is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ch2Vch1Te_SPEC;
    pub type Ch2Vch1Te = crate::EnumBitfieldStruct<u8, Ch2Vch1Te_SPEC>;
    impl Ch2Vch1Te {
        #[doc = "VBATT wakeup I/O 2 output trigger by the VBATWIO1 pin is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "VBATT wakeup I/O 2 output trigger by the VBATWIO1 pin is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ch2Vch0Te_SPEC;
    pub type Ch2Vch0Te = crate::EnumBitfieldStruct<u8, Ch2Vch0Te_SPEC>;
    impl Ch2Vch0Te {
        #[doc = "VBATT wakeup I/O 2 output trigger by the VBATWIO0 pin is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "VBATT wakeup I/O 2 output trigger by the VBATWIO0 pin is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtictlr_SPEC;
impl crate::sealed::RegSpec for Vbtictlr_SPEC {
    type DataType = u8;
}
#[doc = "VBATT Input Control Register"]
pub type Vbtictlr = crate::RegValueT<Vbtictlr_SPEC>;

impl Vbtictlr {
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, Vbtictlr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8, Vbtictlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VBATT Wakeup I/O 2 Input Enable"]
    #[inline(always)]
    pub fn vch2inen(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        vbtictlr::Vch2Inen,
        Vbtictlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            vbtictlr::Vch2Inen,
            Vbtictlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "VBATT Wakeup I/O 1 Input Enable"]
    #[inline(always)]
    pub fn vch1inen(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        vbtictlr::Vch1Inen,
        Vbtictlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            vbtictlr::Vch1Inen,
            Vbtictlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "VBATT Wakeup I/O 0 Input Enable"]
    #[inline(always)]
    pub fn vch0inen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        vbtictlr::Vch0Inen,
        Vbtictlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            vbtictlr::Vch0Inen,
            Vbtictlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Vbtictlr {
    #[inline(always)]
    fn default() -> Vbtictlr {
        <crate::RegValueT<Vbtictlr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod vbtictlr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vch2Inen_SPEC;
    pub type Vch2Inen = crate::EnumBitfieldStruct<u8, Vch2Inen_SPEC>;
    impl Vch2Inen {
        #[doc = "VBATWIO2 and RTCIC2 inputs disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "VBATWIO2 and RTCIC2 inputs enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vch1Inen_SPEC;
    pub type Vch1Inen = crate::EnumBitfieldStruct<u8, Vch1Inen_SPEC>;
    impl Vch1Inen {
        #[doc = "VBATWIO1, RTCIC1 inputs disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "VBATWIO1, RTCIC1 inputs enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vch0Inen_SPEC;
    pub type Vch0Inen = crate::EnumBitfieldStruct<u8, Vch0Inen_SPEC>;
    impl Vch0Inen {
        #[doc = "VBATWIO0, RTCIC0 inputs disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "VBATWIO0, RTCIC0 inputs enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtoctlr_SPEC;
impl crate::sealed::RegSpec for Vbtoctlr_SPEC {
    type DataType = u8;
}
#[doc = "VBATT Output Control Register"]
pub type Vbtoctlr = crate::RegValueT<Vbtoctlr_SPEC>;

impl Vbtoctlr {
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, Vbtoctlr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8, Vbtoctlr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VBATT Wakeup I/O 2 Output Level Selection"]
    #[inline(always)]
    pub fn vout2lsel(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        vbtoctlr::Vout2Lsel,
        Vbtoctlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            vbtoctlr::Vout2Lsel,
            Vbtoctlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "VBATT Wakeup I/O 1 Output Level Selection"]
    #[inline(always)]
    pub fn vout1lsel(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        vbtoctlr::Vout1Lsel,
        Vbtoctlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            vbtoctlr::Vout1Lsel,
            Vbtoctlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "VBATT Wakeup I/O 0 Output Level Selection"]
    #[inline(always)]
    pub fn vout0lsel(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        vbtoctlr::Vout0Lsel,
        Vbtoctlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            vbtoctlr::Vout0Lsel,
            Vbtoctlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "VBATT Wakeup I/O 2 Output Enable"]
    #[inline(always)]
    pub fn vch2oen(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        vbtoctlr::Vch2Oen,
        Vbtoctlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            vbtoctlr::Vch2Oen,
            Vbtoctlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "VBATT Wakeup I/O 1 Output Enable"]
    #[inline(always)]
    pub fn vch1oen(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        vbtoctlr::Vch1Oen,
        Vbtoctlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            vbtoctlr::Vch1Oen,
            Vbtoctlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "VBATT Wakeup I/O 0 Output Enable"]
    #[inline(always)]
    pub fn vch0oen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        vbtoctlr::Vch0Oen,
        Vbtoctlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            vbtoctlr::Vch0Oen,
            Vbtoctlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Vbtoctlr {
    #[inline(always)]
    fn default() -> Vbtoctlr {
        <crate::RegValueT<Vbtoctlr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod vbtoctlr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vout2Lsel_SPEC;
    pub type Vout2Lsel = crate::EnumBitfieldStruct<u8, Vout2Lsel_SPEC>;
    impl Vout2Lsel {
        #[doc = "Output L before VBATT wake up trigger"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output H before VBATT wake up trigger"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vout1Lsel_SPEC;
    pub type Vout1Lsel = crate::EnumBitfieldStruct<u8, Vout1Lsel_SPEC>;
    impl Vout1Lsel {
        #[doc = "Output L before VBATT wake up trigger"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output H before VBATT wake up trigger"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vout0Lsel_SPEC;
    pub type Vout0Lsel = crate::EnumBitfieldStruct<u8, Vout0Lsel_SPEC>;
    impl Vout0Lsel {
        #[doc = "Output L before VBATT wakeup trigger"]
        pub const _0: Self = Self::new(0);
        #[doc = "Output H before VBATT wakeup trigger"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vch2Oen_SPEC;
    pub type Vch2Oen = crate::EnumBitfieldStruct<u8, Vch2Oen_SPEC>;
    impl Vch2Oen {
        #[doc = "VBATWIO2 output disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "VBATWIO2 output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vch1Oen_SPEC;
    pub type Vch1Oen = crate::EnumBitfieldStruct<u8, Vch1Oen_SPEC>;
    impl Vch1Oen {
        #[doc = "VBATWIO1 output disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "VBATWIO1 output enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vch0Oen_SPEC;
    pub type Vch0Oen = crate::EnumBitfieldStruct<u8, Vch0Oen_SPEC>;
    impl Vch0Oen {
        #[doc = "VBATWIO0 output disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "VBATWIO0 output enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtwter_SPEC;
impl crate::sealed::RegSpec for Vbtwter_SPEC {
    type DataType = u8;
}
#[doc = "VBATT Wakeup Trigger source Enable Register"]
pub type Vbtwter = crate::RegValueT<Vbtwter_SPEC>;

impl Vbtwter {
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, Vbtwter_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8, Vbtwter_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "RTC Alarm Signal Enable"]
    #[inline(always)]
    pub fn vrtcae(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, vbtwter::Vrtcae, Vbtwter_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,vbtwter::Vrtcae, Vbtwter_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "RTC Periodic Signal Enable"]
    #[inline(always)]
    pub fn vrtcie(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, vbtwter::Vrtcie, Vbtwter_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,vbtwter::Vrtcie, Vbtwter_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VBATWIO2 Pin Enable"]
    #[inline(always)]
    pub fn vch2e(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, vbtwter::Vch2E, Vbtwter_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,vbtwter::Vch2E, Vbtwter_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VBATWIO1 Pin Enable"]
    #[inline(always)]
    pub fn vch1e(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, vbtwter::Vch1E, Vbtwter_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,vbtwter::Vch1E, Vbtwter_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VBATWIO0 Pin Enable"]
    #[inline(always)]
    pub fn vch0e(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, vbtwter::Vch0E, Vbtwter_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,vbtwter::Vch0E, Vbtwter_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Vbtwter {
    #[inline(always)]
    fn default() -> Vbtwter {
        <crate::RegValueT<Vbtwter_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod vbtwter {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vrtcae_SPEC;
    pub type Vrtcae = crate::EnumBitfieldStruct<u8, Vrtcae_SPEC>;
    impl Vrtcae {
        #[doc = "VBATT wakeup triggered by RTC alarm signal is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "VBATT wakeup triggered by RTC alarm signal is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vrtcie_SPEC;
    pub type Vrtcie = crate::EnumBitfieldStruct<u8, Vrtcie_SPEC>;
    impl Vrtcie {
        #[doc = "VBATT wakeup triggered by RTC periodic signal is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "VBATT wakeup triggered by RTC periodic signal is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vch2E_SPEC;
    pub type Vch2E = crate::EnumBitfieldStruct<u8, Vch2E_SPEC>;
    impl Vch2E {
        #[doc = "VBATT wakeup triggered by the VBATWIO2 pin is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "VBATT wakeup triggered by the VBATWIO2 pin is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vch1E_SPEC;
    pub type Vch1E = crate::EnumBitfieldStruct<u8, Vch1E_SPEC>;
    impl Vch1E {
        #[doc = "VBATT wakeup triggered by the VBATWIO1 pin is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "VBATT wakeup triggered by the VBATWIO1 pin is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vch0E_SPEC;
    pub type Vch0E = crate::EnumBitfieldStruct<u8, Vch0E_SPEC>;
    impl Vch0E {
        #[doc = "VBATT wakeup triggered by the VBATWIO0 pin is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "VBATT wakeup triggered by the VBATWIO0 pin is enabled."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtwegr_SPEC;
impl crate::sealed::RegSpec for Vbtwegr_SPEC {
    type DataType = u8;
}
#[doc = "VBATT Wakeup Trigger source Edge Register"]
pub type Vbtwegr = crate::RegValueT<Vbtwegr_SPEC>;

impl Vbtwegr {
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, Vbtwegr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8, Vbtwegr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VBATWIO2 Wakeup Trigger Source Edge Select"]
    #[inline(always)]
    pub fn vch2eg(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, vbtwegr::Vch2Eg, Vbtwegr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,vbtwegr::Vch2Eg, Vbtwegr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VBATWIO1 Wakeup Trigger Source Edge Select"]
    #[inline(always)]
    pub fn vch1eg(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, vbtwegr::Vch1Eg, Vbtwegr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,vbtwegr::Vch1Eg, Vbtwegr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VBATWIO0 Wakeup Trigger Source Edge Select"]
    #[inline(always)]
    pub fn vch0eg(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, vbtwegr::Vch0Eg, Vbtwegr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,vbtwegr::Vch0Eg, Vbtwegr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Vbtwegr {
    #[inline(always)]
    fn default() -> Vbtwegr {
        <crate::RegValueT<Vbtwegr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod vbtwegr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vch2Eg_SPEC;
    pub type Vch2Eg = crate::EnumBitfieldStruct<u8, Vch2Eg_SPEC>;
    impl Vch2Eg {
        #[doc = "Wakeup trigger is generated at a falling edge"]
        pub const _0: Self = Self::new(0);
        #[doc = "Wakeup trigger is generated at a rising edge."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vch1Eg_SPEC;
    pub type Vch1Eg = crate::EnumBitfieldStruct<u8, Vch1Eg_SPEC>;
    impl Vch1Eg {
        #[doc = "Wakeup trigger is generated at a falling edge"]
        pub const _0: Self = Self::new(0);
        #[doc = "Wakeup trigger is generated at a rising edge."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vch0Eg_SPEC;
    pub type Vch0Eg = crate::EnumBitfieldStruct<u8, Vch0Eg_SPEC>;
    impl Vch0Eg {
        #[doc = "Wakeup trigger is generated at a falling edge"]
        pub const _0: Self = Self::new(0);
        #[doc = "Wakeup trigger is generated at a rising edge."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtwfr_SPEC;
impl crate::sealed::RegSpec for Vbtwfr_SPEC {
    type DataType = u8;
}
#[doc = "VBATT Wakeup trigger source Flag Register"]
pub type Vbtwfr = crate::RegValueT<Vbtwfr_SPEC>;

impl Vbtwfr {
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, Vbtwfr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8, Vbtwfr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VBATT RTC-Alarm Wakeup Trigger Flag"]
    #[inline(always)]
    pub fn vrtcaf(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, vbtwfr::Vrtcaf, Vbtwfr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,vbtwfr::Vrtcaf, Vbtwfr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VBATT RTC-Interval  Wakeup Trigger Flag"]
    #[inline(always)]
    pub fn vrtcif(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, vbtwfr::Vrtcif, Vbtwfr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,vbtwfr::Vrtcif, Vbtwfr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VBATWIO2 Wakeup Trigger Flag"]
    #[inline(always)]
    pub fn vch2f(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, vbtwfr::Vch2F, Vbtwfr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,vbtwfr::Vch2F, Vbtwfr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VBATWIO1 Wakeup Trigger Flag"]
    #[inline(always)]
    pub fn vch1f(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, vbtwfr::Vch1F, Vbtwfr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,vbtwfr::Vch1F, Vbtwfr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VBATWIO0 Wakeup Trigger Flag"]
    #[inline(always)]
    pub fn vch0f(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, vbtwfr::Vch0F, Vbtwfr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,vbtwfr::Vch0F, Vbtwfr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Vbtwfr {
    #[inline(always)]
    fn default() -> Vbtwfr {
        <crate::RegValueT<Vbtwfr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod vbtwfr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vrtcaf_SPEC;
    pub type Vrtcaf = crate::EnumBitfieldStruct<u8, Vrtcaf_SPEC>;
    impl Vrtcaf {
        #[doc = "No wakeup trigger by the RTC alarm is generated"]
        pub const _0: Self = Self::new(0);
        #[doc = "A wakeup trigger by the RTC alarm is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vrtcif_SPEC;
    pub type Vrtcif = crate::EnumBitfieldStruct<u8, Vrtcif_SPEC>;
    impl Vrtcif {
        #[doc = "No wakeup trigger by the RTC interval is generated"]
        pub const _0: Self = Self::new(0);
        #[doc = "A wakeup trigger by the RTC interval is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vch2F_SPEC;
    pub type Vch2F = crate::EnumBitfieldStruct<u8, Vch2F_SPEC>;
    impl Vch2F {
        #[doc = "No wakeup trigger by the VBATWIO2 pin is generated"]
        pub const _0: Self = Self::new(0);
        #[doc = "A wakeup trigger by the VBATWIO2 pin is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vch1F_SPEC;
    pub type Vch1F = crate::EnumBitfieldStruct<u8, Vch1F_SPEC>;
    impl Vch1F {
        #[doc = "No wakeup trigger by the VBATWIO1 pin is generated"]
        pub const _0: Self = Self::new(0);
        #[doc = "A wakeup trigger by the VBATWIO1 pin is generated"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Vch0F_SPEC;
    pub type Vch0F = crate::EnumBitfieldStruct<u8, Vch0F_SPEC>;
    impl Vch0F {
        #[doc = "No wakeup trigger by the VBATWIO0 pin is generated"]
        pub const _0: Self = Self::new(0);
        #[doc = "A wakeup trigger by the VBATWIO0 pin is generated"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vbtbkr_SPEC;
impl crate::sealed::RegSpec for Vbtbkr_SPEC {
    type DataType = u8;
}
#[doc = "VBATT Backup Register \\[%s\\]"]
pub type Vbtbkr = crate::RegValueT<Vbtbkr_SPEC>;

impl Vbtbkr {
    #[doc = "VBTBKR is a 512-byte readable/writable register to store data powered by VBATT.\nThe value of this register is retained even when VCC is not powered but VBATT is powered.\nVBTBKR is initialized by VBATT selected voltage power-on-reset."]
    #[inline(always)]
    pub fn vbtbkr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Vbtbkr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Vbtbkr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Vbtbkr {
    #[inline(always)]
    fn default() -> Vbtbkr {
        <crate::RegValueT<Vbtbkr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sckdivcr_SPEC;
impl crate::sealed::RegSpec for Sckdivcr_SPEC {
    type DataType = u32;
}
#[doc = "System Clock Division Control Register"]
pub type Sckdivcr = crate::RegValueT<Sckdivcr_SPEC>;

impl Sckdivcr {
    #[doc = "Flash IF Clock (FCLK) Select"]
    #[inline(always)]
    pub fn fck(
        self,
    ) -> crate::common::RegisterField<28, 0x7, 1, 0, sckdivcr::Fck, Sckdivcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x7,1,0,sckdivcr::Fck, Sckdivcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "System Clock (ICLK) Select"]
    #[inline(always)]
    pub fn ick(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, sckdivcr::Ick, Sckdivcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x7,1,0,sckdivcr::Ick, Sckdivcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Module Clock A (PCLKA) Select"]
    #[inline(always)]
    pub fn pcka(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, sckdivcr::Pcka, Sckdivcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            12,
            0x7,
            1,
            0,
            sckdivcr::Pcka,
            Sckdivcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Peripheral Module Clock B (PCLKB) Select"]
    #[inline(always)]
    pub fn pckb(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, sckdivcr::Pckb, Sckdivcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,sckdivcr::Pckb, Sckdivcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral Module Clock C (PCLKC) Select"]
    #[inline(always)]
    pub fn pckc(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, sckdivcr::Pckc, Sckdivcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x7,1,0,sckdivcr::Pckc, Sckdivcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Sckdivcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Sckdivcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Peripheral Module Clock D (PCLKD) Select"]
    #[inline(always)]
    pub fn pckd(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, sckdivcr::Pckd, Sckdivcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,sckdivcr::Pckd, Sckdivcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sckdivcr {
    #[inline(always)]
    fn default() -> Sckdivcr {
        <crate::RegValueT<Sckdivcr_SPEC> as RegisterValue<_>>::new(1141130308)
    }
}
pub mod sckdivcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fck_SPEC;
    pub type Fck = crate::EnumBitfieldStruct<u8, Fck_SPEC>;
    impl Fck {
        #[doc = "/1"]
        pub const _000: Self = Self::new(0);
        #[doc = "/2"]
        pub const _001: Self = Self::new(1);
        #[doc = "/4"]
        pub const _010: Self = Self::new(2);
        #[doc = "/8"]
        pub const _011: Self = Self::new(3);
        #[doc = "/16"]
        pub const _100: Self = Self::new(4);
        #[doc = "/32"]
        pub const _101: Self = Self::new(5);
        #[doc = "/64"]
        pub const _110: Self = Self::new(6);
        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ick_SPEC;
    pub type Ick = crate::EnumBitfieldStruct<u8, Ick_SPEC>;
    impl Ick {
        #[doc = "/1"]
        pub const _000: Self = Self::new(0);
        #[doc = "/2"]
        pub const _001: Self = Self::new(1);
        #[doc = "/4"]
        pub const _010: Self = Self::new(2);
        #[doc = "/8"]
        pub const _011: Self = Self::new(3);
        #[doc = "/16"]
        pub const _100: Self = Self::new(4);
        #[doc = "/32"]
        pub const _101: Self = Self::new(5);
        #[doc = "/64"]
        pub const _110: Self = Self::new(6);
        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pcka_SPEC;
    pub type Pcka = crate::EnumBitfieldStruct<u8, Pcka_SPEC>;
    impl Pcka {
        #[doc = "/1"]
        pub const _000: Self = Self::new(0);
        #[doc = "/2"]
        pub const _001: Self = Self::new(1);
        #[doc = "/4"]
        pub const _010: Self = Self::new(2);
        #[doc = "/8"]
        pub const _011: Self = Self::new(3);
        #[doc = "/16"]
        pub const _100: Self = Self::new(4);
        #[doc = "/32"]
        pub const _101: Self = Self::new(5);
        #[doc = "/64"]
        pub const _110: Self = Self::new(6);
        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pckb_SPEC;
    pub type Pckb = crate::EnumBitfieldStruct<u8, Pckb_SPEC>;
    impl Pckb {
        #[doc = "/1"]
        pub const _000: Self = Self::new(0);
        #[doc = "/2"]
        pub const _001: Self = Self::new(1);
        #[doc = "/4"]
        pub const _010: Self = Self::new(2);
        #[doc = "/8"]
        pub const _011: Self = Self::new(3);
        #[doc = "/16"]
        pub const _100: Self = Self::new(4);
        #[doc = "/32"]
        pub const _101: Self = Self::new(5);
        #[doc = "/64"]
        pub const _110: Self = Self::new(6);
        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pckc_SPEC;
    pub type Pckc = crate::EnumBitfieldStruct<u8, Pckc_SPEC>;
    impl Pckc {
        #[doc = "/1"]
        pub const _000: Self = Self::new(0);
        #[doc = "/2"]
        pub const _001: Self = Self::new(1);
        #[doc = "/4"]
        pub const _010: Self = Self::new(2);
        #[doc = "/8"]
        pub const _011: Self = Self::new(3);
        #[doc = "/16"]
        pub const _100: Self = Self::new(4);
        #[doc = "/32"]
        pub const _101: Self = Self::new(5);
        #[doc = "/64"]
        pub const _110: Self = Self::new(6);
        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pckd_SPEC;
    pub type Pckd = crate::EnumBitfieldStruct<u8, Pckd_SPEC>;
    impl Pckd {
        #[doc = "/1"]
        pub const _000: Self = Self::new(0);
        #[doc = "/2"]
        pub const _001: Self = Self::new(1);
        #[doc = "/4"]
        pub const _010: Self = Self::new(2);
        #[doc = "/8"]
        pub const _011: Self = Self::new(3);
        #[doc = "/16"]
        pub const _100: Self = Self::new(4);
        #[doc = "/32"]
        pub const _101: Self = Self::new(5);
        #[doc = "/64"]
        pub const _110: Self = Self::new(6);
        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sckscr_SPEC;
impl crate::sealed::RegSpec for Sckscr_SPEC {
    type DataType = u8;
}
#[doc = "System Clock Source Control Register"]
pub type Sckscr = crate::RegValueT<Sckscr_SPEC>;

impl Sckscr {
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, Sckscr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8, Sckscr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock Source Select\nSelecting the system clock source faster than 32MHz(system clock source > 32MHz ) is prohibit when SCKDIVCR.ICK\\[2:0\\] bits select the division-by-1 and MEMWAIT.MEMWAIT =0."]
    #[inline(always)]
    pub fn cksel(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, sckscr::Cksel, Sckscr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,sckscr::Cksel, Sckscr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sckscr {
    #[inline(always)]
    fn default() -> Sckscr {
        <crate::RegValueT<Sckscr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod sckscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cksel_SPEC;
    pub type Cksel = crate::EnumBitfieldStruct<u8, Cksel_SPEC>;
    impl Cksel {
        #[doc = "HOCO"]
        pub const _000: Self = Self::new(0);
        #[doc = "MOCO"]
        pub const _001: Self = Self::new(1);
        #[doc = "LOCO"]
        pub const _010: Self = Self::new(2);
        #[doc = "Main clock oscillator"]
        pub const _011: Self = Self::new(3);
        #[doc = "Sub-clock oscillator"]
        pub const _100: Self = Self::new(4);
        #[doc = "PLL"]
        pub const _101: Self = Self::new(5);
        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pllcr_SPEC;
impl crate::sealed::RegSpec for Pllcr_SPEC {
    type DataType = u8;
}
#[doc = "PLL Control Register"]
pub type Pllcr = crate::RegValueT<Pllcr_SPEC>;

impl Pllcr {
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, Pllcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8, Pllcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "PLL Stop Control"]
    #[inline(always)]
    pub fn pllstp(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, pllcr::Pllstp, Pllcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,pllcr::Pllstp, Pllcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pllcr {
    #[inline(always)]
    fn default() -> Pllcr {
        <crate::RegValueT<Pllcr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod pllcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pllstp_SPEC;
    pub type Pllstp = crate::EnumBitfieldStruct<u8, Pllstp_SPEC>;
    impl Pllstp {
        #[doc = "PLL is operating."]
        pub const _0: Self = Self::new(0);
        #[doc = "PLL is stopped."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pllccr2_SPEC;
impl crate::sealed::RegSpec for Pllccr2_SPEC {
    type DataType = u8;
}
#[doc = "PLL Clock Control Register2"]
pub type Pllccr2 = crate::RegValueT<Pllccr2_SPEC>;

impl Pllccr2 {
    #[doc = "PLL Output Frequency Division Ratio Select"]
    #[inline(always)]
    pub fn plodiv(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, pllccr2::Plodiv, Pllccr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,pllccr2::Plodiv, Pllccr2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Pllccr2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Pllccr2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "PLL Frequency Multiplication Factor Select"]
    #[inline(always)]
    pub fn pllmul(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, pllccr2::Pllmul, Pllccr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            pllccr2::Pllmul,
            Pllccr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Pllccr2 {
    #[inline(always)]
    fn default() -> Pllccr2 {
        <crate::RegValueT<Pllccr2_SPEC> as RegisterValue<_>>::new(7)
    }
}
pub mod pllccr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Plodiv_SPEC;
    pub type Plodiv = crate::EnumBitfieldStruct<u8, Plodiv_SPEC>;
    impl Plodiv {
        #[doc = "/1."]
        pub const _00: Self = Self::new(0);
        #[doc = "/2."]
        pub const _01: Self = Self::new(1);
        #[doc = "/4."]
        pub const _10: Self = Self::new(2);
        #[doc = "Setting prohibited."]
        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pllmul_SPEC;
    pub type Pllmul = crate::EnumBitfieldStruct<u8, Pllmul_SPEC>;
    impl Pllmul {
        #[doc = "Settings prohibited."]
        pub const _1111: Self = Self::new(15);
        #[doc = "x PLLMUL\\[4:0\\] +1"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Memwait_SPEC;
impl crate::sealed::RegSpec for Memwait_SPEC {
    type DataType = u8;
}
#[doc = "Memory Wait Cycle Control Register"]
pub type Memwait = crate::RegValueT<Memwait_SPEC>;

impl Memwait {
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, Memwait_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8, Memwait_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Memory Wait Cycle Select\nNote: Writing 0 to the MEMWAIT is prohibited when SCKDIVCR.ICK selects division by 1 and SCKSCR.CKSEL\\[2:0\\] bits select the\nsystem clock source that is faster than 32 MHz (ICLK > 32 MHz)."]
    #[inline(always)]
    pub fn memwait(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, memwait::Memwait, Memwait_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            memwait::Memwait,
            Memwait_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Memwait {
    #[inline(always)]
    fn default() -> Memwait {
        <crate::RegValueT<Memwait_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod memwait {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Memwait_SPEC;
    pub type Memwait = crate::EnumBitfieldStruct<u8, Memwait_SPEC>;
    impl Memwait {
        #[doc = "no wait"]
        pub const _0: Self = Self::new(0);
        #[doc = "wait"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mosccr_SPEC;
impl crate::sealed::RegSpec for Mosccr_SPEC {
    type DataType = u8;
}
#[doc = "Main Clock Oscillator Control Register"]
pub type Mosccr = crate::RegValueT<Mosccr_SPEC>;

impl Mosccr {
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, Mosccr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8, Mosccr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Main Clock Oscillator Stop\nNote: MOMCR register must be set before setting MOSTP to 0."]
    #[inline(always)]
    pub fn mostp(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, mosccr::Mostp, Mosccr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,mosccr::Mostp, Mosccr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mosccr {
    #[inline(always)]
    fn default() -> Mosccr {
        <crate::RegValueT<Mosccr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod mosccr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mostp_SPEC;
    pub type Mostp = crate::EnumBitfieldStruct<u8, Mostp_SPEC>;
    impl Mostp {
        #[doc = "Main clock oscillator is operating."]
        pub const _0: Self = Self::new(0);
        #[doc = "Main clock oscillator is stopped."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hococr_SPEC;
impl crate::sealed::RegSpec for Hococr_SPEC {
    type DataType = u8;
}
#[doc = "High-Speed On-Chip Oscillator Control Register"]
pub type Hococr = crate::RegValueT<Hococr_SPEC>;

impl Hococr {
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, Hococr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8, Hococr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HOCO Stop"]
    #[inline(always)]
    pub fn hcstp(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, hococr::Hcstp, Hococr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,hococr::Hcstp, Hococr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Hococr {
    #[inline(always)]
    fn default() -> Hococr {
        <crate::RegValueT<Hococr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod hococr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hcstp_SPEC;
    pub type Hcstp = crate::EnumBitfieldStruct<u8, Hcstp_SPEC>;
    impl Hcstp {
        #[doc = "HOCO is operating."]
        pub const _0: Self = Self::new(0);
        #[doc = "HOCO is stopped."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mococr_SPEC;
impl crate::sealed::RegSpec for Mococr_SPEC {
    type DataType = u8;
}
#[doc = "Middle-Speed On-Chip Oscillator Control Register"]
pub type Mococr = crate::RegValueT<Mococr_SPEC>;

impl Mococr {
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, Mococr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8, Mococr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "MOCO Stop"]
    #[inline(always)]
    pub fn mcstp(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, mococr::Mcstp, Mococr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,mococr::Mcstp, Mococr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mococr {
    #[inline(always)]
    fn default() -> Mococr {
        <crate::RegValueT<Mococr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mococr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mcstp_SPEC;
    pub type Mcstp = crate::EnumBitfieldStruct<u8, Mcstp_SPEC>;
    impl Mcstp {
        #[doc = "MOCO is operating."]
        pub const _0: Self = Self::new(0);
        #[doc = "MOCO is stopped."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oscsf_SPEC;
impl crate::sealed::RegSpec for Oscsf_SPEC {
    type DataType = u8;
}
#[doc = "Oscillation Stabilization Flag Register"]
pub type Oscsf = crate::RegValueT<Oscsf_SPEC>;

impl Oscsf {
    #[doc = "PLL Clock Oscillation Stabilization Flag"]
    #[inline(always)]
    pub fn pllsf(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, oscsf::Pllsf, Oscsf_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<5,0x1,1,0,oscsf::Pllsf, Oscsf_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Main Clock Oscillation Stabilization Flag"]
    #[inline(always)]
    pub fn moscsf(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, oscsf::Moscsf, Oscsf_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<3,0x1,1,0,oscsf::Moscsf, Oscsf_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "These bits are read as 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, Oscsf_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x3,1,0,u8, Oscsf_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "HOCO Clock Oscillation Stabilization Flag\nNOTE: The HOCOSF bit value after a reset is 1 when the OFS1.HOCOEN bit is 0. It is 0 when the OFS1.HOCOEN bit is 1."]
    #[inline(always)]
    pub fn hocosf(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, oscsf::Hocosf, Oscsf_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,oscsf::Hocosf, Oscsf_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Oscsf {
    #[inline(always)]
    fn default() -> Oscsf {
        <crate::RegValueT<Oscsf_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod oscsf {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pllsf_SPEC;
    pub type Pllsf = crate::EnumBitfieldStruct<u8, Pllsf_SPEC>;
    impl Pllsf {
        #[doc = "The PLL clock is stopped or oscillation of the PLL clock has not yet become stable."]
        pub const _0: Self = Self::new(0);
        #[doc = "Oscillation of the PLL clock is stable so the clock is available for use as the system clock."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Moscsf_SPEC;
    pub type Moscsf = crate::EnumBitfieldStruct<u8, Moscsf_SPEC>;
    impl Moscsf {
        #[doc = "MOSTP = 1 (stopping the main clock oscillator) or oscillation of the main clock has not yet become stable."]
        pub const _0: Self = Self::new(0);
        #[doc = "Oscillation of the main clock is stable so the clock is available for use as the system clock."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hocosf_SPEC;
    pub type Hocosf = crate::EnumBitfieldStruct<u8, Hocosf_SPEC>;
    impl Hocosf {
        #[doc = "The HOCO clock is stopped or oscillation of the HOCO clock has not yet become stable."]
        pub const _0: Self = Self::new(0);
        #[doc = "Oscillation of the HOCO clock is stable so the clock is available for use as the system clock."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ckocr_SPEC;
impl crate::sealed::RegSpec for Ckocr_SPEC {
    type DataType = u8;
}
#[doc = "Clock Out Control Register"]
pub type Ckocr = crate::RegValueT<Ckocr_SPEC>;

impl Ckocr {
    #[doc = "Clock out enable"]
    #[inline(always)]
    pub fn ckoen(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, ckocr::Ckoen, Ckocr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,ckocr::Ckoen, Ckocr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock out input frequency Division Select"]
    #[inline(always)]
    pub fn ckodiv(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, ckocr::Ckodiv, Ckocr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x7,1,0,ckocr::Ckodiv, Ckocr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Ckocr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ckocr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Clock out source select"]
    #[inline(always)]
    pub fn ckosel(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, ckocr::Ckosel, Ckocr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,ckocr::Ckosel, Ckocr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ckocr {
    #[inline(always)]
    fn default() -> Ckocr {
        <crate::RegValueT<Ckocr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ckocr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ckoen_SPEC;
    pub type Ckoen = crate::EnumBitfieldStruct<u8, Ckoen_SPEC>;
    impl Ckoen {
        #[doc = "Clock Out disable"]
        pub const _0: Self = Self::new(0);
        #[doc = "Clock Out enable"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ckodiv_SPEC;
    pub type Ckodiv = crate::EnumBitfieldStruct<u8, Ckodiv_SPEC>;
    impl Ckodiv {
        #[doc = "/1"]
        pub const _000: Self = Self::new(0);
        #[doc = "/2"]
        pub const _001: Self = Self::new(1);
        #[doc = "/4"]
        pub const _010: Self = Self::new(2);
        #[doc = "/8"]
        pub const _011: Self = Self::new(3);
        #[doc = "/16"]
        pub const _100: Self = Self::new(4);
        #[doc = "/32"]
        pub const _101: Self = Self::new(5);
        #[doc = "/64"]
        pub const _110: Self = Self::new(6);
        #[doc = "/128"]
        pub const _111: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ckosel_SPEC;
    pub type Ckosel = crate::EnumBitfieldStruct<u8, Ckosel_SPEC>;
    impl Ckosel {
        #[doc = "HOCO"]
        pub const _000: Self = Self::new(0);
        #[doc = "MOCO"]
        pub const _001: Self = Self::new(1);
        #[doc = "LOCO"]
        pub const _010: Self = Self::new(2);
        #[doc = "MOSC"]
        pub const _011: Self = Self::new(3);
        #[doc = "SOSC"]
        pub const _100: Self = Self::new(4);
        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trckcr_SPEC;
impl crate::sealed::RegSpec for Trckcr_SPEC {
    type DataType = u8;
}
#[doc = "Trace Clock Control Register"]
pub type Trckcr = crate::RegValueT<Trckcr_SPEC>;

impl Trckcr {
    #[doc = "Trace Clock operating enable"]
    #[inline(always)]
    pub fn trcken(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, trckcr::Trcken, Trckcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,trckcr::Trcken, Trckcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, Trckcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8, Trckcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trace Clock operating frequency select"]
    #[inline(always)]
    pub fn trck(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, trckcr::Trck, Trckcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,trckcr::Trck, Trckcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Trckcr {
    #[inline(always)]
    fn default() -> Trckcr {
        <crate::RegValueT<Trckcr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod trckcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trcken_SPEC;
    pub type Trcken = crate::EnumBitfieldStruct<u8, Trcken_SPEC>;
    impl Trcken {
        #[doc = "Operation disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Operation enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trck_SPEC;
    pub type Trck = crate::EnumBitfieldStruct<u8, Trck_SPEC>;
    impl Trck {
        #[doc = "/1"]
        pub const _0000: Self = Self::new(0);
        #[doc = "/2(value after reset)"]
        pub const _0001: Self = Self::new(1);
        #[doc = "/4"]
        pub const _0010: Self = Self::new(2);
        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ostdcr_SPEC;
impl crate::sealed::RegSpec for Ostdcr_SPEC {
    type DataType = u8;
}
#[doc = "Oscillation Stop Detection Control Register"]
pub type Ostdcr = crate::RegValueT<Ostdcr_SPEC>;

impl Ostdcr {
    #[doc = "Oscillation Stop Detection Function Enable"]
    #[inline(always)]
    pub fn ostde(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, ostdcr::Ostde, Ostdcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,ostdcr::Ostde, Ostdcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x3f, 1, 0, u8, Ostdcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x3f,1,0,u8, Ostdcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Oscillation Stop Detection Interrupt Enable"]
    #[inline(always)]
    pub fn ostdie(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, ostdcr::Ostdie, Ostdcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,ostdcr::Ostdie, Ostdcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ostdcr {
    #[inline(always)]
    fn default() -> Ostdcr {
        <crate::RegValueT<Ostdcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ostdcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ostde_SPEC;
    pub type Ostde = crate::EnumBitfieldStruct<u8, Ostde_SPEC>;
    impl Ostde {
        #[doc = "Oscillation stop detection function is disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "Oscillation stop detection function is enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ostdie_SPEC;
    pub type Ostdie = crate::EnumBitfieldStruct<u8, Ostdie_SPEC>;
    impl Ostdie {
        #[doc = "The oscillation stop detection interrupt is disabled. Oscillation stop detection is not notified to the POEG."]
        pub const _0: Self = Self::new(0);
        #[doc = "The oscillation stop detection interrupt is enabled. Oscillation stop detection is notified to the POEG."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ostdsr_SPEC;
impl crate::sealed::RegSpec for Ostdsr_SPEC {
    type DataType = u8;
}
#[doc = "Oscillation Stop Detection Status Register"]
pub type Ostdsr = crate::RegValueT<Ostdsr_SPEC>;

impl Ostdsr {
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, Ostdsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8, Ostdsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Oscillation Stop Detection Flag"]
    #[inline(always)]
    pub fn ostdf(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, ostdsr::Ostdf, Ostdsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,ostdsr::Ostdf, Ostdsr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ostdsr {
    #[inline(always)]
    fn default() -> Ostdsr {
        <crate::RegValueT<Ostdsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ostdsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ostdf_SPEC;
    pub type Ostdf = crate::EnumBitfieldStruct<u8, Ostdf_SPEC>;
    impl Ostdf {
        #[doc = "The main clock oscillation stop has not been detected."]
        pub const _0: Self = Self::new(0);
        #[doc = "The main clock oscillation stop has been detected."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Slcdsckcr_SPEC;
impl crate::sealed::RegSpec for Slcdsckcr_SPEC {
    type DataType = u8;
}
#[doc = "Segment LCD Source Clock Control Register"]
pub type Slcdsckcr = crate::RegValueT<Slcdsckcr_SPEC>;

impl Slcdsckcr {
    #[doc = "LCD Source Clock Out Enable"]
    #[inline(always)]
    pub fn lcdscken(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        slcdsckcr::Lcdscken,
        Slcdsckcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            slcdsckcr::Lcdscken,
            Slcdsckcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0xf, 1, 0, u8, Slcdsckcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0xf,1,0,u8, Slcdsckcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LCD Source Clock (LCDSRCCLK) Select"]
    #[inline(always)]
    pub fn lcdscksel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        slcdsckcr::Lcdscksel,
        Slcdsckcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            slcdsckcr::Lcdscksel,
            Slcdsckcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Slcdsckcr {
    #[inline(always)]
    fn default() -> Slcdsckcr {
        <crate::RegValueT<Slcdsckcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod slcdsckcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lcdscken_SPEC;
    pub type Lcdscken = crate::EnumBitfieldStruct<u8, Lcdscken_SPEC>;
    impl Lcdscken {
        #[doc = "LCD source clock out disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "LCD source clock out enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lcdscksel_SPEC;
    pub type Lcdscksel = crate::EnumBitfieldStruct<u8, Lcdscksel_SPEC>;
    impl Lcdscksel {
        #[doc = "LOCO"]
        pub const _000: Self = Self::new(0);
        #[doc = "SOSC"]
        pub const _001: Self = Self::new(1);
        #[doc = "MOSC"]
        pub const _010: Self = Self::new(2);
        #[doc = "HOCO"]
        pub const _100: Self = Self::new(4);
        #[doc = "Settings other than above are prohibited."]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mocoutcr_SPEC;
impl crate::sealed::RegSpec for Mocoutcr_SPEC {
    type DataType = u8;
}
#[doc = "MOCO User Trimming Control Register"]
pub type Mocoutcr = crate::RegValueT<Mocoutcr_SPEC>;

impl Mocoutcr {
    #[doc = "MOCO User Trimming\n  1000_0000 : -128 \n  1000_0001 : -127 \n  1000_0010 : -126 \n  . . .\n  1111_1111 : -1\n  0000_0000 : Center Code\n  0000_0001 : +1\n  . . .\n  0111_1101 : +125\n  0111_1110 : +126\n  0111_1111 : +127\nThese bits are added to original MOCO trimming bits"]
    #[inline(always)]
    pub fn mocoutrm(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Mocoutcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Mocoutcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mocoutcr {
    #[inline(always)]
    fn default() -> Mocoutcr {
        <crate::RegValueT<Mocoutcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hocoutcr_SPEC;
impl crate::sealed::RegSpec for Hocoutcr_SPEC {
    type DataType = u8;
}
#[doc = "HOCO User Trimming Control Register"]
pub type Hocoutcr = crate::RegValueT<Hocoutcr_SPEC>;

impl Hocoutcr {
    #[doc = "HOCO User Trimming\n  1000_0000 : -128 \n  1000_0001 : -127 \n  1000_0010 : -126 \n  . . .\n  1111_1111 : -1\n  0000_0000 : Center Code\n  0000_0001 : +1\n  . . .\n  0111_1101 : +125\n  0111_1110 : +126\n  0111_1111 : +127\nThese bits are added to original HOCO trimming bits"]
    #[inline(always)]
    pub fn hocoutrm(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Hocoutcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Hocoutcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Hocoutcr {
    #[inline(always)]
    fn default() -> Hocoutcr {
        <crate::RegValueT<Hocoutcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Moscwtcr_SPEC;
impl crate::sealed::RegSpec for Moscwtcr_SPEC {
    type DataType = u8;
}
#[doc = "Main Clock Oscillator Wait Control Register"]
pub type Moscwtcr = crate::RegValueT<Moscwtcr_SPEC>;

impl Moscwtcr {
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Moscwtcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Moscwtcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Main clock oscillator wait time setting"]
    #[inline(always)]
    pub fn msts(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, moscwtcr::Msts, Moscwtcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,moscwtcr::Msts, Moscwtcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Moscwtcr {
    #[inline(always)]
    fn default() -> Moscwtcr {
        <crate::RegValueT<Moscwtcr_SPEC> as RegisterValue<_>>::new(5)
    }
}
pub mod moscwtcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Msts_SPEC;
    pub type Msts = crate::EnumBitfieldStruct<u8, Msts_SPEC>;
    impl Msts {
        #[doc = "Wait time = 2 cycles (0.25 us)"]
        pub const _0000: Self = Self::new(0);
        #[doc = "Wait time = 1024 cycles (128 us)"]
        pub const _0001: Self = Self::new(1);
        #[doc = "Wait time = 2048 cycles (256 us)"]
        pub const _0010: Self = Self::new(2);
        #[doc = "Wait time = 4096 cycles (512 us)"]
        pub const _0011: Self = Self::new(3);
        #[doc = "Wait time = 8192 cycles (1024 us)"]
        pub const _0100: Self = Self::new(4);
        #[doc = "Wait time = 16384 cycles (2048 us) (value after reset)"]
        pub const _0101: Self = Self::new(5);
        #[doc = "Wait time = 32768 cycles (4096 us)"]
        pub const _0110: Self = Self::new(6);
        #[doc = "Wait time = 65536 cycles (8192 us)"]
        pub const _0111: Self = Self::new(7);
        #[doc = "Wait time = 131072 cycles (16384 us)"]
        pub const _1000: Self = Self::new(8);
        #[doc = "Wait time = 262144 cycles (32768 us)."]
        pub const _1001: Self = Self::new(9);
        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hocowtcr_SPEC;
impl crate::sealed::RegSpec for Hocowtcr_SPEC {
    type DataType = u8;
}
#[doc = "High-Speed On-Chip Oscillator Wait Control Register"]
pub type Hocowtcr = crate::RegValueT<Hocowtcr_SPEC>;

impl Hocowtcr {
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, Hocowtcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8, Hocowtcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HOCO wait time setting"]
    #[inline(always)]
    pub fn hsts(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, hocowtcr::Hsts, Hocowtcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,hocowtcr::Hsts, Hocowtcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Hocowtcr {
    #[inline(always)]
    fn default() -> Hocowtcr {
        <crate::RegValueT<Hocowtcr_SPEC> as RegisterValue<_>>::new(5)
    }
}
pub mod hocowtcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hsts_SPEC;
    pub type Hsts = crate::EnumBitfieldStruct<u8, Hsts_SPEC>;
    impl Hsts {
        #[doc = "If HOCO frequency is other than 64MHz, should set the value to 101b."]
        pub const _101: Self = Self::new(5);
        #[doc = "If HOCO frequency = 64MHz, should set the value to 110b."]
        pub const _110: Self = Self::new(6);
        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbckcr_SPEC;
impl crate::sealed::RegSpec for Usbckcr_SPEC {
    type DataType = u8;
}
#[doc = "USB Clock Control register"]
pub type Usbckcr = crate::RegValueT<Usbckcr_SPEC>;

impl Usbckcr {
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, Usbckcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8, Usbckcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "USB Clock Source Select"]
    #[inline(always)]
    pub fn usbclksel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        usbckcr::Usbclksel,
        Usbckcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            usbckcr::Usbclksel,
            Usbckcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Usbckcr {
    #[inline(always)]
    fn default() -> Usbckcr {
        <crate::RegValueT<Usbckcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod usbckcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Usbclksel_SPEC;
    pub type Usbclksel = crate::EnumBitfieldStruct<u8, Usbclksel_SPEC>;
    impl Usbclksel {
        #[doc = "PLL(Value after reset)"]
        pub const _0: Self = Self::new(0);
        #[doc = "HOCO"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Momcr_SPEC;
impl crate::sealed::RegSpec for Momcr_SPEC {
    type DataType = u8;
}
#[doc = "Main Clock Oscillator Mode Oscillation Control Register"]
pub type Momcr = crate::RegValueT<Momcr_SPEC>;

impl Momcr {
    #[doc = "Main Clock Oscillator Switching"]
    #[inline(always)]
    pub fn mosel(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, momcr::Mosel, Momcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,momcr::Mosel, Momcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Main Clock Oscillator Drive Capability 1 Switching"]
    #[inline(always)]
    pub fn modrv1(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, momcr::Modrv1, Momcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,momcr::Modrv1, Momcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Momcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, Momcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Momcr {
    #[inline(always)]
    fn default() -> Momcr {
        <crate::RegValueT<Momcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod momcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mosel_SPEC;
    pub type Mosel = crate::EnumBitfieldStruct<u8, Mosel_SPEC>;
    impl Mosel {
        #[doc = "Resonator"]
        pub const _0: Self = Self::new(0);
        #[doc = "External clock input"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Modrv1_SPEC;
    pub type Modrv1 = crate::EnumBitfieldStruct<u8, Modrv1_SPEC>;
    impl Modrv1 {
        #[doc = "10 MHz to 20 MHz"]
        pub const _0: Self = Self::new(0);
        #[doc = "1 MHz to 10 MHz."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sosccr_SPEC;
impl crate::sealed::RegSpec for Sosccr_SPEC {
    type DataType = u8;
}
#[doc = "Sub-Clock Oscillator Control Register"]
pub type Sosccr = crate::RegValueT<Sosccr_SPEC>;

impl Sosccr {
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, Sosccr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8, Sosccr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sub-Clock Oscillator Stop"]
    #[inline(always)]
    pub fn sostp(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, sosccr::Sostp, Sosccr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,sosccr::Sostp, Sosccr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sosccr {
    #[inline(always)]
    fn default() -> Sosccr {
        <crate::RegValueT<Sosccr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod sosccr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sostp_SPEC;
    pub type Sostp = crate::EnumBitfieldStruct<u8, Sostp_SPEC>;
    impl Sostp {
        #[doc = "Sub-clock oscillator is operating."]
        pub const _0: Self = Self::new(0);
        #[doc = "Sub-clock oscillator is stopped."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Somcr_SPEC;
impl crate::sealed::RegSpec for Somcr_SPEC {
    type DataType = u8;
}
#[doc = "Sub Clock Oscillator Mode Control Register"]
pub type Somcr = crate::RegValueT<Somcr_SPEC>;

impl Somcr {
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, Somcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3f,1,0,u8, Somcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sub-Clock Oscillator Drive Capability Switching"]
    #[inline(always)]
    pub fn sodrv(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, somcr::Sodrv, Somcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,somcr::Sodrv, Somcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Somcr {
    #[inline(always)]
    fn default() -> Somcr {
        <crate::RegValueT<Somcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod somcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sodrv_SPEC;
    pub type Sodrv = crate::EnumBitfieldStruct<u8, Sodrv_SPEC>;
    impl Sodrv {
        #[doc = "Normal mode"]
        pub const _00: Self = Self::new(0);
        #[doc = "Low power mode 1"]
        pub const _01: Self = Self::new(1);
        #[doc = "Low power mode 2"]
        pub const _10: Self = Self::new(2);
        #[doc = "Low power mode 3."]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lococr_SPEC;
impl crate::sealed::RegSpec for Lococr_SPEC {
    type DataType = u8;
}
#[doc = "Low-Speed On-Chip Oscillator Control Register"]
pub type Lococr = crate::RegValueT<Lococr_SPEC>;

impl Lococr {
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, Lococr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8, Lococr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LOCO Stop"]
    #[inline(always)]
    pub fn lcstp(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, lococr::Lcstp, Lococr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,lococr::Lcstp, Lococr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Lococr {
    #[inline(always)]
    fn default() -> Lococr {
        <crate::RegValueT<Lococr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod lococr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lcstp_SPEC;
    pub type Lcstp = crate::EnumBitfieldStruct<u8, Lcstp_SPEC>;
    impl Lcstp {
        #[doc = "LOCO is operating."]
        pub const _0: Self = Self::new(0);
        #[doc = "LOCO is stopped."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Locoutcr_SPEC;
impl crate::sealed::RegSpec for Locoutcr_SPEC {
    type DataType = u8;
}
#[doc = "LOCO User Trimming Control Register"]
pub type Locoutcr = crate::RegValueT<Locoutcr_SPEC>;

impl Locoutcr {
    #[doc = "LOCO User Trimming\n  1000_0000 : -128 \n  1000_0001 : -127 \n  1000_0010 : -126 \n  . . .\n  1111_1111 : -1\n  0000_0000 : Center Code\n  0000_0001 : +1\n  . . .\n  0111_1101 : +125\n  0111_1110 : +126\n  0111_1111 : +127\nThese bits are added to original LOCO trimming bits"]
    #[inline(always)]
    pub fn locoutrm(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Locoutcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Locoutcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Locoutcr {
    #[inline(always)]
    fn default() -> Locoutcr {
        <crate::RegValueT<Locoutcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sbycr_SPEC;
impl crate::sealed::RegSpec for Sbycr_SPEC {
    type DataType = u16;
}
#[doc = "Standby Control Register"]
pub type Sbycr = crate::RegValueT<Sbycr_SPEC>;

impl Sbycr {
    #[doc = "Software Standby"]
    #[inline(always)]
    pub fn ssby(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, sbycr::Ssby, Sbycr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,sbycr::Ssby, Sbycr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 00000000000000. The write value should be 00000000000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, Sbycr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3fff,1,0,u16, Sbycr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sbycr {
    #[inline(always)]
    fn default() -> Sbycr {
        <crate::RegValueT<Sbycr_SPEC> as RegisterValue<_>>::new(16384)
    }
}
pub mod sbycr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssby_SPEC;
    pub type Ssby = crate::EnumBitfieldStruct<u8, Ssby_SPEC>;
    impl Ssby {
        #[doc = "Sleep mode"]
        pub const _0: Self = Self::new(0);
        #[doc = "Software Standby mode"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mstpcra_SPEC;
impl crate::sealed::RegSpec for Mstpcra_SPEC {
    type DataType = u32;
}
#[doc = "Module Stop Control Register A"]
pub type Mstpcra = crate::RegValueT<Mstpcra_SPEC>;

impl Mstpcra {
    #[doc = "DMA Controller/Data Transfer Controller Module Stop"]
    #[inline(always)]
    pub fn mstpa22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        mstpcra::Mstpa22,
        Mstpcra_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            mstpcra::Mstpa22,
            Mstpcra_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ECCRAM Module Stop"]
    #[inline(always)]
    pub fn mstpa6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, mstpcra::Mstpa6, Mstpcra_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,mstpcra::Mstpa6, Mstpcra_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 11111. The write value should be 11111."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x1f, 1, 0, u8, Mstpcra_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1f,1,0,u8, Mstpcra_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "RAM0 Module Stop"]
    #[inline(always)]
    pub fn mstpa0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, mstpcra::Mstpa0, Mstpcra_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,mstpcra::Mstpa0, Mstpcra_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mstpcra {
    #[inline(always)]
    fn default() -> Mstpcra {
        <crate::RegValueT<Mstpcra_SPEC> as RegisterValue<_>>::new(4290772926)
    }
}
pub mod mstpcra {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpa22_SPEC;
    pub type Mstpa22 = crate::EnumBitfieldStruct<u8, Mstpa22_SPEC>;
    impl Mstpa22 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpa6_SPEC;
    pub type Mstpa6 = crate::EnumBitfieldStruct<u8, Mstpa6_SPEC>;
    impl Mstpa6 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mstpa0_SPEC;
    pub type Mstpa0 = crate::EnumBitfieldStruct<u8, Mstpa0_SPEC>;
    impl Mstpa0 {
        #[doc = "Cancel the module-stop state"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enter the module-stop state"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Snzcr_SPEC;
impl crate::sealed::RegSpec for Snzcr_SPEC {
    type DataType = u8;
}
#[doc = "Snooze Control Register"]
pub type Snzcr = crate::RegValueT<Snzcr_SPEC>;

impl Snzcr {
    #[doc = "Snooze Mode Enable"]
    #[inline(always)]
    pub fn snze(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, snzcr::Snze, Snzcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,snzcr::Snze, Snzcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x1f, 1, 0, u8, Snzcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1f,1,0,u8, Snzcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DTC Enable in Snooze Mode"]
    #[inline(always)]
    pub fn snzdtcen(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, snzcr::Snzdtcen, Snzcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,snzcr::Snzdtcen, Snzcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "RXD0 Snooze Request Enable \nNOTE: Do not set to 1 other than in asynchronous mode."]
    #[inline(always)]
    pub fn rxdreqen(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, snzcr::Rxdreqen, Snzcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,snzcr::Rxdreqen, Snzcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Snzcr {
    #[inline(always)]
    fn default() -> Snzcr {
        <crate::RegValueT<Snzcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod snzcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snze_SPEC;
    pub type Snze = crate::EnumBitfieldStruct<u8, Snze_SPEC>;
    impl Snze {
        #[doc = "Disable Snooze Mode"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable Snooze Mode"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzdtcen_SPEC;
    pub type Snzdtcen = crate::EnumBitfieldStruct<u8, Snzdtcen_SPEC>;
    impl Snzdtcen {
        #[doc = "Disable DTC operation"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable DTC operation"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxdreqen_SPEC;
    pub type Rxdreqen = crate::EnumBitfieldStruct<u8, Rxdreqen_SPEC>;
    impl Rxdreqen {
        #[doc = "Ignore RXD0 falling edge in Software Standby mode."]
        pub const _0: Self = Self::new(0);
        #[doc = "Accept RXD0 falling edge in Standby mode as a request to transit to Snooze mode."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Snzedcr_SPEC;
impl crate::sealed::RegSpec for Snzedcr_SPEC {
    type DataType = u8;
}
#[doc = "Snooze End Control Register"]
pub type Snzedcr = crate::RegValueT<Snzedcr_SPEC>;

impl Snzedcr {
    #[doc = "SCI0 Address Mismatch Snooze End Enable"]
    #[inline(always)]
    pub fn sci0umted(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        snzedcr::Sci0Umted,
        Snzedcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            snzedcr::Sci0Umted,
            Snzedcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, Snzedcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x3,1,0,u8, Snzedcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ADC140 Compare Mismatch Snooze End Enable"]
    #[inline(always)]
    pub fn ad0umted(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        snzedcr::Ad0Umted,
        Snzedcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            snzedcr::Ad0Umted,
            Snzedcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ADC140 Compare Match Snooze End Enable"]
    #[inline(always)]
    pub fn ad0mated(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        snzedcr::Ad0Mated,
        Snzedcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            snzedcr::Ad0Mated,
            Snzedcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Not Last DTC Transmission Completion Snooze End Enable"]
    #[inline(always)]
    pub fn dtcnzred(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        snzedcr::Dtcnzred,
        Snzedcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            snzedcr::Dtcnzred,
            Snzedcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Last DTC Transmission Completion Snooze End Enable"]
    #[inline(always)]
    pub fn dtczred(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, snzedcr::Dtczred, Snzedcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            snzedcr::Dtczred,
            Snzedcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "AGT1 Underflow Snooze End Enable"]
    #[inline(always)]
    pub fn agtunfed(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        snzedcr::Agtunfed,
        Snzedcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            snzedcr::Agtunfed,
            Snzedcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Snzedcr {
    #[inline(always)]
    fn default() -> Snzedcr {
        <crate::RegValueT<Snzedcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod snzedcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sci0Umted_SPEC;
    pub type Sci0Umted = crate::EnumBitfieldStruct<u8, Sci0Umted_SPEC>;
    impl Sci0Umted {
        #[doc = "Disable the Snooze End request"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable the Snooze End request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ad0Umted_SPEC;
    pub type Ad0Umted = crate::EnumBitfieldStruct<u8, Ad0Umted_SPEC>;
    impl Ad0Umted {
        #[doc = "Disable the Snooze End request"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable the Snooze End request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ad0Mated_SPEC;
    pub type Ad0Mated = crate::EnumBitfieldStruct<u8, Ad0Mated_SPEC>;
    impl Ad0Mated {
        #[doc = "Disable the Snooze End request"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable the Snooze End request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtcnzred_SPEC;
    pub type Dtcnzred = crate::EnumBitfieldStruct<u8, Dtcnzred_SPEC>;
    impl Dtcnzred {
        #[doc = "Disable the Snooze End request"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable the Snooze End request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dtczred_SPEC;
    pub type Dtczred = crate::EnumBitfieldStruct<u8, Dtczred_SPEC>;
    impl Dtczred {
        #[doc = "Disable the Snooze End request"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable the Snooze End request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Agtunfed_SPEC;
    pub type Agtunfed = crate::EnumBitfieldStruct<u8, Agtunfed_SPEC>;
    impl Agtunfed {
        #[doc = "Disable the Snooze End request"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable the Snooze End request"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Snzreqcr_SPEC;
impl crate::sealed::RegSpec for Snzreqcr_SPEC {
    type DataType = u32;
}
#[doc = "Snooze Request Control Register"]
pub type Snzreqcr = crate::RegValueT<Snzreqcr_SPEC>;

impl Snzreqcr {
    #[doc = "Snooze Request Enable 30\nEnable AGT1 compare match B snooze request"]
    #[inline(always)]
    pub fn snzreqen30(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen30,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen30,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Snooze Request Enable 29\nEnable AGT1 compare match A snooze request"]
    #[inline(always)]
    pub fn snzreqen29(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen29,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen29,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Snooze Request Enable 28\nEnable AGT1 underflow snooze request"]
    #[inline(always)]
    pub fn snzreqen28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen28,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen28,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Snooze Request Enable 25\nEnable RTC period snooze request"]
    #[inline(always)]
    pub fn snzreqen25(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen25,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen25,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Snooze Request Enable 24\nEnable RTC alarm snooze request"]
    #[inline(always)]
    pub fn snzreqen24(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen24,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen24,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Snooze Request Enable 23\nEnable RTC alarm snooze request"]
    #[inline(always)]
    pub fn snzreqen23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen23,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen23,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Snooze Request Enable 17\nEnable KINT snooze request"]
    #[inline(always)]
    pub fn snzreqen17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen17,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen17,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Snooze Request Enable 15\nEnable IRQ15 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen15,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen15,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Snooze Request Enable 14\nEnable IRQ14 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen14,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen14,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Snzreqcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,Snzreqcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Snooze Request Enable 12\nEnable IRQ12 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen12,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen12,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Snooze Request Enable 11\nEnable IRQ11 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen11,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen11,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Snooze Request Enable 10\nEnable IRQ10 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen10,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen10,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Snooze Request Enable 9\nEnable IRQ9 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen9,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen9,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Snooze Request Enable 8\nEnable IRQ8 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen8,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen8,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Snooze Request Enable 7\nEnable IRQ7 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen7,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen7,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Snooze Request Enable 6\nEnable IRQ6 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen6,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen6,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Snooze Request Enable 5\nEnable IRQ5 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen5,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen5,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Snooze Request Enable 4\nEnable IRQ4 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen4,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen4,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Snooze Request Enable 3\nEnable IRQ3 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen3,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen3,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Snooze Request Enable 2\nEnable IRQ2 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen2,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen2,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Snooze Request Enable 1\nEnable IRQ1 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen1,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen1,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Snooze Request Enable 0\nEnable IRQ0 pin snooze request"]
    #[inline(always)]
    pub fn snzreqen0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        snzreqcr::Snzreqen0,
        Snzreqcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            snzreqcr::Snzreqen0,
            Snzreqcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Snzreqcr {
    #[inline(always)]
    fn default() -> Snzreqcr {
        <crate::RegValueT<Snzreqcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod snzreqcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen30_SPEC;
    pub type Snzreqen30 = crate::EnumBitfieldStruct<u8, Snzreqen30_SPEC>;
    impl Snzreqen30 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen29_SPEC;
    pub type Snzreqen29 = crate::EnumBitfieldStruct<u8, Snzreqen29_SPEC>;
    impl Snzreqen29 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen28_SPEC;
    pub type Snzreqen28 = crate::EnumBitfieldStruct<u8, Snzreqen28_SPEC>;
    impl Snzreqen28 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen25_SPEC;
    pub type Snzreqen25 = crate::EnumBitfieldStruct<u8, Snzreqen25_SPEC>;
    impl Snzreqen25 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen24_SPEC;
    pub type Snzreqen24 = crate::EnumBitfieldStruct<u8, Snzreqen24_SPEC>;
    impl Snzreqen24 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen23_SPEC;
    pub type Snzreqen23 = crate::EnumBitfieldStruct<u8, Snzreqen23_SPEC>;
    impl Snzreqen23 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen17_SPEC;
    pub type Snzreqen17 = crate::EnumBitfieldStruct<u8, Snzreqen17_SPEC>;
    impl Snzreqen17 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen15_SPEC;
    pub type Snzreqen15 = crate::EnumBitfieldStruct<u8, Snzreqen15_SPEC>;
    impl Snzreqen15 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen14_SPEC;
    pub type Snzreqen14 = crate::EnumBitfieldStruct<u8, Snzreqen14_SPEC>;
    impl Snzreqen14 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen12_SPEC;
    pub type Snzreqen12 = crate::EnumBitfieldStruct<u8, Snzreqen12_SPEC>;
    impl Snzreqen12 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen11_SPEC;
    pub type Snzreqen11 = crate::EnumBitfieldStruct<u8, Snzreqen11_SPEC>;
    impl Snzreqen11 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen10_SPEC;
    pub type Snzreqen10 = crate::EnumBitfieldStruct<u8, Snzreqen10_SPEC>;
    impl Snzreqen10 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen9_SPEC;
    pub type Snzreqen9 = crate::EnumBitfieldStruct<u8, Snzreqen9_SPEC>;
    impl Snzreqen9 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen8_SPEC;
    pub type Snzreqen8 = crate::EnumBitfieldStruct<u8, Snzreqen8_SPEC>;
    impl Snzreqen8 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen7_SPEC;
    pub type Snzreqen7 = crate::EnumBitfieldStruct<u8, Snzreqen7_SPEC>;
    impl Snzreqen7 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen6_SPEC;
    pub type Snzreqen6 = crate::EnumBitfieldStruct<u8, Snzreqen6_SPEC>;
    impl Snzreqen6 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen5_SPEC;
    pub type Snzreqen5 = crate::EnumBitfieldStruct<u8, Snzreqen5_SPEC>;
    impl Snzreqen5 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen4_SPEC;
    pub type Snzreqen4 = crate::EnumBitfieldStruct<u8, Snzreqen4_SPEC>;
    impl Snzreqen4 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen3_SPEC;
    pub type Snzreqen3 = crate::EnumBitfieldStruct<u8, Snzreqen3_SPEC>;
    impl Snzreqen3 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen2_SPEC;
    pub type Snzreqen2 = crate::EnumBitfieldStruct<u8, Snzreqen2_SPEC>;
    impl Snzreqen2 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen1_SPEC;
    pub type Snzreqen1 = crate::EnumBitfieldStruct<u8, Snzreqen1_SPEC>;
    impl Snzreqen1 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Snzreqen0_SPEC;
    pub type Snzreqen0 = crate::EnumBitfieldStruct<u8, Snzreqen0_SPEC>;
    impl Snzreqen0 {
        #[doc = "Disable snooze request"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enable snooze request"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flstop_SPEC;
impl crate::sealed::RegSpec for Flstop_SPEC {
    type DataType = u8;
}
#[doc = "Flash Operation Control Register"]
pub type Flstop = crate::RegValueT<Flstop_SPEC>;

impl Flstop {
    #[doc = "Flash Memory Operation Status Flag"]
    #[inline(always)]
    pub fn flstpf(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, flstop::Flstpf, Flstop_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0x1,1,0,flstop::Flstpf, Flstop_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7, 1, 0, u8, Flstop_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7,1,0,u8, Flstop_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Selecting ON/OFF of the Flash Memory Operation"]
    #[inline(always)]
    pub fn flstop(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, flstop::Flstop, Flstop_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,flstop::Flstop, Flstop_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Flstop {
    #[inline(always)]
    fn default() -> Flstop {
        <crate::RegValueT<Flstop_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod flstop {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flstpf_SPEC;
    pub type Flstpf = crate::EnumBitfieldStruct<u8, Flstpf_SPEC>;
    impl Flstpf {
        #[doc = "Transition completed"]
        pub const _0: Self = Self::new(0);
        #[doc = "During transition (from the flash-stop-status to flash-operating-status or vice versa)"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Flstop_SPEC;
    pub type Flstop = crate::EnumBitfieldStruct<u8, Flstop_SPEC>;
    impl Flstop {
        #[doc = "Code flash and data flash memory operates"]
        pub const _0: Self = Self::new(0);
        #[doc = "Code flash and data flash memory stops."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Opccr_SPEC;
impl crate::sealed::RegSpec for Opccr_SPEC {
    type DataType = u8;
}
#[doc = "Operating Power Control Register"]
pub type Opccr = crate::RegValueT<Opccr_SPEC>;

impl Opccr {
    #[doc = "Operating Power Control Mode Transition Status Flag"]
    #[inline(always)]
    pub fn opcmtsf(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, opccr::Opcmtsf, Opccr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0x1,1,0,opccr::Opcmtsf, Opccr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, Opccr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8, Opccr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Operating Power Control Mode Select"]
    #[inline(always)]
    pub fn opcm(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, opccr::Opcm, Opccr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,opccr::Opcm, Opccr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Opccr {
    #[inline(always)]
    fn default() -> Opccr {
        <crate::RegValueT<Opccr_SPEC> as RegisterValue<_>>::new(2)
    }
}
pub mod opccr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Opcmtsf_SPEC;
    pub type Opcmtsf = crate::EnumBitfieldStruct<u8, Opcmtsf_SPEC>;
    impl Opcmtsf {
        #[doc = "Transition completed"]
        pub const _0: Self = Self::new(0);
        #[doc = "During transition"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Opcm_SPEC;
    pub type Opcm = crate::EnumBitfieldStruct<u8, Opcm_SPEC>;
    impl Opcm {
        #[doc = "High-speed mode"]
        pub const _00: Self = Self::new(0);
        #[doc = "Middle-speed mode"]
        pub const _01: Self = Self::new(1);
        #[doc = "Low-voltage mode"]
        pub const _10: Self = Self::new(2);
        #[doc = "Low-speed mode"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sopccr_SPEC;
impl crate::sealed::RegSpec for Sopccr_SPEC {
    type DataType = u8;
}
#[doc = "Sub Operating Power Control Register"]
pub type Sopccr = crate::RegValueT<Sopccr_SPEC>;

impl Sopccr {
    #[doc = "Sub Operating Power Control Mode Transition Status Flag"]
    #[inline(always)]
    pub fn sopcmtsf(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, sopccr::Sopcmtsf, Sopccr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0x1,1,0,sopccr::Sopcmtsf, Sopccr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "These bits are read as 000. The write value should be 000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7, 1, 0, u8, Sopccr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7,1,0,u8, Sopccr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sub Operating Power Control Mode Select"]
    #[inline(always)]
    pub fn sopcm(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, sopccr::Sopcm, Sopccr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,sopccr::Sopcm, Sopccr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sopccr {
    #[inline(always)]
    fn default() -> Sopccr {
        <crate::RegValueT<Sopccr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sopccr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sopcmtsf_SPEC;
    pub type Sopcmtsf = crate::EnumBitfieldStruct<u8, Sopcmtsf_SPEC>;
    impl Sopcmtsf {
        #[doc = "Transition completed"]
        pub const _0: Self = Self::new(0);
        #[doc = "During transition"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sopcm_SPEC;
    pub type Sopcm = crate::EnumBitfieldStruct<u8, Sopcm_SPEC>;
    impl Sopcm {
        #[doc = "Other than Subosc-speed mode"]
        pub const _0: Self = Self::new(0);
        #[doc = "Subosc-speed mode"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syocdcr_SPEC;
impl crate::sealed::RegSpec for Syocdcr_SPEC {
    type DataType = u8;
}
#[doc = "System Control OCD Control Register"]
pub type Syocdcr = crate::RegValueT<Syocdcr_SPEC>;

impl Syocdcr {
    #[doc = "Debugger Enable bit"]
    #[inline(always)]
    pub fn dbgen(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, syocdcr::Dbgen, Syocdcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,syocdcr::Dbgen, Syocdcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, Syocdcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8, Syocdcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Syocdcr {
    #[inline(always)]
    fn default() -> Syocdcr {
        <crate::RegValueT<Syocdcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod syocdcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dbgen_SPEC;
    pub type Dbgen = crate::EnumBitfieldStruct<u8, Dbgen_SPEC>;
    impl Dbgen {
        #[doc = "On-chip debugger is disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "On-chip debugger is enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvcmpcr_SPEC;
impl crate::sealed::RegSpec for Lvcmpcr_SPEC {
    type DataType = u8;
}
#[doc = "Voltage Monitor Circuit Control Register"]
pub type Lvcmpcr = crate::RegValueT<Lvcmpcr_SPEC>;

impl Lvcmpcr {
    #[doc = "Voltage Detection 2 Enable"]
    #[inline(always)]
    pub fn lvd2e(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, lvcmpcr::Lvd2E, Lvcmpcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,lvcmpcr::Lvd2E, Lvcmpcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Voltage Detection 1 Enable"]
    #[inline(always)]
    pub fn lvd1e(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, lvcmpcr::Lvd1E, Lvcmpcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,lvcmpcr::Lvd1E, Lvcmpcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 00. The write value should be 00."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Lvcmpcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Lvcmpcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Lvcmpcr {
    #[inline(always)]
    fn default() -> Lvcmpcr {
        <crate::RegValueT<Lvcmpcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod lvcmpcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd2E_SPEC;
    pub type Lvd2E = crate::EnumBitfieldStruct<u8, Lvd2E_SPEC>;
    impl Lvd2E {
        #[doc = "Voltage detection 2 circuit disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Voltage detection 2 circuit enabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd1E_SPEC;
    pub type Lvd1E = crate::EnumBitfieldStruct<u8, Lvd1E_SPEC>;
    impl Lvd1E {
        #[doc = "Voltage detection 1 circuit disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Voltage detection 1 circuit enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvdlvlr_SPEC;
impl crate::sealed::RegSpec for Lvdlvlr_SPEC {
    type DataType = u8;
}
#[doc = "Voltage Detection Level Select Register"]
pub type Lvdlvlr = crate::RegValueT<Lvdlvlr_SPEC>;

impl Lvdlvlr {
    #[doc = "Voltage Detection 2 Level Select (Standard voltage during drop in voltage)"]
    #[inline(always)]
    pub fn lvd2lvl(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, lvdlvlr::Lvd2Lvl, Lvdlvlr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            5,
            0x7,
            1,
            0,
            lvdlvlr::Lvd2Lvl,
            Lvdlvlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Voltage Detection 1 Level Select (Standard voltage during drop in voltage)"]
    #[inline(always)]
    pub fn lvd1lvl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        lvdlvlr::Lvd1Lvl,
        Lvdlvlr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            lvdlvlr::Lvd1Lvl,
            Lvdlvlr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Lvdlvlr {
    #[inline(always)]
    fn default() -> Lvdlvlr {
        <crate::RegValueT<Lvdlvlr_SPEC> as RegisterValue<_>>::new(7)
    }
}
pub mod lvdlvlr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd2Lvl_SPEC;
    pub type Lvd2Lvl = crate::EnumBitfieldStruct<u8, Lvd2Lvl_SPEC>;
    impl Lvd2Lvl {
        #[doc = "4.29V (Vdet2_0)"]
        pub const _000: Self = Self::new(0);
        #[doc = "4.14V (Vdet2_1)"]
        pub const _001: Self = Self::new(1);
        #[doc = "4.02V (Vdet2_2)"]
        pub const _010: Self = Self::new(2);
        #[doc = "3.84V (Vdet2_3)"]
        pub const _011: Self = Self::new(3);
        #[doc = "Setting prohibited."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd1Lvl_SPEC;
    pub type Lvd1Lvl = crate::EnumBitfieldStruct<u8, Lvd1Lvl_SPEC>;
    impl Lvd1Lvl {
        #[doc = "4.29V (Vdet1_0)"]
        pub const _00000: Self = Self::new(0);
        #[doc = "4.14V (Vdet1_1)"]
        pub const _00001: Self = Self::new(1);
        #[doc = "4.02V (Vdet1_2)"]
        pub const _00010: Self = Self::new(2);
        #[doc = "3.84V (Vdet1_3)"]
        pub const _00011: Self = Self::new(3);
        #[doc = "3.10V (Vdet1_4)"]
        pub const _00100: Self = Self::new(4);
        #[doc = "3.00V (Vdet1_5)"]
        pub const _00101: Self = Self::new(5);
        #[doc = "2.90V (Vdet1_6)"]
        pub const _00110: Self = Self::new(6);
        #[doc = "2.79V (Vdet1_7)"]
        pub const _00111: Self = Self::new(7);
        #[doc = "2.68V (Vdet1_8)"]
        pub const _01000: Self = Self::new(8);
        #[doc = "2.58V (Vdet1_9)"]
        pub const _01001: Self = Self::new(9);
        #[doc = "2.48V (Vdet1_A)"]
        pub const _01010: Self = Self::new(10);
        #[doc = "2.20V (Vdet1_B)"]
        pub const _01011: Self = Self::new(11);
        #[doc = "1.96V (Vdet1_C)"]
        pub const _01100: Self = Self::new(12);
        #[doc = "1.86V (Vdet1_D)"]
        pub const _01101: Self = Self::new(13);
        #[doc = "1.75V (Vdet1_E)"]
        pub const _01110: Self = Self::new(14);
        #[doc = "1.65V (Vdet1_F)"]
        pub const _01111: Self = Self::new(15);
        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvdcr0_SPEC;
impl crate::sealed::RegSpec for Lvdcr0_SPEC {
    type DataType = u8;
}
#[doc = "Voltage Monitor %s Circuit Control Register 0"]
pub type Lvdcr0 = crate::RegValueT<Lvdcr0_SPEC>;

impl Lvdcr0 {
    #[doc = "Voltage Monitor Reset Negate Select"]
    #[inline(always)]
    pub fn rn(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, lvdcr0::Rn, Lvdcr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,lvdcr0::Rn, Lvdcr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Voltage Monitor Circuit Mode Select"]
    #[inline(always)]
    pub fn ri(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, lvdcr0::Ri, Lvdcr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,lvdcr0::Ri, Lvdcr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Voltage Monitor Circuit Comparison Result Output Enable"]
    #[inline(always)]
    pub fn cmpe(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, lvdcr0::Cmpe, Lvdcr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,lvdcr0::Cmpe, Lvdcr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Lvdcr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Lvdcr0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Voltage Monitor Interrupt/Reset Enable"]
    #[inline(always)]
    pub fn rie(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, lvdcr0::Rie, Lvdcr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,lvdcr0::Rie, Lvdcr0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Lvdcr0 {
    #[inline(always)]
    fn default() -> Lvdcr0 {
        <crate::RegValueT<Lvdcr0_SPEC> as RegisterValue<_>>::new(128)
    }
}
pub mod lvdcr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rn_SPEC;
    pub type Rn = crate::EnumBitfieldStruct<u8, Rn_SPEC>;
    impl Rn {
        #[doc = "Negation follows a stabilization time (tLVD) after VCC > Vdet1 is detected."]
        pub const _0: Self = Self::new(0);
        #[doc = "Negation follows a stabilization time (tLVD) after assertion of the LVD reset."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ri_SPEC;
    pub type Ri = crate::EnumBitfieldStruct<u8, Ri_SPEC>;
    impl Ri {
        #[doc = "Voltage Monitor  interrupt during Vdet1 passage"]
        pub const _0: Self = Self::new(0);
        #[doc = "Voltage Monitor  reset enabled when the voltage falls to and below Vdet1"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpe_SPEC;
    pub type Cmpe = crate::EnumBitfieldStruct<u8, Cmpe_SPEC>;
    impl Cmpe {
        #[doc = "Voltage Monitor circuit comparison result output disabled."]
        pub const _0: Self = Self::new(0);
        #[doc = "Voltage Monitor circuit comparison result output enabled."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rie_SPEC;
    pub type Rie = crate::EnumBitfieldStruct<u8, Rie_SPEC>;
    impl Rie {
        #[doc = "Disabled"]
        pub const _0: Self = Self::new(0);
        #[doc = "Enabled"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvdcr1_SPEC;
impl crate::sealed::RegSpec for Lvdcr1_SPEC {
    type DataType = u8;
}
#[doc = "Voltage Monitor %s Circuit Control Register 1"]
pub type Lvdcr1 = crate::RegValueT<Lvdcr1_SPEC>;

impl Lvdcr1 {
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, Lvdcr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8, Lvdcr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Voltage Monitor Interrupt Type Select"]
    #[inline(always)]
    pub fn irqsel(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, lvdcr1::Irqsel, Lvdcr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,lvdcr1::Irqsel, Lvdcr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Voltage Monitor Interrupt Generation Condition Select"]
    #[inline(always)]
    pub fn idtsel(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, lvdcr1::Idtsel, Lvdcr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,lvdcr1::Idtsel, Lvdcr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Lvdcr1 {
    #[inline(always)]
    fn default() -> Lvdcr1 {
        <crate::RegValueT<Lvdcr1_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod lvdcr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Irqsel_SPEC;
    pub type Irqsel = crate::EnumBitfieldStruct<u8, Irqsel_SPEC>;
    impl Irqsel {
        #[doc = "Non-maskable interrupt"]
        pub const _0: Self = Self::new(0);
        #[doc = "Maskable interrupt"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Idtsel_SPEC;
    pub type Idtsel = crate::EnumBitfieldStruct<u8, Idtsel_SPEC>;
    impl Idtsel {
        #[doc = "When VCC>=Vdet (rise) is detected"]
        pub const _00: Self = Self::new(0);
        #[doc = "When VCC<Vdet (drop) is detected"]
        pub const _01: Self = Self::new(1);
        #[doc = "When drop and rise are detected"]
        pub const _10: Self = Self::new(2);
        #[doc = "Settings prohibited"]
        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvdsr_SPEC;
impl crate::sealed::RegSpec for Lvdsr_SPEC {
    type DataType = u8;
}
#[doc = "Voltage Monitor %s Circuit Status Register"]
pub type Lvdsr = crate::RegValueT<Lvdsr_SPEC>;

impl Lvdsr {
    #[doc = "These bits are read as 000000. The write value should be 000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, Lvdsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3f,1,0,u8, Lvdsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Voltage Monitor 1 Signal Monitor Flag"]
    #[inline(always)]
    pub fn mon(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, lvdsr::Mon, Lvdsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,lvdsr::Mon, Lvdsr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Voltage Monitor Voltage Change Detection Flag \nNOTE: Only 0 can be written to this bit. After writing 0 to this bit, it takes 2 system clock cycles for the bit to be read as 0."]
    #[inline(always)]
    pub fn det(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, lvdsr::Det, Lvdsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,lvdsr::Det, Lvdsr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Lvdsr {
    #[inline(always)]
    fn default() -> Lvdsr {
        <crate::RegValueT<Lvdsr_SPEC> as RegisterValue<_>>::new(2)
    }
}
pub mod lvdsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mon_SPEC;
    pub type Mon = crate::EnumBitfieldStruct<u8, Mon_SPEC>;
    impl Mon {
        #[doc = "VCC < Vdet"]
        pub const _0: Self = Self::new(0);
        #[doc = "VCC >= Vdet or MON bit is disabled"]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Det_SPEC;
    pub type Det = crate::EnumBitfieldStruct<u8, Det_SPEC>;
    impl Det {
        #[doc = "Not detected"]
        pub const _0: Self = Self::new(0);
        #[doc = "Vdet1 passage detection"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prcr_SPEC;
impl crate::sealed::RegSpec for Prcr_SPEC {
    type DataType = u16;
}
#[doc = "Protect Register"]
pub type Prcr = crate::RegValueT<Prcr_SPEC>;

impl Prcr {
    #[doc = "PRC Key Code"]
    #[inline(always)]
    pub fn prkey(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, prcr::Prkey, Prcr_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xff,1,0,prcr::Prkey, Prcr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Protect Bit 3"]
    #[inline(always)]
    pub fn prc3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, prcr::Prc3, Prcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,prcr::Prc3, Prcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "This bit is read as 0. The write value should be 0."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Prcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Prcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Protect Bit 1"]
    #[inline(always)]
    pub fn prc1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, prcr::Prc1, Prcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,prcr::Prc1, Prcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Protect Bit 0"]
    #[inline(always)]
    pub fn prc0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, prcr::Prc0, Prcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,prcr::Prc0, Prcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Prcr {
    #[inline(always)]
    fn default() -> Prcr {
        <crate::RegValueT<Prcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod prcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prkey_SPEC;
    pub type Prkey = crate::EnumBitfieldStruct<u8, Prkey_SPEC>;
    impl Prkey {
        #[doc = "Enables writing to the PRCR register."]
        pub const _0_X_5_A: Self = Self::new(90);
        #[doc = "Disables writing to the PRCR register."]
        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prc3_SPEC;
    pub type Prc3 = crate::EnumBitfieldStruct<u8, Prc3_SPEC>;
    impl Prc3 {
        #[doc = "Writes protected."]
        pub const _0: Self = Self::new(0);
        #[doc = "Writes not protected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prc1_SPEC;
    pub type Prc1 = crate::EnumBitfieldStruct<u8, Prc1_SPEC>;
    impl Prc1 {
        #[doc = "Writes protected."]
        pub const _0: Self = Self::new(0);
        #[doc = "Writes not protected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Prc0_SPEC;
    pub type Prc0 = crate::EnumBitfieldStruct<u8, Prc0_SPEC>;
    impl Prc0 {
        #[doc = "Writes protected."]
        pub const _0: Self = Self::new(0);
        #[doc = "Writes not protected."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rstsr0_SPEC;
impl crate::sealed::RegSpec for Rstsr0_SPEC {
    type DataType = u8;
}
#[doc = "Reset Status Register 0"]
pub type Rstsr0 = crate::RegValueT<Rstsr0_SPEC>;

impl Rstsr0 {
    #[doc = "These bits are read as 0000. The write value should be 0000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Rstsr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Rstsr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Voltage Monitor 2 Reset Detect Flag\nNote: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn lvd2rf(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, rstsr0::Lvd2Rf, Rstsr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,rstsr0::Lvd2Rf, Rstsr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Voltage Monitor 1 Reset Detect Flag\nNote: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn lvd1rf(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, rstsr0::Lvd1Rf, Rstsr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,rstsr0::Lvd1Rf, Rstsr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Voltage Monitor 0 Reset Detect Flag\nNote: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn lvd0rf(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, rstsr0::Lvd0Rf, Rstsr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,rstsr0::Lvd0Rf, Rstsr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Power-On Reset Detect Flag\nNote: Only 0 can be written to clear the flag. The reset flag must be written with 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn porf(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, rstsr0::Porf, Rstsr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,rstsr0::Porf, Rstsr0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rstsr0 {
    #[inline(always)]
    fn default() -> Rstsr0 {
        <crate::RegValueT<Rstsr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rstsr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd2Rf_SPEC;
    pub type Lvd2Rf = crate::EnumBitfieldStruct<u8, Lvd2Rf_SPEC>;
    impl Lvd2Rf {
        #[doc = "Voltage Monitor 2 reset not detected."]
        pub const _0: Self = Self::new(0);
        #[doc = "Voltage Monitor 2 reset detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd1Rf_SPEC;
    pub type Lvd1Rf = crate::EnumBitfieldStruct<u8, Lvd1Rf_SPEC>;
    impl Lvd1Rf {
        #[doc = "Voltage Monitor 1 reset not detected."]
        pub const _0: Self = Self::new(0);
        #[doc = "Voltage Monitor 1 reset detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvd0Rf_SPEC;
    pub type Lvd0Rf = crate::EnumBitfieldStruct<u8, Lvd0Rf_SPEC>;
    impl Lvd0Rf {
        #[doc = "Voltage Monitor 0 reset not detected."]
        pub const _0: Self = Self::new(0);
        #[doc = "Voltage Monitor 0 reset detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Porf_SPEC;
    pub type Porf = crate::EnumBitfieldStruct<u8, Porf_SPEC>;
    impl Porf {
        #[doc = "Power-on reset not detected."]
        pub const _0: Self = Self::new(0);
        #[doc = "Power-on reset detected."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rstsr2_SPEC;
impl crate::sealed::RegSpec for Rstsr2_SPEC {
    type DataType = u8;
}
#[doc = "Reset Status Register 2"]
pub type Rstsr2 = crate::RegValueT<Rstsr2_SPEC>;

impl Rstsr2 {
    #[doc = "These bits are read as 0000000. The write value should be 0000000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, Rstsr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8, Rstsr2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Cold/Warm Start Determination Flag\nNote: Only 1 can be written to set the flag."]
    #[inline(always)]
    pub fn cwsf(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, rstsr2::Cwsf, Rstsr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,rstsr2::Cwsf, Rstsr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rstsr2 {
    #[inline(always)]
    fn default() -> Rstsr2 {
        <crate::RegValueT<Rstsr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rstsr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cwsf_SPEC;
    pub type Cwsf = crate::EnumBitfieldStruct<u8, Cwsf_SPEC>;
    impl Cwsf {
        #[doc = "Cold start"]
        pub const _0: Self = Self::new(0);
        #[doc = "Warm start"]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rstsr1_SPEC;
impl crate::sealed::RegSpec for Rstsr1_SPEC {
    type DataType = u16;
}
#[doc = "Reset Status Register 1"]
pub type Rstsr1 = crate::RegValueT<Rstsr1_SPEC>;

impl Rstsr1 {
    #[doc = "SP Error Reset Detect Flag\nNote: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn sperf(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, rstsr1::Sperf, Rstsr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,rstsr1::Sperf, Rstsr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bus Master MPU Reset Detect Flag\nNote: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn busmrf(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, rstsr1::Busmrf, Rstsr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,rstsr1::Busmrf, Rstsr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bus Slave MPU Reset Detect Flag\nNote: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn bussrf(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, rstsr1::Bussrf, Rstsr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,rstsr1::Bussrf, Rstsr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "RAM ECC Error Reset Detect Flag\nNote: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn reerf(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, rstsr1::Reerf, Rstsr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,rstsr1::Reerf, Rstsr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "RAM Parity Error Reset Detect Flag\nNote: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn rperf(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, rstsr1::Rperf, Rstsr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,rstsr1::Rperf, Rstsr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, Rstsr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8, Rstsr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Software Reset Detect Flag\nNote: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn swrf(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, rstsr1::Swrf, Rstsr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,rstsr1::Swrf, Rstsr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Watchdog Timer Reset Detect Flag\nNote: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn wdtrf(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, rstsr1::Wdtrf, Rstsr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,rstsr1::Wdtrf, Rstsr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Independent Watchdog Timer Reset Detect Flag\nNote: Only 0 can be written to clear the flag. The reset flag must be written as 0 after the reset flag is read as 1."]
    #[inline(always)]
    pub fn iwdtrf(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, rstsr1::Iwdtrf, Rstsr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,rstsr1::Iwdtrf, Rstsr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Rstsr1 {
    #[inline(always)]
    fn default() -> Rstsr1 {
        <crate::RegValueT<Rstsr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rstsr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sperf_SPEC;
    pub type Sperf = crate::EnumBitfieldStruct<u8, Sperf_SPEC>;
    impl Sperf {
        #[doc = "SP error reset not detected."]
        pub const _0: Self = Self::new(0);
        #[doc = "SP error reset detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Busmrf_SPEC;
    pub type Busmrf = crate::EnumBitfieldStruct<u8, Busmrf_SPEC>;
    impl Busmrf {
        #[doc = "Bus Master MPU reset not detected."]
        pub const _0: Self = Self::new(0);
        #[doc = "Bus Master MPU reset detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bussrf_SPEC;
    pub type Bussrf = crate::EnumBitfieldStruct<u8, Bussrf_SPEC>;
    impl Bussrf {
        #[doc = "Bus Slave MPU reset not detected."]
        pub const _0: Self = Self::new(0);
        #[doc = "Bus Slave MPU reset detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Reerf_SPEC;
    pub type Reerf = crate::EnumBitfieldStruct<u8, Reerf_SPEC>;
    impl Reerf {
        #[doc = "RAM ECC error reset not detected."]
        pub const _0: Self = Self::new(0);
        #[doc = "RAM ECC error reset detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rperf_SPEC;
    pub type Rperf = crate::EnumBitfieldStruct<u8, Rperf_SPEC>;
    impl Rperf {
        #[doc = "RAM parity error reset not detected."]
        pub const _0: Self = Self::new(0);
        #[doc = "RAM parity error reset detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Swrf_SPEC;
    pub type Swrf = crate::EnumBitfieldStruct<u8, Swrf_SPEC>;
    impl Swrf {
        #[doc = "Software reset not detected."]
        pub const _0: Self = Self::new(0);
        #[doc = "Software reset detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wdtrf_SPEC;
    pub type Wdtrf = crate::EnumBitfieldStruct<u8, Wdtrf_SPEC>;
    impl Wdtrf {
        #[doc = "Watchdog timer reset not detected."]
        pub const _0: Self = Self::new(0);
        #[doc = "Watchdog timer reset detected."]
        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iwdtrf_SPEC;
    pub type Iwdtrf = crate::EnumBitfieldStruct<u8, Iwdtrf_SPEC>;
    impl Iwdtrf {
        #[doc = "Independent watchdog timer reset not detected."]
        pub const _0: Self = Self::new(0);
        #[doc = "Independent watchdog timer reset detected."]
        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bkracr_SPEC;
impl crate::sealed::RegSpec for Bkracr_SPEC {
    type DataType = u8;
}
#[doc = "Backup Register Access Control Register"]
pub type Bkracr = crate::RegValueT<Bkracr_SPEC>;

impl Bkracr {
    #[doc = "These bits are read as 00000. The write value should be 00000."]
    #[inline(always)]
    pub fn reserved(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, Bkracr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8, Bkracr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Backup Register Access Control Register"]
    #[inline(always)]
    pub fn bkracs(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, bkracr::Bkracs, Bkracr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,bkracr::Bkracs, Bkracr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bkracr {
    #[inline(always)]
    fn default() -> Bkracr {
        <crate::RegValueT<Bkracr_SPEC> as RegisterValue<_>>::new(6)
    }
}
pub mod bkracr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bkracs_SPEC;
    pub type Bkracs = crate::EnumBitfieldStruct<u8, Bkracs_SPEC>;
    impl Bkracs {
        #[doc = "Access control disable. When System clock source is SOSC or LOCO."]
        pub const _000: Self = Self::new(0);
        #[doc = "Access control enable. System clock source is other than SOSC or LOCO."]
        pub const _110: Self = Self::new(6);
        #[doc = "Setting prohibited"]
        pub const OTHERS: Self = Self::new(0);
    }
}
