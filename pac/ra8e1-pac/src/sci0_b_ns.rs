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
// Generated from SVD 1.00.01, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:22:22 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Serial Communication Interface 0"]
unsafe impl ::core::marker::Send for super::Sci0BNs {}
unsafe impl ::core::marker::Sync for super::Sci0BNs {}
impl super::Sci0BNs {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn rdr(&self) -> &'static crate::common::Reg<self::Rdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Rdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rdr_by(&self) -> &'static crate::common::Reg<self::RdrBy_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::RdrBy_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tdr(&self) -> &'static crate::common::Reg<self::Tdr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tdr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tdrll(&self) -> &'static crate::common::Reg<self::Tdrll_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tdrll_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn tdrlh(&self) -> &'static crate::common::Reg<self::Tdrlh_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Tdrlh_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(5usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ccr0(&self) -> &'static crate::common::Reg<self::Ccr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ccr1(&self) -> &'static crate::common::Reg<self::Ccr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ccr2(&self) -> &'static crate::common::Reg<self::Ccr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ccr3(&self) -> &'static crate::common::Reg<self::Ccr3_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr3_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ccr4(&self) -> &'static crate::common::Reg<self::Ccr4_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ccr4_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cesr(&self) -> &'static crate::common::Reg<self::Cesr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Cesr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn icr(&self) -> &'static crate::common::Reg<self::Icr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Icr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn fcr(&self) -> &'static crate::common::Reg<self::Fcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Fcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mcr(&self) -> &'static crate::common::Reg<self::Mcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Mcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dcr(&self) -> &'static crate::common::Reg<self::Dcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn xcr0(&self) -> &'static crate::common::Reg<self::Xcr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xcr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[inline(always)]
    pub const fn xcr1(&self) -> &'static crate::common::Reg<self::Xcr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xcr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[inline(always)]
    pub const fn xcr2(&self) -> &'static crate::common::Reg<self::Xcr2_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xcr2_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[inline(always)]
    pub const fn csr(&self) -> &'static crate::common::Reg<self::Csr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Csr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[inline(always)]
    pub const fn isr(&self) -> &'static crate::common::Reg<self::Isr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Isr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[inline(always)]
    pub const fn frsr(&self) -> &'static crate::common::Reg<self::Frsr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Frsr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ftsr(&self) -> &'static crate::common::Reg<self::Ftsr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ftsr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[inline(always)]
    pub const fn msr(&self) -> &'static crate::common::Reg<self::Msr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Msr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[inline(always)]
    pub const fn xsr0(&self) -> &'static crate::common::Reg<self::Xsr0_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Xsr0_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[inline(always)]
    pub const fn xsr1(&self) -> &'static crate::common::Reg<self::Xsr1_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Xsr1_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[inline(always)]
    pub const fn cfclr(&self) -> &'static crate::common::Reg<self::Cfclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Cfclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[inline(always)]
    pub const fn icfclr(&self) -> &'static crate::common::Reg<self::Icfclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Icfclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ffclr(&self) -> &'static crate::common::Reg<self::Ffclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Ffclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[inline(always)]
    pub const fn mfclr(&self) -> &'static crate::common::Reg<self::Mfclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Mfclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[inline(always)]
    pub const fn xfclr(&self) -> &'static crate::common::Reg<self::Xfclr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Xfclr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdr_SPEC;
impl crate::sealed::RegSpec for Rdr_SPEC {
    type DataType = u32;
}

pub type Rdr = crate::RegValueT<Rdr_SPEC>;

impl Rdr {
    #[inline(always)]
    pub fn rdat(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Rdr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mpb(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, rdr::Mpb, rdr::Mpb, Rdr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<9,0x1,1,0,rdr::Mpb,rdr::Mpb,Rdr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dr(self) -> crate::common::RegisterFieldBool<10, 1, 0, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Rdr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn fper(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, rdr::Fper, rdr::Fper, Rdr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            rdr::Fper,
            rdr::Fper,
            Rdr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ffer(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, rdr::Ffer, rdr::Ffer, Rdr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            rdr::Ffer,
            rdr::Ffer,
            Rdr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn orer(self) -> crate::common::RegisterFieldBool<24, 1, 0, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Rdr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn per(self) -> crate::common::RegisterFieldBool<27, 1, 0, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Rdr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn fer(self) -> crate::common::RegisterFieldBool<28, 1, 0, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Rdr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Rdr {
    #[inline(always)]
    fn default() -> Rdr {
        <crate::RegValueT<Rdr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rdr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpb_SPEC;
    pub type Mpb = crate::EnumBitfieldStruct<u8, Mpb_SPEC>;
    impl Mpb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fper_SPEC;
    pub type Fper = crate::EnumBitfieldStruct<u8, Fper_SPEC>;
    impl Fper {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ffer_SPEC;
    pub type Ffer = crate::EnumBitfieldStruct<u8, Ffer_SPEC>;
    impl Ffer {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RdrBy_SPEC;
impl crate::sealed::RegSpec for RdrBy_SPEC {
    type DataType = u8;
}

pub type RdrBy = crate::RegValueT<RdrBy_SPEC>;

impl RdrBy {
    #[inline(always)]
    pub fn orer(self) -> crate::common::RegisterFieldBool<0, 1, 0, RdrBy_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, RdrBy_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn per(self) -> crate::common::RegisterFieldBool<3, 1, 0, RdrBy_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, RdrBy_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn fer(self) -> crate::common::RegisterFieldBool<4, 1, 0, RdrBy_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, RdrBy_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for RdrBy {
    #[inline(always)]
    fn default() -> RdrBy {
        <crate::RegValueT<RdrBy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdr_SPEC;
impl crate::sealed::RegSpec for Tdr_SPEC {
    type DataType = u32;
}

pub type Tdr = crate::RegValueT<Tdr_SPEC>;

impl Tdr {
    #[inline(always)]
    pub fn tdat(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Tdr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Tdr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mpbt(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, tdr::Mpbt, tdr::Mpbt, Tdr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            tdr::Mpbt,
            tdr::Mpbt,
            Tdr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tsync(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        tdr::Tsync,
        tdr::Tsync,
        Tdr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            tdr::Tsync,
            tdr::Tsync,
            Tdr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Tdr {
    #[inline(always)]
    fn default() -> Tdr {
        <crate::RegValueT<Tdr_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod tdr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpbt_SPEC;
    pub type Mpbt = crate::EnumBitfieldStruct<u8, Mpbt_SPEC>;
    impl Mpbt {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsync_SPEC;
    pub type Tsync = crate::EnumBitfieldStruct<u8, Tsync_SPEC>;
    impl Tsync {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdrll_SPEC;
impl crate::sealed::RegSpec for Tdrll_SPEC {
    type DataType = u16;
}

pub type Tdrll = crate::RegValueT<Tdrll_SPEC>;

impl NoBitfieldReg<Tdrll_SPEC> for Tdrll {}
impl ::core::default::Default for Tdrll {
    #[inline(always)]
    fn default() -> Tdrll {
        <crate::RegValueT<Tdrll_SPEC> as RegisterValue<_>>::new(65535)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdrlh_SPEC;
impl crate::sealed::RegSpec for Tdrlh_SPEC {
    type DataType = u16;
}

pub type Tdrlh = crate::RegValueT<Tdrlh_SPEC>;

impl Tdrlh {
    #[inline(always)]
    pub fn tdat(self) -> crate::common::RegisterFieldBool<0, 1, 0, Tdrlh_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Tdrlh_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn mpbt(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        tdrlh::Mpbt,
        tdrlh::Mpbt,
        Tdrlh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            tdrlh::Mpbt,
            tdrlh::Mpbt,
            Tdrlh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tsync(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        tdrlh::Tsync,
        tdrlh::Tsync,
        Tdrlh_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            tdrlh::Tsync,
            tdrlh::Tsync,
            Tdrlh_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Tdrlh {
    #[inline(always)]
    fn default() -> Tdrlh {
        <crate::RegValueT<Tdrlh_SPEC> as RegisterValue<_>>::new(65535)
    }
}
pub mod tdrlh {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpbt_SPEC;
    pub type Mpbt = crate::EnumBitfieldStruct<u8, Mpbt_SPEC>;
    impl Mpbt {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tsync_SPEC;
    pub type Tsync = crate::EnumBitfieldStruct<u8, Tsync_SPEC>;
    impl Tsync {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr0_SPEC;
impl crate::sealed::RegSpec for Ccr0_SPEC {
    type DataType = u32;
}

pub type Ccr0 = crate::RegValueT<Ccr0_SPEC>;

impl Ccr0 {
    #[inline(always)]
    pub fn re(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, ccr0::Re, ccr0::Re, Ccr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,ccr0::Re,ccr0::Re,Ccr0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn te(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, ccr0::Te, ccr0::Te, Ccr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,ccr0::Te,ccr0::Te,Ccr0_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mpie(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        ccr0::Mpie,
        ccr0::Mpie,
        Ccr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            ccr0::Mpie,
            ccr0::Mpie,
            Ccr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dcme(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ccr0::Dcme,
        ccr0::Dcme,
        Ccr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ccr0::Dcme,
            ccr0::Dcme,
            Ccr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn idsel(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        ccr0::Idsel,
        ccr0::Idsel,
        Ccr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            ccr0::Idsel,
            ccr0::Idsel,
            Ccr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rie(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ccr0::Rie,
        ccr0::Rie,
        Ccr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ccr0::Rie,
            ccr0::Rie,
            Ccr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tie(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        ccr0::Tie,
        ccr0::Tie,
        Ccr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            ccr0::Tie,
            ccr0::Tie,
            Ccr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn teie(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        ccr0::Teie,
        ccr0::Teie,
        Ccr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            ccr0::Teie,
            ccr0::Teie,
            Ccr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sse(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        ccr0::Sse,
        ccr0::Sse,
        Ccr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            ccr0::Sse,
            ccr0::Sse,
            Ccr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ccr0 {
    #[inline(always)]
    fn default() -> Ccr0 {
        <crate::RegValueT<Ccr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ccr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Re_SPEC;
    pub type Re = crate::EnumBitfieldStruct<u8, Re_SPEC>;
    impl Re {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Te_SPEC;
    pub type Te = crate::EnumBitfieldStruct<u8, Te_SPEC>;
    impl Te {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mpie_SPEC;
    pub type Mpie = crate::EnumBitfieldStruct<u8, Mpie_SPEC>;
    impl Mpie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcme_SPEC;
    pub type Dcme = crate::EnumBitfieldStruct<u8, Dcme_SPEC>;
    impl Dcme {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Idsel_SPEC;
    pub type Idsel = crate::EnumBitfieldStruct<u8, Idsel_SPEC>;
    impl Idsel {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rie_SPEC;
    pub type Rie = crate::EnumBitfieldStruct<u8, Rie_SPEC>;
    impl Rie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tie_SPEC;
    pub type Tie = crate::EnumBitfieldStruct<u8, Tie_SPEC>;
    impl Tie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Teie_SPEC;
    pub type Teie = crate::EnumBitfieldStruct<u8, Teie_SPEC>;
    impl Teie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sse_SPEC;
    pub type Sse = crate::EnumBitfieldStruct<u8, Sse_SPEC>;
    impl Sse {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr1_SPEC;
impl crate::sealed::RegSpec for Ccr1_SPEC {
    type DataType = u32;
}

pub type Ccr1 = crate::RegValueT<Ccr1_SPEC>;

impl Ccr1 {
    #[inline(always)]
    pub fn ctse(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ccr1::Ctse,
        ccr1::Ctse,
        Ccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ccr1::Ctse,
            ccr1::Ctse,
            Ccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ctspen(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ccr1::Ctspen,
        ccr1::Ctspen,
        Ccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ccr1::Ctspen,
            ccr1::Ctspen,
            Ccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spb2dt(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ccr1::Spb2Dt,
        ccr1::Spb2Dt,
        Ccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ccr1::Spb2Dt,
            ccr1::Spb2Dt,
            Ccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spb2io(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ccr1::Spb2Io,
        ccr1::Spb2Io,
        Ccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ccr1::Spb2Io,
            ccr1::Spb2Io,
            Ccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pe(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, ccr1::Pe, ccr1::Pe, Ccr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,ccr1::Pe,ccr1::Pe,Ccr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pm(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, ccr1::Pm, ccr1::Pm, Ccr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,ccr1::Pm,ccr1::Pm,Ccr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tinv(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        ccr1::Tinv,
        ccr1::Tinv,
        Ccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            ccr1::Tinv,
            ccr1::Tinv,
            Ccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rinv(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        ccr1::Rinv,
        ccr1::Rinv,
        Ccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            ccr1::Rinv,
            ccr1::Rinv,
            Ccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn splp(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ccr1::Splp,
        ccr1::Splp,
        Ccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ccr1::Splp,
            ccr1::Splp,
            Ccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sharps(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        ccr1::Sharps,
        ccr1::Sharps,
        Ccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            ccr1::Sharps,
            ccr1::Sharps,
            Ccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn nfcs(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        ccr1::Nfcs,
        ccr1::Nfcs,
        Ccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            ccr1::Nfcs,
            ccr1::Nfcs,
            Ccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn nfen(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        ccr1::Nfen,
        ccr1::Nfen,
        Ccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            ccr1::Nfen,
            ccr1::Nfen,
            Ccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn nfm(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        ccr1::Nfm,
        ccr1::Nfm,
        Ccr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            ccr1::Nfm,
            ccr1::Nfm,
            Ccr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ccr1 {
    #[inline(always)]
    fn default() -> Ccr1 {
        <crate::RegValueT<Ccr1_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod ccr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctse_SPEC;
    pub type Ctse = crate::EnumBitfieldStruct<u8, Ctse_SPEC>;
    impl Ctse {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctspen_SPEC;
    pub type Ctspen = crate::EnumBitfieldStruct<u8, Ctspen_SPEC>;
    impl Ctspen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spb2Dt_SPEC;
    pub type Spb2Dt = crate::EnumBitfieldStruct<u8, Spb2Dt_SPEC>;
    impl Spb2Dt {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spb2Io_SPEC;
    pub type Spb2Io = crate::EnumBitfieldStruct<u8, Spb2Io_SPEC>;
    impl Spb2Io {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pe_SPEC;
    pub type Pe = crate::EnumBitfieldStruct<u8, Pe_SPEC>;
    impl Pe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pm_SPEC;
    pub type Pm = crate::EnumBitfieldStruct<u8, Pm_SPEC>;
    impl Pm {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tinv_SPEC;
    pub type Tinv = crate::EnumBitfieldStruct<u8, Tinv_SPEC>;
    impl Tinv {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rinv_SPEC;
    pub type Rinv = crate::EnumBitfieldStruct<u8, Rinv_SPEC>;
    impl Rinv {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Splp_SPEC;
    pub type Splp = crate::EnumBitfieldStruct<u8, Splp_SPEC>;
    impl Splp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sharps_SPEC;
    pub type Sharps = crate::EnumBitfieldStruct<u8, Sharps_SPEC>;
    impl Sharps {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfcs_SPEC;
    pub type Nfcs = crate::EnumBitfieldStruct<u8, Nfcs_SPEC>;
    impl Nfcs {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _100: Self = Self::new(4);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfen_SPEC;
    pub type Nfen = crate::EnumBitfieldStruct<u8, Nfen_SPEC>;
    impl Nfen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Nfm_SPEC;
    pub type Nfm = crate::EnumBitfieldStruct<u8, Nfm_SPEC>;
    impl Nfm {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr2_SPEC;
impl crate::sealed::RegSpec for Ccr2_SPEC {
    type DataType = u32;
}

pub type Ccr2 = crate::RegValueT<Ccr2_SPEC>;

impl Ccr2 {
    #[inline(always)]
    pub fn bcp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        ccr2::Bcp,
        ccr2::Bcp,
        Ccr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            ccr2::Bcp,
            ccr2::Bcp,
            Ccr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bgdm(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ccr2::Bgdm,
        ccr2::Bgdm,
        Ccr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ccr2::Bgdm,
            ccr2::Bgdm,
            Ccr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn abcs(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ccr2::Abcs,
        ccr2::Abcs,
        Ccr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ccr2::Abcs,
            ccr2::Abcs,
            Ccr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn abcse(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        ccr2::Abcse,
        ccr2::Abcse,
        Ccr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ccr2::Abcse,
            ccr2::Abcse,
            Ccr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn abcse2(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ccr2::Abcse2,
        ccr2::Abcse2,
        Ccr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ccr2::Abcse2,
            ccr2::Abcse2,
            Ccr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn brr(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Ccr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Ccr2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn brme(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ccr2::Brme,
        ccr2::Brme,
        Ccr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ccr2::Brme,
            ccr2::Brme,
            Ccr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cks(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x3,
        1,
        0,
        ccr2::Cks,
        ccr2::Cks,
        Ccr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x3,
            1,
            0,
            ccr2::Cks,
            ccr2::Cks,
            Ccr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mddr(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Ccr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Ccr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ccr2 {
    #[inline(always)]
    fn default() -> Ccr2 {
        <crate::RegValueT<Ccr2_SPEC> as RegisterValue<_>>::new(4278255364)
    }
}
pub mod ccr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bcp_SPEC;
    pub type Bcp = crate::EnumBitfieldStruct<u8, Bcp_SPEC>;
    impl Bcp {
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
    pub struct Bgdm_SPEC;
    pub type Bgdm = crate::EnumBitfieldStruct<u8, Bgdm_SPEC>;
    impl Bgdm {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Abcs_SPEC;
    pub type Abcs = crate::EnumBitfieldStruct<u8, Abcs_SPEC>;
    impl Abcs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Abcse_SPEC;
    pub type Abcse = crate::EnumBitfieldStruct<u8, Abcse_SPEC>;
    impl Abcse {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Abcse2_SPEC;
    pub type Abcse2 = crate::EnumBitfieldStruct<u8, Abcse2_SPEC>;
    impl Abcse2 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Brme_SPEC;
    pub type Brme = crate::EnumBitfieldStruct<u8, Brme_SPEC>;
    impl Brme {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cks_SPEC;
    pub type Cks = crate::EnumBitfieldStruct<u8, Cks_SPEC>;
    impl Cks {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr3_SPEC;
impl crate::sealed::RegSpec for Ccr3_SPEC {
    type DataType = u32;
}

pub type Ccr3 = crate::RegValueT<Ccr3_SPEC>;

impl Ccr3 {
    #[inline(always)]
    pub fn cpha(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ccr3::Cpha,
        ccr3::Cpha,
        Ccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ccr3::Cpha,
            ccr3::Cpha,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cpol(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ccr3::Cpol,
        ccr3::Cpol,
        Ccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ccr3::Cpol,
            ccr3::Cpol,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bpen(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ccr3::Bpen,
        ccr3::Bpen,
        Ccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ccr3::Bpen,
            ccr3::Bpen,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn chr(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        ccr3::Chr,
        ccr3::Chr,
        Ccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            ccr3::Chr,
            ccr3::Chr,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lsbf(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        ccr3::Lsbf,
        ccr3::Lsbf,
        Ccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            ccr3::Lsbf,
            ccr3::Lsbf,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sinv(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        ccr3::Sinv,
        ccr3::Sinv,
        Ccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            ccr3::Sinv,
            ccr3::Sinv,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn stp(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        ccr3::Stp,
        ccr3::Stp,
        Ccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            ccr3::Stp,
            ccr3::Stp,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rxdesel(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        ccr3::Rxdesel,
        ccr3::Rxdesel,
        Ccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            ccr3::Rxdesel,
            ccr3::Rxdesel,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn r#mod(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        ccr3::Mod,
        ccr3::Mod,
        Ccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            ccr3::Mod,
            ccr3::Mod,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mp(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, ccr3::Mp, ccr3::Mp, Ccr3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            ccr3::Mp,
            ccr3::Mp,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fm(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, ccr3::Fm, ccr3::Fm, Ccr3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            ccr3::Fm,
            ccr3::Fm,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn den(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        ccr3::Den,
        ccr3::Den,
        Ccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            ccr3::Den,
            ccr3::Den,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cke(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, u8, Ccr3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3,1,0,u8,u8,Ccr3_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn acs0(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        ccr3::Acs0,
        ccr3::Acs0,
        Ccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            ccr3::Acs0,
            ccr3::Acs0,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gm(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, ccr3::Gm, ccr3::Gm, Ccr3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            ccr3::Gm,
            ccr3::Gm,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn blk(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        ccr3::Blk,
        ccr3::Blk,
        Ccr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            ccr3::Blk,
            ccr3::Blk,
            Ccr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ccr3 {
    #[inline(always)]
    fn default() -> Ccr3 {
        <crate::RegValueT<Ccr3_SPEC> as RegisterValue<_>>::new(4611)
    }
}
pub mod ccr3 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpha_SPEC;
    pub type Cpha = crate::EnumBitfieldStruct<u8, Cpha_SPEC>;
    impl Cpha {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpol_SPEC;
    pub type Cpol = crate::EnumBitfieldStruct<u8, Cpol_SPEC>;
    impl Cpol {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bpen_SPEC;
    pub type Bpen = crate::EnumBitfieldStruct<u8, Bpen_SPEC>;
    impl Bpen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Chr_SPEC;
    pub type Chr = crate::EnumBitfieldStruct<u8, Chr_SPEC>;
    impl Chr {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lsbf_SPEC;
    pub type Lsbf = crate::EnumBitfieldStruct<u8, Lsbf_SPEC>;
    impl Lsbf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sinv_SPEC;
    pub type Sinv = crate::EnumBitfieldStruct<u8, Sinv_SPEC>;
    impl Sinv {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Stp_SPEC;
    pub type Stp = crate::EnumBitfieldStruct<u8, Stp_SPEC>;
    impl Stp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxdesel_SPEC;
    pub type Rxdesel = crate::EnumBitfieldStruct<u8, Rxdesel_SPEC>;
    impl Rxdesel {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mod_SPEC;
    pub type Mod = crate::EnumBitfieldStruct<u8, Mod_SPEC>;
    impl Mod {
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
    pub struct Mp_SPEC;
    pub type Mp = crate::EnumBitfieldStruct<u8, Mp_SPEC>;
    impl Mp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fm_SPEC;
    pub type Fm = crate::EnumBitfieldStruct<u8, Fm_SPEC>;
    impl Fm {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Den_SPEC;
    pub type Den = crate::EnumBitfieldStruct<u8, Den_SPEC>;
    impl Den {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Acs0_SPEC;
    pub type Acs0 = crate::EnumBitfieldStruct<u8, Acs0_SPEC>;
    impl Acs0 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gm_SPEC;
    pub type Gm = crate::EnumBitfieldStruct<u8, Gm_SPEC>;
    impl Gm {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Blk_SPEC;
    pub type Blk = crate::EnumBitfieldStruct<u8, Blk_SPEC>;
    impl Blk {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr4_SPEC;
impl crate::sealed::RegSpec for Ccr4_SPEC {
    type DataType = u32;
}

pub type Ccr4 = crate::RegValueT<Ccr4_SPEC>;

impl Ccr4 {
    #[inline(always)]
    pub fn cmpd(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, Ccr4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,Ccr4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn asen(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ccr4::Asen,
        ccr4::Asen,
        Ccr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ccr4::Asen,
            ccr4::Asen,
            Ccr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn aten(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        ccr4::Aten,
        ccr4::Aten,
        Ccr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            ccr4::Aten,
            ccr4::Aten,
            Ccr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn scksel(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        ccr4::Scksel,
        ccr4::Scksel,
        Ccr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            ccr4::Scksel,
            ccr4::Scksel,
            Ccr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ast(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        ccr4::Ast,
        ccr4::Ast,
        Ccr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            ccr4::Ast,
            ccr4::Ast,
            Ccr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ajd(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        ccr4::Ajd,
        ccr4::Ajd,
        Ccr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            ccr4::Ajd,
            ccr4::Ajd,
            Ccr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn att(
        self,
    ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, u8, Ccr4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x7,1,0,u8,u8,Ccr4_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn aet(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        ccr4::Aet,
        ccr4::Aet,
        Ccr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            ccr4::Aet,
            ccr4::Aet,
            Ccr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ccr4 {
    #[inline(always)]
    fn default() -> Ccr4 {
        <crate::RegValueT<Ccr4_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ccr4 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Asen_SPEC;
    pub type Asen = crate::EnumBitfieldStruct<u8, Asen_SPEC>;
    impl Asen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aten_SPEC;
    pub type Aten = crate::EnumBitfieldStruct<u8, Aten_SPEC>;
    impl Aten {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Scksel_SPEC;
    pub type Scksel = crate::EnumBitfieldStruct<u8, Scksel_SPEC>;
    impl Scksel {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ast_SPEC;
    pub type Ast = crate::EnumBitfieldStruct<u8, Ast_SPEC>;
    impl Ast {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ajd_SPEC;
    pub type Ajd = crate::EnumBitfieldStruct<u8, Ajd_SPEC>;
    impl Ajd {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aet_SPEC;
    pub type Aet = crate::EnumBitfieldStruct<u8, Aet_SPEC>;
    impl Aet {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cesr_SPEC;
impl crate::sealed::RegSpec for Cesr_SPEC {
    type DataType = u8;
}

pub type Cesr = crate::RegValueT<Cesr_SPEC>;

impl Cesr {
    #[inline(always)]
    pub fn rist(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cesr::Rist,
        cesr::Rist,
        Cesr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cesr::Rist,
            cesr::Rist,
            Cesr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tist(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        cesr::Tist,
        cesr::Tist,
        Cesr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            cesr::Tist,
            cesr::Tist,
            Cesr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Cesr {
    #[inline(always)]
    fn default() -> Cesr {
        <crate::RegValueT<Cesr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cesr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rist_SPEC;
    pub type Rist = crate::EnumBitfieldStruct<u8, Rist_SPEC>;
    impl Rist {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tist_SPEC;
    pub type Tist = crate::EnumBitfieldStruct<u8, Tist_SPEC>;
    impl Tist {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr_SPEC;
impl crate::sealed::RegSpec for Icr_SPEC {
    type DataType = u32;
}

pub type Icr = crate::RegValueT<Icr_SPEC>;

impl Icr {
    #[inline(always)]
    pub fn iicdl(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Icr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Icr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn iicintm(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        icr::Iicintm,
        icr::Iicintm,
        Icr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            icr::Iicintm,
            icr::Iicintm,
            Icr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn iiccsc(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        icr::Iiccsc,
        icr::Iiccsc,
        Icr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            icr::Iiccsc,
            icr::Iiccsc,
            Icr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn iicackt(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        icr::Iicackt,
        icr::Iicackt,
        Icr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            icr::Iicackt,
            icr::Iicackt,
            Icr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn iicstareq(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        icr::Iicstareq,
        icr::Iicstareq,
        Icr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            icr::Iicstareq,
            icr::Iicstareq,
            Icr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn iicrstareq(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        icr::Iicrstareq,
        icr::Iicrstareq,
        Icr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            icr::Iicrstareq,
            icr::Iicrstareq,
            Icr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn iicstpreq(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        icr::Iicstpreq,
        icr::Iicstpreq,
        Icr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            icr::Iicstpreq,
            icr::Iicstpreq,
            Icr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn iicsdas(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x3,
        1,
        0,
        icr::Iicsdas,
        icr::Iicsdas,
        Icr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x3,
            1,
            0,
            icr::Iicsdas,
            icr::Iicsdas,
            Icr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn iicscls(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x3,
        1,
        0,
        icr::Iicscls,
        icr::Iicscls,
        Icr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x3,
            1,
            0,
            icr::Iicscls,
            icr::Iicscls,
            Icr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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
    pub struct Iicintm_SPEC;
    pub type Iicintm = crate::EnumBitfieldStruct<u8, Iicintm_SPEC>;
    impl Iicintm {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iiccsc_SPEC;
    pub type Iiccsc = crate::EnumBitfieldStruct<u8, Iiccsc_SPEC>;
    impl Iiccsc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicackt_SPEC;
    pub type Iicackt = crate::EnumBitfieldStruct<u8, Iicackt_SPEC>;
    impl Iicackt {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicstareq_SPEC;
    pub type Iicstareq = crate::EnumBitfieldStruct<u8, Iicstareq_SPEC>;
    impl Iicstareq {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicrstareq_SPEC;
    pub type Iicrstareq = crate::EnumBitfieldStruct<u8, Iicrstareq_SPEC>;
    impl Iicrstareq {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicstpreq_SPEC;
    pub type Iicstpreq = crate::EnumBitfieldStruct<u8, Iicstpreq_SPEC>;
    impl Iicstpreq {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicsdas_SPEC;
    pub type Iicsdas = crate::EnumBitfieldStruct<u8, Iicsdas_SPEC>;
    impl Iicsdas {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicscls_SPEC;
    pub type Iicscls = crate::EnumBitfieldStruct<u8, Iicscls_SPEC>;
    impl Iicscls {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcr_SPEC;
impl crate::sealed::RegSpec for Fcr_SPEC {
    type DataType = u32;
}

pub type Fcr = crate::RegValueT<Fcr_SPEC>;

impl Fcr {
    #[inline(always)]
    pub fn dres(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, fcr::Dres, fcr::Dres, Fcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            fcr::Dres,
            fcr::Dres,
            Fcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ttrg(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, Fcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,Fcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tfrst(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        fcr::Tfrst,
        fcr::Tfrst,
        Fcr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            fcr::Tfrst,
            fcr::Tfrst,
            Fcr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rtrg(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Fcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,Fcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfrst(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        fcr::Rfrst,
        fcr::Rfrst,
        Fcr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            fcr::Rfrst,
            fcr::Rfrst,
            Fcr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rstrg(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, u8, Fcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8,u8,Fcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fcr {
    #[inline(always)]
    fn default() -> Fcr {
        <crate::RegValueT<Fcr_SPEC> as RegisterValue<_>>::new(522125312)
    }
}
pub mod fcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dres_SPEC;
    pub type Dres = crate::EnumBitfieldStruct<u8, Dres_SPEC>;
    impl Dres {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tfrst_SPEC;
    pub type Tfrst = crate::EnumBitfieldStruct<u8, Tfrst_SPEC>;
    impl Tfrst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfrst_SPEC;
    pub type Rfrst = crate::EnumBitfieldStruct<u8, Rfrst_SPEC>;
    impl Rfrst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcr_SPEC;
impl crate::sealed::RegSpec for Mcr_SPEC {
    type DataType = u32;
}

pub type Mcr = crate::RegValueT<Mcr_SPEC>;

impl Mcr {
    #[inline(always)]
    pub fn rmpol(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        mcr::Rmpol,
        mcr::Rmpol,
        Mcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            mcr::Rmpol,
            mcr::Rmpol,
            Mcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tmpol(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        mcr::Tmpol,
        mcr::Tmpol,
        Mcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            mcr::Tmpol,
            mcr::Tmpol,
            Mcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn erten(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        mcr::Erten,
        mcr::Erten,
        Mcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            mcr::Erten,
            mcr::Erten,
            Mcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn synval(self) -> crate::common::RegisterFieldBool<4, 1, 0, Mcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Mcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn synsel(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        mcr::Synsel,
        mcr::Synsel,
        Mcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            mcr::Synsel,
            mcr::Synsel,
            Mcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sbsel(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        mcr::Sbsel,
        mcr::Sbsel,
        Mcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            mcr::Sbsel,
            mcr::Sbsel,
            Mcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tplen(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xf,
        1,
        0,
        mcr::Tplen,
        mcr::Tplen,
        Mcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            mcr::Tplen,
            mcr::Tplen,
            Mcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tppat(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        mcr::Tppat,
        mcr::Tppat,
        Mcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            mcr::Tppat,
            mcr::Tppat,
            Mcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rplen(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xf,
        1,
        0,
        mcr::Rplen,
        mcr::Rplen,
        Mcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xf,
            1,
            0,
            mcr::Rplen,
            mcr::Rplen,
            Mcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rppat(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x3,
        1,
        0,
        mcr::Rppat,
        mcr::Rppat,
        Mcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x3,
            1,
            0,
            mcr::Rppat,
            mcr::Rppat,
            Mcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pferen(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        mcr::Pferen,
        mcr::Pferen,
        Mcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            mcr::Pferen,
            mcr::Pferen,
            Mcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn syeren(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        mcr::Syeren,
        mcr::Syeren,
        Mcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            mcr::Syeren,
            mcr::Syeren,
            Mcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sberen(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        mcr::Sberen,
        mcr::Sberen,
        Mcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            mcr::Sberen,
            mcr::Sberen,
            Mcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Mcr {
    #[inline(always)]
    fn default() -> Mcr {
        <crate::RegValueT<Mcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmpol_SPEC;
    pub type Rmpol = crate::EnumBitfieldStruct<u8, Rmpol_SPEC>;
    impl Rmpol {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmpol_SPEC;
    pub type Tmpol = crate::EnumBitfieldStruct<u8, Tmpol_SPEC>;
    impl Tmpol {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Erten_SPEC;
    pub type Erten = crate::EnumBitfieldStruct<u8, Erten_SPEC>;
    impl Erten {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Synsel_SPEC;
    pub type Synsel = crate::EnumBitfieldStruct<u8, Synsel_SPEC>;
    impl Synsel {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sbsel_SPEC;
    pub type Sbsel = crate::EnumBitfieldStruct<u8, Sbsel_SPEC>;
    impl Sbsel {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tplen_SPEC;
    pub type Tplen = crate::EnumBitfieldStruct<u8, Tplen_SPEC>;
    impl Tplen {
        pub const _0_X_0: Self = Self::new(0);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tppat_SPEC;
    pub type Tppat = crate::EnumBitfieldStruct<u8, Tppat_SPEC>;
    impl Tppat {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rplen_SPEC;
    pub type Rplen = crate::EnumBitfieldStruct<u8, Rplen_SPEC>;
    impl Rplen {
        pub const _0_X_0: Self = Self::new(0);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rppat_SPEC;
    pub type Rppat = crate::EnumBitfieldStruct<u8, Rppat_SPEC>;
    impl Rppat {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pferen_SPEC;
    pub type Pferen = crate::EnumBitfieldStruct<u8, Pferen_SPEC>;
    impl Pferen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Syeren_SPEC;
    pub type Syeren = crate::EnumBitfieldStruct<u8, Syeren_SPEC>;
    impl Syeren {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sberen_SPEC;
    pub type Sberen = crate::EnumBitfieldStruct<u8, Sberen_SPEC>;
    impl Sberen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcr_SPEC;
impl crate::sealed::RegSpec for Dcr_SPEC {
    type DataType = u32;
}

pub type Dcr = crate::RegValueT<Dcr_SPEC>;

impl Dcr {
    #[inline(always)]
    pub fn depol(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        dcr::Depol,
        dcr::Depol,
        Dcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            dcr::Depol,
            dcr::Depol,
            Dcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn deast(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, Dcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,Dcr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dengt(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, Dcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,Dcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dcr {
    #[inline(always)]
    fn default() -> Dcr {
        <crate::RegValueT<Dcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Depol_SPEC;
    pub type Depol = crate::EnumBitfieldStruct<u8, Depol_SPEC>;
    impl Depol {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xcr0_SPEC;
impl crate::sealed::RegSpec for Xcr0_SPEC {
    type DataType = u32;
}

pub type Xcr0 = crate::RegValueT<Xcr0_SPEC>;

impl Xcr0 {
    #[inline(always)]
    pub fn tcss(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        xcr0::Tcss,
        xcr0::Tcss,
        Xcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            xcr0::Tcss,
            xcr0::Tcss,
            Xcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bfe(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        xcr0::Bfe,
        xcr0::Bfe,
        Xcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            xcr0::Bfe,
            xcr0::Bfe,
            Xcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cf0re(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        xcr0::Cf0Re,
        xcr0::Cf0Re,
        Xcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            xcr0::Cf0Re,
            xcr0::Cf0Re,
            Xcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cf1ds(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        xcr0::Cf1Ds,
        xcr0::Cf1Ds,
        Xcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            xcr0::Cf1Ds,
            xcr0::Cf1Ds,
            Xcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pibe(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        xcr0::Pibe,
        xcr0::Pibe,
        Xcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            xcr0::Pibe,
            xcr0::Pibe,
            Xcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pibs(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x7,
        1,
        0,
        xcr0::Pibs,
        xcr0::Pibs,
        Xcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x7,
            1,
            0,
            xcr0::Pibs,
            xcr0::Pibs,
            Xcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bfoie(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        xcr0::Bfoie,
        xcr0::Bfoie,
        Xcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            xcr0::Bfoie,
            xcr0::Bfoie,
            Xcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bcdie(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        xcr0::Bcdie,
        xcr0::Bcdie,
        Xcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            xcr0::Bcdie,
            xcr0::Bcdie,
            Xcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bfdie(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        xcr0::Bfdie,
        xcr0::Bfdie,
        Xcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            xcr0::Bfdie,
            xcr0::Bfdie,
            Xcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cofie(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        xcr0::Cofie,
        xcr0::Cofie,
        Xcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            xcr0::Cofie,
            xcr0::Cofie,
            Xcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn aedie(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        xcr0::Aedie,
        xcr0::Aedie,
        Xcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            xcr0::Aedie,
            xcr0::Aedie,
            Xcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bccs(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x3,
        1,
        0,
        xcr0::Bccs,
        xcr0::Bccs,
        Xcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x3,
            1,
            0,
            xcr0::Bccs,
            xcr0::Bccs,
            Xcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Xcr0 {
    #[inline(always)]
    fn default() -> Xcr0 {
        <crate::RegValueT<Xcr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod xcr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcss_SPEC;
    pub type Tcss = crate::EnumBitfieldStruct<u8, Tcss_SPEC>;
    impl Tcss {
        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bfe_SPEC;
    pub type Bfe = crate::EnumBitfieldStruct<u8, Bfe_SPEC>;
    impl Bfe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf0Re_SPEC;
    pub type Cf0Re = crate::EnumBitfieldStruct<u8, Cf0Re_SPEC>;
    impl Cf0Re {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf1Ds_SPEC;
    pub type Cf1Ds = crate::EnumBitfieldStruct<u8, Cf1Ds_SPEC>;
    impl Cf1Ds {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pibe_SPEC;
    pub type Pibe = crate::EnumBitfieldStruct<u8, Pibe_SPEC>;
    impl Pibe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pibs_SPEC;
    pub type Pibs = crate::EnumBitfieldStruct<u8, Pibs_SPEC>;
    impl Pibs {
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
    pub struct Bfoie_SPEC;
    pub type Bfoie = crate::EnumBitfieldStruct<u8, Bfoie_SPEC>;
    impl Bfoie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bcdie_SPEC;
    pub type Bcdie = crate::EnumBitfieldStruct<u8, Bcdie_SPEC>;
    impl Bcdie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bfdie_SPEC;
    pub type Bfdie = crate::EnumBitfieldStruct<u8, Bfdie_SPEC>;
    impl Bfdie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cofie_SPEC;
    pub type Cofie = crate::EnumBitfieldStruct<u8, Cofie_SPEC>;
    impl Cofie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aedie_SPEC;
    pub type Aedie = crate::EnumBitfieldStruct<u8, Aedie_SPEC>;
    impl Aedie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bccs_SPEC;
    pub type Bccs = crate::EnumBitfieldStruct<u8, Bccs_SPEC>;
    impl Bccs {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xcr1_SPEC;
impl crate::sealed::RegSpec for Xcr1_SPEC {
    type DataType = u32;
}

pub type Xcr1 = crate::RegValueT<Xcr1_SPEC>;

impl Xcr1 {
    #[inline(always)]
    pub fn tcst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        xcr1::Tcst,
        xcr1::Tcst,
        Xcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            xcr1::Tcst,
            xcr1::Tcst,
            Xcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sdst(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        xcr1::Sdst,
        xcr1::Sdst,
        Xcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            xcr1::Sdst,
            xcr1::Sdst,
            Xcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bmen(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        xcr1::Bmen,
        xcr1::Bmen,
        Xcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            xcr1::Bmen,
            xcr1::Bmen,
            Xcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pcf1d(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, Xcr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,Xcr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn scf1d(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Xcr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Xcr1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cf1ce(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        xcr1::Cf1Ce,
        xcr1::Cf1Ce,
        Xcr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            xcr1::Cf1Ce,
            xcr1::Cf1Ce,
            Xcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Xcr1 {
    #[inline(always)]
    fn default() -> Xcr1 {
        <crate::RegValueT<Xcr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod xcr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tcst_SPEC;
    pub type Tcst = crate::EnumBitfieldStruct<u8, Tcst_SPEC>;
    impl Tcst {
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
    pub struct Bmen_SPEC;
    pub type Bmen = crate::EnumBitfieldStruct<u8, Bmen_SPEC>;
    impl Bmen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf1Ce_SPEC;
    pub type Cf1Ce = crate::EnumBitfieldStruct<u8, Cf1Ce_SPEC>;
    impl Cf1Ce {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xcr2_SPEC;
impl crate::sealed::RegSpec for Xcr2_SPEC {
    type DataType = u32;
}

pub type Xcr2 = crate::RegValueT<Xcr2_SPEC>;

impl Xcr2 {
    #[inline(always)]
    pub fn cf0d(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Xcr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Xcr2_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cf0ce(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        xcr2::Cf0Ce,
        xcr2::Cf0Ce,
        Xcr2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            xcr2::Cf0Ce,
            xcr2::Cf0Ce,
            Xcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bflw(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, u16, Xcr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16,u16,Xcr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Xcr2 {
    #[inline(always)]
    fn default() -> Xcr2 {
        <crate::RegValueT<Xcr2_SPEC> as RegisterValue<_>>::new(4294836224)
    }
}
pub mod xcr2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf0Ce_SPEC;
    pub type Cf0Ce = crate::EnumBitfieldStruct<u8, Cf0Ce_SPEC>;
    impl Cf0Ce {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csr_SPEC;
impl crate::sealed::RegSpec for Csr_SPEC {
    type DataType = u32;
}

pub type Csr = crate::RegValueT<Csr_SPEC>;

impl Csr {
    #[inline(always)]
    pub fn ers(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, csr::Ers, csr::Ers, Csr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0x1,1,0,csr::Ers,csr::Ers,Csr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rxdmon(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        csr::Rxdmon,
        csr::Rxdmon,
        Csr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            csr::Rxdmon,
            csr::Rxdmon,
            Csr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dcmf(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, csr::Dcmf, csr::Dcmf, Csr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            csr::Dcmf,
            csr::Dcmf,
            Csr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dper(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, csr::Dper, csr::Dper, Csr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            csr::Dper,
            csr::Dper,
            Csr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dfer(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, csr::Dfer, csr::Dfer, Csr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            csr::Dfer,
            csr::Dfer,
            Csr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn orer(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, csr::Orer, csr::Orer, Csr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            csr::Orer,
            csr::Orer,
            Csr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mff(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, csr::Mff, csr::Mff, Csr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<26,0x1,1,0,csr::Mff,csr::Mff,Csr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn per(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, csr::Per, csr::Per, Csr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<27,0x1,1,0,csr::Per,csr::Per,Csr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn fer(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, csr::Fer, csr::Fer, Csr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<28,0x1,1,0,csr::Fer,csr::Fer,Csr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tdre(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, csr::Tdre, csr::Tdre, Csr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            csr::Tdre,
            csr::Tdre,
            Csr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tend(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, csr::Tend, csr::Tend, Csr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            csr::Tend,
            csr::Tend,
            Csr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rdrf(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, csr::Rdrf, csr::Rdrf, Csr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            csr::Rdrf,
            csr::Rdrf,
            Csr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Csr {
    #[inline(always)]
    fn default() -> Csr {
        <crate::RegValueT<Csr_SPEC> as RegisterValue<_>>::new(1610645504)
    }
}
pub mod csr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ers_SPEC;
    pub type Ers = crate::EnumBitfieldStruct<u8, Ers_SPEC>;
    impl Ers {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxdmon_SPEC;
    pub type Rxdmon = crate::EnumBitfieldStruct<u8, Rxdmon_SPEC>;
    impl Rxdmon {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dcmf_SPEC;
    pub type Dcmf = crate::EnumBitfieldStruct<u8, Dcmf_SPEC>;
    impl Dcmf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dper_SPEC;
    pub type Dper = crate::EnumBitfieldStruct<u8, Dper_SPEC>;
    impl Dper {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dfer_SPEC;
    pub type Dfer = crate::EnumBitfieldStruct<u8, Dfer_SPEC>;
    impl Dfer {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Orer_SPEC;
    pub type Orer = crate::EnumBitfieldStruct<u8, Orer_SPEC>;
    impl Orer {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mff_SPEC;
    pub type Mff = crate::EnumBitfieldStruct<u8, Mff_SPEC>;
    impl Mff {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Per_SPEC;
    pub type Per = crate::EnumBitfieldStruct<u8, Per_SPEC>;
    impl Per {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fer_SPEC;
    pub type Fer = crate::EnumBitfieldStruct<u8, Fer_SPEC>;
    impl Fer {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tdre_SPEC;
    pub type Tdre = crate::EnumBitfieldStruct<u8, Tdre_SPEC>;
    impl Tdre {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tend_SPEC;
    pub type Tend = crate::EnumBitfieldStruct<u8, Tend_SPEC>;
    impl Tend {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdrf_SPEC;
    pub type Rdrf = crate::EnumBitfieldStruct<u8, Rdrf_SPEC>;
    impl Rdrf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr_SPEC;
impl crate::sealed::RegSpec for Isr_SPEC {
    type DataType = u32;
}

pub type Isr = crate::RegValueT<Isr_SPEC>;

impl Isr {
    #[inline(always)]
    pub fn iicackr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        isr::Iicackr,
        isr::Iicackr,
        Isr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            isr::Iicackr,
            isr::Iicackr,
            Isr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn iicstif(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        isr::Iicstif,
        isr::Iicstif,
        Isr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            isr::Iicstif,
            isr::Iicstif,
            Isr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Isr {
    #[inline(always)]
    fn default() -> Isr {
        <crate::RegValueT<Isr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod isr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicackr_SPEC;
    pub type Iicackr = crate::EnumBitfieldStruct<u8, Iicackr_SPEC>;
    impl Iicackr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iicstif_SPEC;
    pub type Iicstif = crate::EnumBitfieldStruct<u8, Iicstif_SPEC>;
    impl Iicstif {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frsr_SPEC;
impl crate::sealed::RegSpec for Frsr_SPEC {
    type DataType = u32;
}

pub type Frsr = crate::RegValueT<Frsr_SPEC>;

impl Frsr {
    #[inline(always)]
    pub fn dr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, frsr::Dr, frsr::Dr, Frsr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,frsr::Dr,frsr::Dr,Frsr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn r(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, u8, Frsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x3f,1,0,u8,u8,Frsr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pnum(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, u8, Frsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3f,1,0,u8,u8,Frsr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn fnum(
        self,
    ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, u8, Frsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x3f,1,0,u8,u8,Frsr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Frsr {
    #[inline(always)]
    fn default() -> Frsr {
        <crate::RegValueT<Frsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod frsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dr_SPEC;
    pub type Dr = crate::EnumBitfieldStruct<u8, Dr_SPEC>;
    impl Dr {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ftsr_SPEC;
impl crate::sealed::RegSpec for Ftsr_SPEC {
    type DataType = u32;
}

pub type Ftsr = crate::RegValueT<Ftsr_SPEC>;

impl Ftsr {
    #[inline(always)]
    pub fn t(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Ftsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Ftsr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Ftsr {
    #[inline(always)]
    fn default() -> Ftsr {
        <crate::RegValueT<Ftsr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msr_SPEC;
impl crate::sealed::RegSpec for Msr_SPEC {
    type DataType = u32;
}

pub type Msr = crate::RegValueT<Msr_SPEC>;

impl Msr {
    #[inline(always)]
    pub fn pfer(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, msr::Pfer, msr::Pfer, Msr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,msr::Pfer,msr::Pfer,Msr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn syer(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, msr::Syer, msr::Syer, Msr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,msr::Syer,msr::Syer,Msr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sber(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, msr::Sber, msr::Sber, Msr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x1,1,0,msr::Sber,msr::Sber,Msr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mer(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, msr::Mer, msr::Mer, Msr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0x1,1,0,msr::Mer,msr::Mer,Msr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rsync(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        msr::Rsync,
        msr::Rsync,
        Msr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            msr::Rsync,
            msr::Rsync,
            Msr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Msr {
    #[inline(always)]
    fn default() -> Msr {
        <crate::RegValueT<Msr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod msr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pfer_SPEC;
    pub type Pfer = crate::EnumBitfieldStruct<u8, Pfer_SPEC>;
    impl Pfer {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Syer_SPEC;
    pub type Syer = crate::EnumBitfieldStruct<u8, Syer_SPEC>;
    impl Syer {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sber_SPEC;
    pub type Sber = crate::EnumBitfieldStruct<u8, Sber_SPEC>;
    impl Sber {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mer_SPEC;
    pub type Mer = crate::EnumBitfieldStruct<u8, Mer_SPEC>;
    impl Mer {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rsync_SPEC;
    pub type Rsync = crate::EnumBitfieldStruct<u8, Rsync_SPEC>;
    impl Rsync {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xsr0_SPEC;
impl crate::sealed::RegSpec for Xsr0_SPEC {
    type DataType = u32;
}

pub type Xsr0 = crate::RegValueT<Xsr0_SPEC>;

impl Xsr0 {
    #[inline(always)]
    pub fn sfsf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        xsr0::Sfsf,
        xsr0::Sfsf,
        Xsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            xsr0::Sfsf,
            xsr0::Sfsf,
            Xsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rxdsf(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        xsr0::Rxdsf,
        xsr0::Rxdsf,
        Xsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            xsr0::Rxdsf,
            xsr0::Rxdsf,
            Xsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bfof(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        xsr0::Bfof,
        xsr0::Bfof,
        Xsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            xsr0::Bfof,
            xsr0::Bfof,
            Xsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bcdf(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        xsr0::Bcdf,
        xsr0::Bcdf,
        Xsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            xsr0::Bcdf,
            xsr0::Bcdf,
            Xsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bfdf(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        xsr0::Bfdf,
        xsr0::Bfdf,
        Xsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            xsr0::Bfdf,
            xsr0::Bfdf,
            Xsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cf0mf(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        xsr0::Cf0Mf,
        xsr0::Cf0Mf,
        Xsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            xsr0::Cf0Mf,
            xsr0::Cf0Mf,
            Xsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cf1mf(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        xsr0::Cf1Mf,
        xsr0::Cf1Mf,
        Xsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            xsr0::Cf1Mf,
            xsr0::Cf1Mf,
            Xsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pibdf(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        xsr0::Pibdf,
        xsr0::Pibdf,
        Xsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            xsr0::Pibdf,
            xsr0::Pibdf,
            Xsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cof(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        xsr0::Cof,
        xsr0::Cof,
        Xsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            xsr0::Cof,
            xsr0::Cof,
            Xsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn aedf(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        xsr0::Aedf,
        xsr0::Aedf,
        Xsr0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            xsr0::Aedf,
            xsr0::Aedf,
            Xsr0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cf0rd(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, Xsr0_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,Xsr0_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cf1rd(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, Xsr0_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,Xsr0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Xsr0 {
    #[inline(always)]
    fn default() -> Xsr0 {
        <crate::RegValueT<Xsr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod xsr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sfsf_SPEC;
    pub type Sfsf = crate::EnumBitfieldStruct<u8, Sfsf_SPEC>;
    impl Sfsf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rxdsf_SPEC;
    pub type Rxdsf = crate::EnumBitfieldStruct<u8, Rxdsf_SPEC>;
    impl Rxdsf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bfof_SPEC;
    pub type Bfof = crate::EnumBitfieldStruct<u8, Bfof_SPEC>;
    impl Bfof {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bcdf_SPEC;
    pub type Bcdf = crate::EnumBitfieldStruct<u8, Bcdf_SPEC>;
    impl Bcdf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bfdf_SPEC;
    pub type Bfdf = crate::EnumBitfieldStruct<u8, Bfdf_SPEC>;
    impl Bfdf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf0Mf_SPEC;
    pub type Cf0Mf = crate::EnumBitfieldStruct<u8, Cf0Mf_SPEC>;
    impl Cf0Mf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cf1Mf_SPEC;
    pub type Cf1Mf = crate::EnumBitfieldStruct<u8, Cf1Mf_SPEC>;
    impl Cf1Mf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pibdf_SPEC;
    pub type Pibdf = crate::EnumBitfieldStruct<u8, Pibdf_SPEC>;
    impl Pibdf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cof_SPEC;
    pub type Cof = crate::EnumBitfieldStruct<u8, Cof_SPEC>;
    impl Cof {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aedf_SPEC;
    pub type Aedf = crate::EnumBitfieldStruct<u8, Aedf_SPEC>;
    impl Aedf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xsr1_SPEC;
impl crate::sealed::RegSpec for Xsr1_SPEC {
    type DataType = u32;
}

pub type Xsr1 = crate::RegValueT<Xsr1_SPEC>;

impl Xsr1 {
    #[inline(always)]
    pub fn tcnt(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Xsr1_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Xsr1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Xsr1 {
    #[inline(always)]
    fn default() -> Xsr1 {
        <crate::RegValueT<Xsr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfclr_SPEC;
impl crate::sealed::RegSpec for Cfclr_SPEC {
    type DataType = u32;
}

pub type Cfclr = crate::RegValueT<Cfclr_SPEC>;

impl Cfclr {
    #[inline(always)]
    pub fn ersc(self) -> crate::common::RegisterFieldBool<4, 1, 0, Cfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn dcmfc(self) -> crate::common::RegisterFieldBool<16, 1, 0, Cfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, Cfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn dperc(self) -> crate::common::RegisterFieldBool<17, 1, 0, Cfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, Cfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn dferc(self) -> crate::common::RegisterFieldBool<18, 1, 0, Cfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, Cfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn orerc(self) -> crate::common::RegisterFieldBool<24, 1, 0, Cfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, Cfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn mffc(self) -> crate::common::RegisterFieldBool<26, 1, 0, Cfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, Cfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn perc(self) -> crate::common::RegisterFieldBool<27, 1, 0, Cfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<27, 1, 0, Cfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn ferc(self) -> crate::common::RegisterFieldBool<28, 1, 0, Cfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, Cfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn tdrec(self) -> crate::common::RegisterFieldBool<29, 1, 0, Cfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<29, 1, 0, Cfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn rdrfc(self) -> crate::common::RegisterFieldBool<31, 1, 0, Cfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Cfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Cfclr {
    #[inline(always)]
    fn default() -> Cfclr {
        <crate::RegValueT<Cfclr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icfclr_SPEC;
impl crate::sealed::RegSpec for Icfclr_SPEC {
    type DataType = u32;
}

pub type Icfclr = crate::RegValueT<Icfclr_SPEC>;

impl Icfclr {
    #[inline(always)]
    pub fn iicstifc(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Icfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Icfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Icfclr {
    #[inline(always)]
    fn default() -> Icfclr {
        <crate::RegValueT<Icfclr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ffclr_SPEC;
impl crate::sealed::RegSpec for Ffclr_SPEC {
    type DataType = u32;
}

pub type Ffclr = crate::RegValueT<Ffclr_SPEC>;

impl Ffclr {
    #[inline(always)]
    pub fn drc(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ffclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ffclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Ffclr {
    #[inline(always)]
    fn default() -> Ffclr {
        <crate::RegValueT<Ffclr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mfclr_SPEC;
impl crate::sealed::RegSpec for Mfclr_SPEC {
    type DataType = u32;
}

pub type Mfclr = crate::RegValueT<Mfclr_SPEC>;

impl Mfclr {
    #[inline(always)]
    pub fn pferc(self) -> crate::common::RegisterFieldBool<0, 1, 0, Mfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Mfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn syerc(self) -> crate::common::RegisterFieldBool<1, 1, 0, Mfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Mfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn sberc(self) -> crate::common::RegisterFieldBool<2, 1, 0, Mfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Mfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn merc(self) -> crate::common::RegisterFieldBool<4, 1, 0, Mfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Mfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Mfclr {
    #[inline(always)]
    fn default() -> Mfclr {
        <crate::RegValueT<Mfclr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xfclr_SPEC;
impl crate::sealed::RegSpec for Xfclr_SPEC {
    type DataType = u32;
}

pub type Xfclr = crate::RegValueT<Xfclr_SPEC>;

impl Xfclr {
    #[inline(always)]
    pub fn bfoc(self) -> crate::common::RegisterFieldBool<8, 1, 0, Xfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, Xfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn bcdc(self) -> crate::common::RegisterFieldBool<9, 1, 0, Xfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, Xfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn bfdc(self) -> crate::common::RegisterFieldBool<10, 1, 0, Xfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, Xfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn cf0mc(self) -> crate::common::RegisterFieldBool<11, 1, 0, Xfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, Xfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn cf1mc(self) -> crate::common::RegisterFieldBool<12, 1, 0, Xfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, Xfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn pibdc(self) -> crate::common::RegisterFieldBool<13, 1, 0, Xfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, Xfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn cofc(self) -> crate::common::RegisterFieldBool<14, 1, 0, Xfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, Xfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }

    #[inline(always)]
    pub fn aedc(self) -> crate::common::RegisterFieldBool<15, 1, 0, Xfclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, Xfclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Xfclr {
    #[inline(always)]
    fn default() -> Xfclr {
        <crate::RegValueT<Xfclr_SPEC> as RegisterValue<_>>::new(0)
    }
}
