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
// Generated from SVD 1.51.00, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:17:11 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"12-bit A/D Converter"]
unsafe impl ::core::marker::Send for super::Adc120 {}
unsafe impl ::core::marker::Sync for super::Adc120 {}
impl super::Adc120 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn adcsr(&self) -> &'static crate::common::Reg<self::Adcsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adansa0(
        &self,
    ) -> &'static crate::common::Reg<self::Adansa0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adansa0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adansa1(
        &self,
    ) -> &'static crate::common::Reg<self::Adansa1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adansa1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adads0(
        &self,
    ) -> &'static crate::common::Reg<self::Adads0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adads0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adads1(
        &self,
    ) -> &'static crate::common::Reg<self::Adads1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adads1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adadc(&self) -> &'static crate::common::Reg<self::Adadc_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adadc_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adcer(&self) -> &'static crate::common::Reg<self::Adcer_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcer_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adstrgr(
        &self,
    ) -> &'static crate::common::Reg<self::Adstrgr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adstrgr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adexicr(
        &self,
    ) -> &'static crate::common::Reg<self::Adexicr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adexicr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adansb0(
        &self,
    ) -> &'static crate::common::Reg<self::Adansb0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adansb0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adansb1(
        &self,
    ) -> &'static crate::common::Reg<self::Adansb1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adansb1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(22usize),
            )
        }
    }

    #[inline(always)]
    pub const fn addbldr(
        &self,
    ) -> &'static crate::common::Reg<self::Addbldr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Addbldr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adtsdr(&self) -> &'static crate::common::Reg<self::Adtsdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adtsdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(26usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adocdr(&self) -> &'static crate::common::Reg<self::Adocdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adocdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adrd(&self) -> &'static crate::common::Reg<self::Adrd_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adrd_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(30usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adctdr(&self) -> &'static crate::common::Reg<self::Adctdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adctdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn addr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Addr_SPEC, crate::common::R>,
        6,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x42usize))
        }
    }

    #[inline(always)]
    pub const fn addiscr(
        &self,
    ) -> &'static crate::common::Reg<self::Addiscr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Addiscr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(122usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adacsr(
        &self,
    ) -> &'static crate::common::Reg<self::Adacsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adacsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(126usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adgspcr(
        &self,
    ) -> &'static crate::common::Reg<self::Adgspcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adgspcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[inline(always)]
    pub const fn addbldra(
        &self,
    ) -> &'static crate::common::Reg<self::Addbldra_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Addbldra_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[inline(always)]
    pub const fn addbldrb(
        &self,
    ) -> &'static crate::common::Reg<self::Addbldrb_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Addbldrb_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(134usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adhvrefcnt(
        &self,
    ) -> &'static crate::common::Reg<self::Adhvrefcnt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adhvrefcnt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(138usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adwinmon(
        &self,
    ) -> &'static crate::common::Reg<self::Adwinmon_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Adwinmon_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(140usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adcmpcr(
        &self,
    ) -> &'static crate::common::Reg<self::Adcmpcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcmpcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adcmpanser(
        &self,
    ) -> &'static crate::common::Reg<self::Adcmpanser_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcmpanser_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(146usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adcmpler(
        &self,
    ) -> &'static crate::common::Reg<self::Adcmpler_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcmpler_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(147usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adcmpansr0(
        &self,
    ) -> &'static crate::common::Reg<self::Adcmpansr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcmpansr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adcmpansr1(
        &self,
    ) -> &'static crate::common::Reg<self::Adcmpansr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcmpansr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(150usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adcmplr0(
        &self,
    ) -> &'static crate::common::Reg<self::Adcmplr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcmplr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(152usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adcmplr1(
        &self,
    ) -> &'static crate::common::Reg<self::Adcmplr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcmplr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(154usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adcmpdr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Adcmpdr_SPEC, crate::common::RW>,
        2,
        0x2,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0x9cusize))
        }
    }

    #[inline(always)]
    pub const fn adcmpsr0(
        &self,
    ) -> &'static crate::common::Reg<self::Adcmpsr0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcmpsr0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(160usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adcmpsr1(
        &self,
    ) -> &'static crate::common::Reg<self::Adcmpsr1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcmpsr1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(162usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adcmpser(
        &self,
    ) -> &'static crate::common::Reg<self::Adcmpser_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcmpser_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(164usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adcmpbnsr(
        &self,
    ) -> &'static crate::common::Reg<self::Adcmpbnsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcmpbnsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(166usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adwinllb(
        &self,
    ) -> &'static crate::common::Reg<self::Adwinllb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adwinllb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(168usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adwinulb(
        &self,
    ) -> &'static crate::common::Reg<self::Adwinulb_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adwinulb_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(170usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adcmpbsr(
        &self,
    ) -> &'static crate::common::Reg<self::Adcmpbsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adcmpbsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(172usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adsstrl(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstrl_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstrl_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(221usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adsstrt(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstrt_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstrt_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(222usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adsstro(
        &self,
    ) -> &'static crate::common::Reg<self::Adsstro_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Adsstro_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(223usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adsstr(
        &self,
    ) -> &'static crate::common::ClusterRegisterArray<
        crate::common::Reg<self::Adsstr_SPEC, crate::common::RW>,
        11,
        0x1,
    > {
        unsafe {
            crate::common::ClusterRegisterArray::from_ptr(self._svd2pac_as_ptr().add(0xe0usize))
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcsr_SPEC;
impl crate::sealed::RegSpec for Adcsr_SPEC {
    type DataType = u16;
}

pub type Adcsr = crate::RegValueT<Adcsr_SPEC>;

impl Adcsr {
    #[inline(always)]
    pub fn dblans(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Adcsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Adcsr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gbadie(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adcsr::Gbadie,
        adcsr::Gbadie,
        Adcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adcsr::Gbadie,
            adcsr::Gbadie,
            Adcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dble(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adcsr::Dble,
        adcsr::Dble,
        Adcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adcsr::Dble,
            adcsr::Dble,
            Adcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn extrg(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adcsr::Extrg,
        adcsr::Extrg,
        Adcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adcsr::Extrg,
            adcsr::Extrg,
            Adcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn trge(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        adcsr::Trge,
        adcsr::Trge,
        Adcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            adcsr::Trge,
            adcsr::Trge,
            Adcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adhsc(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        adcsr::Adhsc,
        adcsr::Adhsc,
        Adcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            adcsr::Adhsc,
            adcsr::Adhsc,
            Adcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adcs(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x3,
        1,
        0,
        adcsr::Adcs,
        adcsr::Adcs,
        Adcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x3,
            1,
            0,
            adcsr::Adcs,
            adcsr::Adcs,
            Adcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adst(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        adcsr::Adst,
        adcsr::Adst,
        Adcsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            adcsr::Adst,
            adcsr::Adst,
            Adcsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcsr {
    #[inline(always)]
    fn default() -> Adcsr {
        <crate::RegValueT<Adcsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gbadie_SPEC;
    pub type Gbadie = crate::EnumBitfieldStruct<u8, Gbadie_SPEC>;
    impl Gbadie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Dble_SPEC;
    pub type Dble = crate::EnumBitfieldStruct<u8, Dble_SPEC>;
    impl Dble {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Extrg_SPEC;
    pub type Extrg = crate::EnumBitfieldStruct<u8, Extrg_SPEC>;
    impl Extrg {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trge_SPEC;
    pub type Trge = crate::EnumBitfieldStruct<u8, Trge_SPEC>;
    impl Trge {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adhsc_SPEC;
    pub type Adhsc = crate::EnumBitfieldStruct<u8, Adhsc_SPEC>;
    impl Adhsc {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adcs_SPEC;
    pub type Adcs = crate::EnumBitfieldStruct<u8, Adcs_SPEC>;
    impl Adcs {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adst_SPEC;
    pub type Adst = crate::EnumBitfieldStruct<u8, Adst_SPEC>;
    impl Adst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adansa0_SPEC;
impl crate::sealed::RegSpec for Adansa0_SPEC {
    type DataType = u16;
}

pub type Adansa0 = crate::RegValueT<Adansa0_SPEC>;

impl Adansa0 {
    #[inline(always)]
    pub fn ansa00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adansa0::Ansa00,
        adansa0::Ansa00,
        Adansa0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adansa0::Ansa00,
            adansa0::Ansa00,
            Adansa0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ansa01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adansa0::Ansa01,
        adansa0::Ansa01,
        Adansa0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adansa0::Ansa01,
            adansa0::Ansa01,
            Adansa0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ansa02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adansa0::Ansa02,
        adansa0::Ansa02,
        Adansa0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adansa0::Ansa02,
            adansa0::Ansa02,
            Adansa0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ansa03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adansa0::Ansa03,
        adansa0::Ansa03,
        Adansa0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adansa0::Ansa03,
            adansa0::Ansa03,
            Adansa0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ansa04(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adansa0::Ansa04,
        adansa0::Ansa04,
        Adansa0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adansa0::Ansa04,
            adansa0::Ansa04,
            Adansa0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ansa05(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adansa0::Ansa05,
        adansa0::Ansa05,
        Adansa0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adansa0::Ansa05,
            adansa0::Ansa05,
            Adansa0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ansa06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adansa0::Ansa06,
        adansa0::Ansa06,
        Adansa0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adansa0::Ansa06,
            adansa0::Ansa06,
            Adansa0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ansa07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adansa0::Ansa07,
        adansa0::Ansa07,
        Adansa0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adansa0::Ansa07,
            adansa0::Ansa07,
            Adansa0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ansa08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adansa0::Ansa08,
        adansa0::Ansa08,
        Adansa0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adansa0::Ansa08,
            adansa0::Ansa08,
            Adansa0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ansa09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        adansa0::Ansa09,
        adansa0::Ansa09,
        Adansa0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            adansa0::Ansa09,
            adansa0::Ansa09,
            Adansa0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ansa10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        adansa0::Ansa10,
        adansa0::Ansa10,
        Adansa0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            adansa0::Ansa10,
            adansa0::Ansa10,
            Adansa0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adansa0 {
    #[inline(always)]
    fn default() -> Adansa0 {
        <crate::RegValueT<Adansa0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adansa0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa00_SPEC;
    pub type Ansa00 = crate::EnumBitfieldStruct<u8, Ansa00_SPEC>;
    impl Ansa00 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa01_SPEC;
    pub type Ansa01 = crate::EnumBitfieldStruct<u8, Ansa01_SPEC>;
    impl Ansa01 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa02_SPEC;
    pub type Ansa02 = crate::EnumBitfieldStruct<u8, Ansa02_SPEC>;
    impl Ansa02 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa03_SPEC;
    pub type Ansa03 = crate::EnumBitfieldStruct<u8, Ansa03_SPEC>;
    impl Ansa03 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa04_SPEC;
    pub type Ansa04 = crate::EnumBitfieldStruct<u8, Ansa04_SPEC>;
    impl Ansa04 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa05_SPEC;
    pub type Ansa05 = crate::EnumBitfieldStruct<u8, Ansa05_SPEC>;
    impl Ansa05 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa06_SPEC;
    pub type Ansa06 = crate::EnumBitfieldStruct<u8, Ansa06_SPEC>;
    impl Ansa06 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa07_SPEC;
    pub type Ansa07 = crate::EnumBitfieldStruct<u8, Ansa07_SPEC>;
    impl Ansa07 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa08_SPEC;
    pub type Ansa08 = crate::EnumBitfieldStruct<u8, Ansa08_SPEC>;
    impl Ansa08 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa09_SPEC;
    pub type Ansa09 = crate::EnumBitfieldStruct<u8, Ansa09_SPEC>;
    impl Ansa09 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa10_SPEC;
    pub type Ansa10 = crate::EnumBitfieldStruct<u8, Ansa10_SPEC>;
    impl Ansa10 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adansa1_SPEC;
impl crate::sealed::RegSpec for Adansa1_SPEC {
    type DataType = u16;
}

pub type Adansa1 = crate::RegValueT<Adansa1_SPEC>;

impl Adansa1 {
    #[inline(always)]
    pub fn ansa16(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adansa1::Ansa16,
        adansa1::Ansa16,
        Adansa1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adansa1::Ansa16,
            adansa1::Ansa16,
            Adansa1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ansa17(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adansa1::Ansa17,
        adansa1::Ansa17,
        Adansa1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adansa1::Ansa17,
            adansa1::Ansa17,
            Adansa1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ansa18(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adansa1::Ansa18,
        adansa1::Ansa18,
        Adansa1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adansa1::Ansa18,
            adansa1::Ansa18,
            Adansa1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ansa19(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adansa1::Ansa19,
        adansa1::Ansa19,
        Adansa1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adansa1::Ansa19,
            adansa1::Ansa19,
            Adansa1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ansa20(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adansa1::Ansa20,
        adansa1::Ansa20,
        Adansa1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adansa1::Ansa20,
            adansa1::Ansa20,
            Adansa1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ansa21(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adansa1::Ansa21,
        adansa1::Ansa21,
        Adansa1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adansa1::Ansa21,
            adansa1::Ansa21,
            Adansa1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ansa22(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adansa1::Ansa22,
        adansa1::Ansa22,
        Adansa1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adansa1::Ansa22,
            adansa1::Ansa22,
            Adansa1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adansa1 {
    #[inline(always)]
    fn default() -> Adansa1 {
        <crate::RegValueT<Adansa1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adansa1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa16_SPEC;
    pub type Ansa16 = crate::EnumBitfieldStruct<u8, Ansa16_SPEC>;
    impl Ansa16 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa17_SPEC;
    pub type Ansa17 = crate::EnumBitfieldStruct<u8, Ansa17_SPEC>;
    impl Ansa17 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa18_SPEC;
    pub type Ansa18 = crate::EnumBitfieldStruct<u8, Ansa18_SPEC>;
    impl Ansa18 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa19_SPEC;
    pub type Ansa19 = crate::EnumBitfieldStruct<u8, Ansa19_SPEC>;
    impl Ansa19 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa20_SPEC;
    pub type Ansa20 = crate::EnumBitfieldStruct<u8, Ansa20_SPEC>;
    impl Ansa20 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa21_SPEC;
    pub type Ansa21 = crate::EnumBitfieldStruct<u8, Ansa21_SPEC>;
    impl Ansa21 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansa22_SPEC;
    pub type Ansa22 = crate::EnumBitfieldStruct<u8, Ansa22_SPEC>;
    impl Ansa22 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adads0_SPEC;
impl crate::sealed::RegSpec for Adads0_SPEC {
    type DataType = u16;
}

pub type Adads0 = crate::RegValueT<Adads0_SPEC>;

impl Adads0 {
    #[inline(always)]
    pub fn ads00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adads0::Ads00,
        adads0::Ads00,
        Adads0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adads0::Ads00,
            adads0::Ads00,
            Adads0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ads01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adads0::Ads01,
        adads0::Ads01,
        Adads0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adads0::Ads01,
            adads0::Ads01,
            Adads0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ads02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adads0::Ads02,
        adads0::Ads02,
        Adads0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adads0::Ads02,
            adads0::Ads02,
            Adads0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ads03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adads0::Ads03,
        adads0::Ads03,
        Adads0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adads0::Ads03,
            adads0::Ads03,
            Adads0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ads04(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adads0::Ads04,
        adads0::Ads04,
        Adads0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adads0::Ads04,
            adads0::Ads04,
            Adads0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ads05(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adads0::Ads05,
        adads0::Ads05,
        Adads0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adads0::Ads05,
            adads0::Ads05,
            Adads0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ads06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adads0::Ads06,
        adads0::Ads06,
        Adads0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adads0::Ads06,
            adads0::Ads06,
            Adads0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ads07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adads0::Ads07,
        adads0::Ads07,
        Adads0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adads0::Ads07,
            adads0::Ads07,
            Adads0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ads08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adads0::Ads08,
        adads0::Ads08,
        Adads0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adads0::Ads08,
            adads0::Ads08,
            Adads0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ads09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        adads0::Ads09,
        adads0::Ads09,
        Adads0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            adads0::Ads09,
            adads0::Ads09,
            Adads0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ads10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        adads0::Ads10,
        adads0::Ads10,
        Adads0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            adads0::Ads10,
            adads0::Ads10,
            Adads0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adads0 {
    #[inline(always)]
    fn default() -> Adads0 {
        <crate::RegValueT<Adads0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adads0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads00_SPEC;
    pub type Ads00 = crate::EnumBitfieldStruct<u8, Ads00_SPEC>;
    impl Ads00 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads01_SPEC;
    pub type Ads01 = crate::EnumBitfieldStruct<u8, Ads01_SPEC>;
    impl Ads01 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads02_SPEC;
    pub type Ads02 = crate::EnumBitfieldStruct<u8, Ads02_SPEC>;
    impl Ads02 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads03_SPEC;
    pub type Ads03 = crate::EnumBitfieldStruct<u8, Ads03_SPEC>;
    impl Ads03 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads04_SPEC;
    pub type Ads04 = crate::EnumBitfieldStruct<u8, Ads04_SPEC>;
    impl Ads04 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads05_SPEC;
    pub type Ads05 = crate::EnumBitfieldStruct<u8, Ads05_SPEC>;
    impl Ads05 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads06_SPEC;
    pub type Ads06 = crate::EnumBitfieldStruct<u8, Ads06_SPEC>;
    impl Ads06 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads07_SPEC;
    pub type Ads07 = crate::EnumBitfieldStruct<u8, Ads07_SPEC>;
    impl Ads07 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads08_SPEC;
    pub type Ads08 = crate::EnumBitfieldStruct<u8, Ads08_SPEC>;
    impl Ads08 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads09_SPEC;
    pub type Ads09 = crate::EnumBitfieldStruct<u8, Ads09_SPEC>;
    impl Ads09 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads10_SPEC;
    pub type Ads10 = crate::EnumBitfieldStruct<u8, Ads10_SPEC>;
    impl Ads10 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adads1_SPEC;
impl crate::sealed::RegSpec for Adads1_SPEC {
    type DataType = u16;
}

pub type Adads1 = crate::RegValueT<Adads1_SPEC>;

impl Adads1 {
    #[inline(always)]
    pub fn ads16(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adads1::Ads16,
        adads1::Ads16,
        Adads1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adads1::Ads16,
            adads1::Ads16,
            Adads1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ads17(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adads1::Ads17,
        adads1::Ads17,
        Adads1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adads1::Ads17,
            adads1::Ads17,
            Adads1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ads18(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adads1::Ads18,
        adads1::Ads18,
        Adads1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adads1::Ads18,
            adads1::Ads18,
            Adads1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ads19(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adads1::Ads19,
        adads1::Ads19,
        Adads1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adads1::Ads19,
            adads1::Ads19,
            Adads1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ads20(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adads1::Ads20,
        adads1::Ads20,
        Adads1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adads1::Ads20,
            adads1::Ads20,
            Adads1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ads21(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adads1::Ads21,
        adads1::Ads21,
        Adads1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adads1::Ads21,
            adads1::Ads21,
            Adads1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ads22(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adads1::Ads22,
        adads1::Ads22,
        Adads1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adads1::Ads22,
            adads1::Ads22,
            Adads1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adads1 {
    #[inline(always)]
    fn default() -> Adads1 {
        <crate::RegValueT<Adads1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adads1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads16_SPEC;
    pub type Ads16 = crate::EnumBitfieldStruct<u8, Ads16_SPEC>;
    impl Ads16 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads17_SPEC;
    pub type Ads17 = crate::EnumBitfieldStruct<u8, Ads17_SPEC>;
    impl Ads17 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads18_SPEC;
    pub type Ads18 = crate::EnumBitfieldStruct<u8, Ads18_SPEC>;
    impl Ads18 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads19_SPEC;
    pub type Ads19 = crate::EnumBitfieldStruct<u8, Ads19_SPEC>;
    impl Ads19 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads20_SPEC;
    pub type Ads20 = crate::EnumBitfieldStruct<u8, Ads20_SPEC>;
    impl Ads20 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads21_SPEC;
    pub type Ads21 = crate::EnumBitfieldStruct<u8, Ads21_SPEC>;
    impl Ads21 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ads22_SPEC;
    pub type Ads22 = crate::EnumBitfieldStruct<u8, Ads22_SPEC>;
    impl Ads22 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adadc_SPEC;
impl crate::sealed::RegSpec for Adadc_SPEC {
    type DataType = u8;
}

pub type Adadc = crate::RegValueT<Adadc_SPEC>;

impl Adadc {
    #[inline(always)]
    pub fn adc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        adadc::Adc,
        adadc::Adc,
        Adadc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            adadc::Adc,
            adadc::Adc,
            Adadc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn avee(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adadc::Avee,
        adadc::Avee,
        Adadc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adadc::Avee,
            adadc::Avee,
            Adadc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adadc {
    #[inline(always)]
    fn default() -> Adadc {
        <crate::RegValueT<Adadc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adadc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adc_SPEC;
    pub type Adc = crate::EnumBitfieldStruct<u8, Adc_SPEC>;
    impl Adc {
        pub const _000: Self = Self::new(0);

        pub const _001: Self = Self::new(1);

        pub const _010: Self = Self::new(2);

        pub const _011: Self = Self::new(3);

        pub const _101: Self = Self::new(5);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Avee_SPEC;
    pub type Avee = crate::EnumBitfieldStruct<u8, Avee_SPEC>;
    impl Avee {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcer_SPEC;
impl crate::sealed::RegSpec for Adcer_SPEC {
    type DataType = u16;
}

pub type Adcer = crate::RegValueT<Adcer_SPEC>;

impl Adcer {
    #[inline(always)]
    pub fn ace(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adcer::Ace,
        adcer::Ace,
        Adcer_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adcer::Ace,
            adcer::Ace,
            Adcer_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn diagval(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        adcer::Diagval,
        adcer::Diagval,
        Adcer_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            adcer::Diagval,
            adcer::Diagval,
            Adcer_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn diagld(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        adcer::Diagld,
        adcer::Diagld,
        Adcer_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            adcer::Diagld,
            adcer::Diagld,
            Adcer_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn diagm(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        adcer::Diagm,
        adcer::Diagm,
        Adcer_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            adcer::Diagm,
            adcer::Diagm,
            Adcer_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adrfmt(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        adcer::Adrfmt,
        adcer::Adrfmt,
        Adcer_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            adcer::Adrfmt,
            adcer::Adrfmt,
            Adcer_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcer {
    #[inline(always)]
    fn default() -> Adcer {
        <crate::RegValueT<Adcer_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcer {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ace_SPEC;
    pub type Ace = crate::EnumBitfieldStruct<u8, Ace_SPEC>;
    impl Ace {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Diagval_SPEC;
    pub type Diagval = crate::EnumBitfieldStruct<u8, Diagval_SPEC>;
    impl Diagval {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Diagld_SPEC;
    pub type Diagld = crate::EnumBitfieldStruct<u8, Diagld_SPEC>;
    impl Diagld {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Diagm_SPEC;
    pub type Diagm = crate::EnumBitfieldStruct<u8, Diagm_SPEC>;
    impl Diagm {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adrfmt_SPEC;
    pub type Adrfmt = crate::EnumBitfieldStruct<u8, Adrfmt_SPEC>;
    impl Adrfmt {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adstrgr_SPEC;
impl crate::sealed::RegSpec for Adstrgr_SPEC {
    type DataType = u16;
}

pub type Adstrgr = crate::RegValueT<Adstrgr_SPEC>;

impl Adstrgr {
    #[inline(always)]
    pub fn trsb(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Adstrgr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Adstrgr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn trsa(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, u8, Adstrgr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3f,1,0,u8,u8,Adstrgr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adstrgr {
    #[inline(always)]
    fn default() -> Adstrgr {
        <crate::RegValueT<Adstrgr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adexicr_SPEC;
impl crate::sealed::RegSpec for Adexicr_SPEC {
    type DataType = u16;
}

pub type Adexicr = crate::RegValueT<Adexicr_SPEC>;

impl Adexicr {
    #[inline(always)]
    pub fn tssad(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adexicr::Tssad,
        adexicr::Tssad,
        Adexicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adexicr::Tssad,
            adexicr::Tssad,
            Adexicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ocsad(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adexicr::Ocsad,
        adexicr::Ocsad,
        Adexicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adexicr::Ocsad,
            adexicr::Ocsad,
            Adexicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tssa(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adexicr::Tssa,
        adexicr::Tssa,
        Adexicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adexicr::Tssa,
            adexicr::Tssa,
            Adexicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ocsa(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        adexicr::Ocsa,
        adexicr::Ocsa,
        Adexicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            adexicr::Ocsa,
            adexicr::Ocsa,
            Adexicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adexicr {
    #[inline(always)]
    fn default() -> Adexicr {
        <crate::RegValueT<Adexicr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adexicr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tssad_SPEC;
    pub type Tssad = crate::EnumBitfieldStruct<u8, Tssad_SPEC>;
    impl Tssad {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ocsad_SPEC;
    pub type Ocsad = crate::EnumBitfieldStruct<u8, Ocsad_SPEC>;
    impl Ocsad {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tssa_SPEC;
    pub type Tssa = crate::EnumBitfieldStruct<u8, Tssa_SPEC>;
    impl Tssa {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ocsa_SPEC;
    pub type Ocsa = crate::EnumBitfieldStruct<u8, Ocsa_SPEC>;
    impl Ocsa {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adansb0_SPEC;
impl crate::sealed::RegSpec for Adansb0_SPEC {
    type DataType = u16;
}

pub type Adansb0 = crate::RegValueT<Adansb0_SPEC>;

impl Adansb0 {
    #[inline(always)]
    pub fn ansb00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adansb0::Ansb00,
        adansb0::Ansb00,
        Adansb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adansb0::Ansb00,
            adansb0::Ansb00,
            Adansb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ansb01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adansb0::Ansb01,
        adansb0::Ansb01,
        Adansb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adansb0::Ansb01,
            adansb0::Ansb01,
            Adansb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ansb02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adansb0::Ansb02,
        adansb0::Ansb02,
        Adansb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adansb0::Ansb02,
            adansb0::Ansb02,
            Adansb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ansb03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adansb0::Ansb03,
        adansb0::Ansb03,
        Adansb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adansb0::Ansb03,
            adansb0::Ansb03,
            Adansb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ansb04(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adansb0::Ansb04,
        adansb0::Ansb04,
        Adansb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adansb0::Ansb04,
            adansb0::Ansb04,
            Adansb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ansb05(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adansb0::Ansb05,
        adansb0::Ansb05,
        Adansb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adansb0::Ansb05,
            adansb0::Ansb05,
            Adansb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ansb06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adansb0::Ansb06,
        adansb0::Ansb06,
        Adansb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adansb0::Ansb06,
            adansb0::Ansb06,
            Adansb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ansb07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adansb0::Ansb07,
        adansb0::Ansb07,
        Adansb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adansb0::Ansb07,
            adansb0::Ansb07,
            Adansb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ansb08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adansb0::Ansb08,
        adansb0::Ansb08,
        Adansb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adansb0::Ansb08,
            adansb0::Ansb08,
            Adansb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ansb09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        adansb0::Ansb09,
        adansb0::Ansb09,
        Adansb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            adansb0::Ansb09,
            adansb0::Ansb09,
            Adansb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ansb10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        adansb0::Ansb10,
        adansb0::Ansb10,
        Adansb0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            adansb0::Ansb10,
            adansb0::Ansb10,
            Adansb0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adansb0 {
    #[inline(always)]
    fn default() -> Adansb0 {
        <crate::RegValueT<Adansb0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adansb0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb00_SPEC;
    pub type Ansb00 = crate::EnumBitfieldStruct<u8, Ansb00_SPEC>;
    impl Ansb00 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb01_SPEC;
    pub type Ansb01 = crate::EnumBitfieldStruct<u8, Ansb01_SPEC>;
    impl Ansb01 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb02_SPEC;
    pub type Ansb02 = crate::EnumBitfieldStruct<u8, Ansb02_SPEC>;
    impl Ansb02 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb03_SPEC;
    pub type Ansb03 = crate::EnumBitfieldStruct<u8, Ansb03_SPEC>;
    impl Ansb03 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb04_SPEC;
    pub type Ansb04 = crate::EnumBitfieldStruct<u8, Ansb04_SPEC>;
    impl Ansb04 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb05_SPEC;
    pub type Ansb05 = crate::EnumBitfieldStruct<u8, Ansb05_SPEC>;
    impl Ansb05 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb06_SPEC;
    pub type Ansb06 = crate::EnumBitfieldStruct<u8, Ansb06_SPEC>;
    impl Ansb06 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb07_SPEC;
    pub type Ansb07 = crate::EnumBitfieldStruct<u8, Ansb07_SPEC>;
    impl Ansb07 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb08_SPEC;
    pub type Ansb08 = crate::EnumBitfieldStruct<u8, Ansb08_SPEC>;
    impl Ansb08 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb09_SPEC;
    pub type Ansb09 = crate::EnumBitfieldStruct<u8, Ansb09_SPEC>;
    impl Ansb09 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb10_SPEC;
    pub type Ansb10 = crate::EnumBitfieldStruct<u8, Ansb10_SPEC>;
    impl Ansb10 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adansb1_SPEC;
impl crate::sealed::RegSpec for Adansb1_SPEC {
    type DataType = u16;
}

pub type Adansb1 = crate::RegValueT<Adansb1_SPEC>;

impl Adansb1 {
    #[inline(always)]
    pub fn ansb16(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adansb1::Ansb16,
        adansb1::Ansb16,
        Adansb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adansb1::Ansb16,
            adansb1::Ansb16,
            Adansb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ansb17(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adansb1::Ansb17,
        adansb1::Ansb17,
        Adansb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adansb1::Ansb17,
            adansb1::Ansb17,
            Adansb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ansb18(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adansb1::Ansb18,
        adansb1::Ansb18,
        Adansb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adansb1::Ansb18,
            adansb1::Ansb18,
            Adansb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ansb19(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adansb1::Ansb19,
        adansb1::Ansb19,
        Adansb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adansb1::Ansb19,
            adansb1::Ansb19,
            Adansb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ansb20(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adansb1::Ansb20,
        adansb1::Ansb20,
        Adansb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adansb1::Ansb20,
            adansb1::Ansb20,
            Adansb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ansb21(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adansb1::Ansb21,
        adansb1::Ansb21,
        Adansb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adansb1::Ansb21,
            adansb1::Ansb21,
            Adansb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ansb22(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adansb1::Ansb22,
        adansb1::Ansb22,
        Adansb1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adansb1::Ansb22,
            adansb1::Ansb22,
            Adansb1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adansb1 {
    #[inline(always)]
    fn default() -> Adansb1 {
        <crate::RegValueT<Adansb1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adansb1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb16_SPEC;
    pub type Ansb16 = crate::EnumBitfieldStruct<u8, Ansb16_SPEC>;
    impl Ansb16 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb17_SPEC;
    pub type Ansb17 = crate::EnumBitfieldStruct<u8, Ansb17_SPEC>;
    impl Ansb17 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb18_SPEC;
    pub type Ansb18 = crate::EnumBitfieldStruct<u8, Ansb18_SPEC>;
    impl Ansb18 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb19_SPEC;
    pub type Ansb19 = crate::EnumBitfieldStruct<u8, Ansb19_SPEC>;
    impl Ansb19 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb20_SPEC;
    pub type Ansb20 = crate::EnumBitfieldStruct<u8, Ansb20_SPEC>;
    impl Ansb20 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb21_SPEC;
    pub type Ansb21 = crate::EnumBitfieldStruct<u8, Ansb21_SPEC>;
    impl Ansb21 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ansb22_SPEC;
    pub type Ansb22 = crate::EnumBitfieldStruct<u8, Ansb22_SPEC>;
    impl Ansb22 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Addbldr_SPEC;
impl crate::sealed::RegSpec for Addbldr_SPEC {
    type DataType = u16;
}

pub type Addbldr = crate::RegValueT<Addbldr_SPEC>;

impl Addbldr {
    #[inline(always)]
    pub fn addbldr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Addbldr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Addbldr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Addbldr {
    #[inline(always)]
    fn default() -> Addbldr {
        <crate::RegValueT<Addbldr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adtsdr_SPEC;
impl crate::sealed::RegSpec for Adtsdr_SPEC {
    type DataType = u16;
}

pub type Adtsdr = crate::RegValueT<Adtsdr_SPEC>;

impl Adtsdr {
    #[inline(always)]
    pub fn adtsdr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Adtsdr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Adtsdr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Adtsdr {
    #[inline(always)]
    fn default() -> Adtsdr {
        <crate::RegValueT<Adtsdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adocdr_SPEC;
impl crate::sealed::RegSpec for Adocdr_SPEC {
    type DataType = u16;
}

pub type Adocdr = crate::RegValueT<Adocdr_SPEC>;

impl Adocdr {
    #[inline(always)]
    pub fn adocdr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Adocdr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Adocdr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Adocdr {
    #[inline(always)]
    fn default() -> Adocdr {
        <crate::RegValueT<Adocdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adrd_SPEC;
impl crate::sealed::RegSpec for Adrd_SPEC {
    type DataType = u16;
}

pub type Adrd = crate::RegValueT<Adrd_SPEC>;

impl Adrd {
    #[inline(always)]
    pub fn ad(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, Adrd_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,Adrd_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diagst(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        adrd::Diagst,
        adrd::Diagst,
        Adrd_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            adrd::Diagst,
            adrd::Diagst,
            Adrd_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adrd {
    #[inline(always)]
    fn default() -> Adrd {
        <crate::RegValueT<Adrd_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adrd {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Diagst_SPEC;
    pub type Diagst = crate::EnumBitfieldStruct<u8, Diagst_SPEC>;
    impl Diagst {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adctdr_SPEC;
impl crate::sealed::RegSpec for Adctdr_SPEC {
    type DataType = u16;
}

pub type Adctdr = crate::RegValueT<Adctdr_SPEC>;

impl Adctdr {
    #[inline(always)]
    pub fn adctdr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Adctdr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Adctdr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Adctdr {
    #[inline(always)]
    fn default() -> Adctdr {
        <crate::RegValueT<Adctdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Addr_SPEC;
impl crate::sealed::RegSpec for Addr_SPEC {
    type DataType = u16;
}

pub type Addr = crate::RegValueT<Addr_SPEC>;

impl Addr {
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Addr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Addr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Addr {
    #[inline(always)]
    fn default() -> Addr {
        <crate::RegValueT<Addr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Addiscr_SPEC;
impl crate::sealed::RegSpec for Addiscr_SPEC {
    type DataType = u8;
}

pub type Addiscr = crate::RegValueT<Addiscr_SPEC>;

impl Addiscr {
    #[inline(always)]
    pub fn adndis(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        addiscr::Adndis,
        addiscr::Adndis,
        Addiscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            addiscr::Adndis,
            addiscr::Adndis,
            Addiscr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pchg(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        addiscr::Pchg,
        addiscr::Pchg,
        Addiscr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            addiscr::Pchg,
            addiscr::Pchg,
            Addiscr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Addiscr {
    #[inline(always)]
    fn default() -> Addiscr {
        <crate::RegValueT<Addiscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod addiscr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adndis_SPEC;
    pub type Adndis = crate::EnumBitfieldStruct<u8, Adndis_SPEC>;
    impl Adndis {
        pub const _0_X_0: Self = Self::new(0);

        pub const _0_X_1: Self = Self::new(1);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pchg_SPEC;
    pub type Pchg = crate::EnumBitfieldStruct<u8, Pchg_SPEC>;
    impl Pchg {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adacsr_SPEC;
impl crate::sealed::RegSpec for Adacsr_SPEC {
    type DataType = u8;
}

pub type Adacsr = crate::RegValueT<Adacsr_SPEC>;

impl Adacsr {
    #[inline(always)]
    pub fn adsac(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adacsr::Adsac,
        adacsr::Adsac,
        Adacsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adacsr::Adsac,
            adacsr::Adsac,
            Adacsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adacsr {
    #[inline(always)]
    fn default() -> Adacsr {
        <crate::RegValueT<Adacsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adacsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adsac_SPEC;
    pub type Adsac = crate::EnumBitfieldStruct<u8, Adsac_SPEC>;
    impl Adsac {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adgspcr_SPEC;
impl crate::sealed::RegSpec for Adgspcr_SPEC {
    type DataType = u16;
}

pub type Adgspcr = crate::RegValueT<Adgspcr_SPEC>;

impl Adgspcr {
    #[inline(always)]
    pub fn pgs(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adgspcr::Pgs,
        adgspcr::Pgs,
        Adgspcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adgspcr::Pgs,
            adgspcr::Pgs,
            Adgspcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gbrscn(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adgspcr::Gbrscn,
        adgspcr::Gbrscn,
        Adgspcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adgspcr::Gbrscn,
            adgspcr::Gbrscn,
            Adgspcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn gbrp(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        adgspcr::Gbrp,
        adgspcr::Gbrp,
        Adgspcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            adgspcr::Gbrp,
            adgspcr::Gbrp,
            Adgspcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adgspcr {
    #[inline(always)]
    fn default() -> Adgspcr {
        <crate::RegValueT<Adgspcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adgspcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pgs_SPEC;
    pub type Pgs = crate::EnumBitfieldStruct<u8, Pgs_SPEC>;
    impl Pgs {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gbrscn_SPEC;
    pub type Gbrscn = crate::EnumBitfieldStruct<u8, Gbrscn_SPEC>;
    impl Gbrscn {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gbrp_SPEC;
    pub type Gbrp = crate::EnumBitfieldStruct<u8, Gbrp_SPEC>;
    impl Gbrp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Addbldra_SPEC;
impl crate::sealed::RegSpec for Addbldra_SPEC {
    type DataType = u16;
}

pub type Addbldra = crate::RegValueT<Addbldra_SPEC>;

impl Addbldra {
    #[inline(always)]
    pub fn addbldr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Addbldra_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Addbldra_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Addbldra {
    #[inline(always)]
    fn default() -> Addbldra {
        <crate::RegValueT<Addbldra_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Addbldrb_SPEC;
impl crate::sealed::RegSpec for Addbldrb_SPEC {
    type DataType = u16;
}

pub type Addbldrb = crate::RegValueT<Addbldrb_SPEC>;

impl Addbldrb {
    #[inline(always)]
    pub fn addbldr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Addbldrb_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Addbldrb_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Addbldrb {
    #[inline(always)]
    fn default() -> Addbldrb {
        <crate::RegValueT<Addbldrb_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adhvrefcnt_SPEC;
impl crate::sealed::RegSpec for Adhvrefcnt_SPEC {
    type DataType = u8;
}

pub type Adhvrefcnt = crate::RegValueT<Adhvrefcnt_SPEC>;

impl Adhvrefcnt {
    #[inline(always)]
    pub fn hvsel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        adhvrefcnt::Hvsel,
        adhvrefcnt::Hvsel,
        Adhvrefcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            adhvrefcnt::Hvsel,
            adhvrefcnt::Hvsel,
            Adhvrefcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lvsel(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adhvrefcnt::Lvsel,
        adhvrefcnt::Lvsel,
        Adhvrefcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adhvrefcnt::Lvsel,
            adhvrefcnt::Lvsel,
            Adhvrefcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn adslp(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adhvrefcnt::Adslp,
        adhvrefcnt::Adslp,
        Adhvrefcnt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adhvrefcnt::Adslp,
            adhvrefcnt::Adslp,
            Adhvrefcnt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adhvrefcnt {
    #[inline(always)]
    fn default() -> Adhvrefcnt {
        <crate::RegValueT<Adhvrefcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adhvrefcnt {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hvsel_SPEC;
    pub type Hvsel = crate::EnumBitfieldStruct<u8, Hvsel_SPEC>;
    impl Hvsel {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lvsel_SPEC;
    pub type Lvsel = crate::EnumBitfieldStruct<u8, Lvsel_SPEC>;
    impl Lvsel {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Adslp_SPEC;
    pub type Adslp = crate::EnumBitfieldStruct<u8, Adslp_SPEC>;
    impl Adslp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adwinmon_SPEC;
impl crate::sealed::RegSpec for Adwinmon_SPEC {
    type DataType = u8;
}

pub type Adwinmon = crate::RegValueT<Adwinmon_SPEC>;

impl Adwinmon {
    #[inline(always)]
    pub fn moncomb(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adwinmon::Moncomb,
        adwinmon::Moncomb,
        Adwinmon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adwinmon::Moncomb,
            adwinmon::Moncomb,
            Adwinmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn moncmpa(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adwinmon::Moncmpa,
        adwinmon::Moncmpa,
        Adwinmon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adwinmon::Moncmpa,
            adwinmon::Moncmpa,
            Adwinmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn moncmpb(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adwinmon::Moncmpb,
        adwinmon::Moncmpb,
        Adwinmon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adwinmon::Moncmpb,
            adwinmon::Moncmpb,
            Adwinmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adwinmon {
    #[inline(always)]
    fn default() -> Adwinmon {
        <crate::RegValueT<Adwinmon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adwinmon {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Moncomb_SPEC;
    pub type Moncomb = crate::EnumBitfieldStruct<u8, Moncomb_SPEC>;
    impl Moncomb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Moncmpa_SPEC;
    pub type Moncmpa = crate::EnumBitfieldStruct<u8, Moncmpa_SPEC>;
    impl Moncmpa {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Moncmpb_SPEC;
    pub type Moncmpb = crate::EnumBitfieldStruct<u8, Moncmpb_SPEC>;
    impl Moncmpb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpcr_SPEC;
impl crate::sealed::RegSpec for Adcmpcr_SPEC {
    type DataType = u16;
}

pub type Adcmpcr = crate::RegValueT<Adcmpcr_SPEC>;

impl Adcmpcr {
    #[inline(always)]
    pub fn cmpab(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        adcmpcr::Cmpab,
        adcmpcr::Cmpab,
        Adcmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            adcmpcr::Cmpab,
            adcmpcr::Cmpab,
            Adcmpcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpbe(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        adcmpcr::Cmpbe,
        adcmpcr::Cmpbe,
        Adcmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            adcmpcr::Cmpbe,
            adcmpcr::Cmpbe,
            Adcmpcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpae(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        adcmpcr::Cmpae,
        adcmpcr::Cmpae,
        Adcmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            adcmpcr::Cmpae,
            adcmpcr::Cmpae,
            Adcmpcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpbie(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        adcmpcr::Cmpbie,
        adcmpcr::Cmpbie,
        Adcmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            adcmpcr::Cmpbie,
            adcmpcr::Cmpbie,
            Adcmpcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn wcmpe(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        adcmpcr::Wcmpe,
        adcmpcr::Wcmpe,
        Adcmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            adcmpcr::Wcmpe,
            adcmpcr::Wcmpe,
            Adcmpcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpaie(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        adcmpcr::Cmpaie,
        adcmpcr::Cmpaie,
        Adcmpcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            adcmpcr::Cmpaie,
            adcmpcr::Cmpaie,
            Adcmpcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcmpcr {
    #[inline(always)]
    fn default() -> Adcmpcr {
        <crate::RegValueT<Adcmpcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmpcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpab_SPEC;
    pub type Cmpab = crate::EnumBitfieldStruct<u8, Cmpab_SPEC>;
    impl Cmpab {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpbe_SPEC;
    pub type Cmpbe = crate::EnumBitfieldStruct<u8, Cmpbe_SPEC>;
    impl Cmpbe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpae_SPEC;
    pub type Cmpae = crate::EnumBitfieldStruct<u8, Cmpae_SPEC>;
    impl Cmpae {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpbie_SPEC;
    pub type Cmpbie = crate::EnumBitfieldStruct<u8, Cmpbie_SPEC>;
    impl Cmpbie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wcmpe_SPEC;
    pub type Wcmpe = crate::EnumBitfieldStruct<u8, Wcmpe_SPEC>;
    impl Wcmpe {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpaie_SPEC;
    pub type Cmpaie = crate::EnumBitfieldStruct<u8, Cmpaie_SPEC>;
    impl Cmpaie {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpanser_SPEC;
impl crate::sealed::RegSpec for Adcmpanser_SPEC {
    type DataType = u8;
}

pub type Adcmpanser = crate::RegValueT<Adcmpanser_SPEC>;

impl Adcmpanser {
    #[inline(always)]
    pub fn cmptsa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adcmpanser::Cmptsa,
        adcmpanser::Cmptsa,
        Adcmpanser_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adcmpanser::Cmptsa,
            adcmpanser::Cmptsa,
            Adcmpanser_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpoca(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adcmpanser::Cmpoca,
        adcmpanser::Cmpoca,
        Adcmpanser_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adcmpanser::Cmpoca,
            adcmpanser::Cmpoca,
            Adcmpanser_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcmpanser {
    #[inline(always)]
    fn default() -> Adcmpanser {
        <crate::RegValueT<Adcmpanser_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmpanser {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmptsa_SPEC;
    pub type Cmptsa = crate::EnumBitfieldStruct<u8, Cmptsa_SPEC>;
    impl Cmptsa {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpoca_SPEC;
    pub type Cmpoca = crate::EnumBitfieldStruct<u8, Cmpoca_SPEC>;
    impl Cmpoca {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpler_SPEC;
impl crate::sealed::RegSpec for Adcmpler_SPEC {
    type DataType = u8;
}

pub type Adcmpler = crate::RegValueT<Adcmpler_SPEC>;

impl Adcmpler {
    #[inline(always)]
    pub fn cmpltsa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adcmpler::Cmpltsa,
        adcmpler::Cmpltsa,
        Adcmpler_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adcmpler::Cmpltsa,
            adcmpler::Cmpltsa,
            Adcmpler_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmploca(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adcmpler::Cmploca,
        adcmpler::Cmploca,
        Adcmpler_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adcmpler::Cmploca,
            adcmpler::Cmploca,
            Adcmpler_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcmpler {
    #[inline(always)]
    fn default() -> Adcmpler {
        <crate::RegValueT<Adcmpler_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmpler {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpltsa_SPEC;
    pub type Cmpltsa = crate::EnumBitfieldStruct<u8, Cmpltsa_SPEC>;
    impl Cmpltsa {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmploca_SPEC;
    pub type Cmploca = crate::EnumBitfieldStruct<u8, Cmploca_SPEC>;
    impl Cmploca {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpansr0_SPEC;
impl crate::sealed::RegSpec for Adcmpansr0_SPEC {
    type DataType = u16;
}

pub type Adcmpansr0 = crate::RegValueT<Adcmpansr0_SPEC>;

impl Adcmpansr0 {
    #[inline(always)]
    pub fn cmpcha00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adcmpansr0::Cmpcha00,
        adcmpansr0::Cmpcha00,
        Adcmpansr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adcmpansr0::Cmpcha00,
            adcmpansr0::Cmpcha00,
            Adcmpansr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpcha01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adcmpansr0::Cmpcha01,
        adcmpansr0::Cmpcha01,
        Adcmpansr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adcmpansr0::Cmpcha01,
            adcmpansr0::Cmpcha01,
            Adcmpansr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpcha02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adcmpansr0::Cmpcha02,
        adcmpansr0::Cmpcha02,
        Adcmpansr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adcmpansr0::Cmpcha02,
            adcmpansr0::Cmpcha02,
            Adcmpansr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpcha03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adcmpansr0::Cmpcha03,
        adcmpansr0::Cmpcha03,
        Adcmpansr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adcmpansr0::Cmpcha03,
            adcmpansr0::Cmpcha03,
            Adcmpansr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpcha04(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adcmpansr0::Cmpcha04,
        adcmpansr0::Cmpcha04,
        Adcmpansr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adcmpansr0::Cmpcha04,
            adcmpansr0::Cmpcha04,
            Adcmpansr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpcha05(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adcmpansr0::Cmpcha05,
        adcmpansr0::Cmpcha05,
        Adcmpansr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adcmpansr0::Cmpcha05,
            adcmpansr0::Cmpcha05,
            Adcmpansr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpcha06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adcmpansr0::Cmpcha06,
        adcmpansr0::Cmpcha06,
        Adcmpansr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adcmpansr0::Cmpcha06,
            adcmpansr0::Cmpcha06,
            Adcmpansr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpcha07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adcmpansr0::Cmpcha07,
        adcmpansr0::Cmpcha07,
        Adcmpansr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adcmpansr0::Cmpcha07,
            adcmpansr0::Cmpcha07,
            Adcmpansr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpcha08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adcmpansr0::Cmpcha08,
        adcmpansr0::Cmpcha08,
        Adcmpansr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adcmpansr0::Cmpcha08,
            adcmpansr0::Cmpcha08,
            Adcmpansr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpcha09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        adcmpansr0::Cmpcha09,
        adcmpansr0::Cmpcha09,
        Adcmpansr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            adcmpansr0::Cmpcha09,
            adcmpansr0::Cmpcha09,
            Adcmpansr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpcha10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        adcmpansr0::Cmpcha10,
        adcmpansr0::Cmpcha10,
        Adcmpansr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            adcmpansr0::Cmpcha10,
            adcmpansr0::Cmpcha10,
            Adcmpansr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcmpansr0 {
    #[inline(always)]
    fn default() -> Adcmpansr0 {
        <crate::RegValueT<Adcmpansr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmpansr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha00_SPEC;
    pub type Cmpcha00 = crate::EnumBitfieldStruct<u8, Cmpcha00_SPEC>;
    impl Cmpcha00 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha01_SPEC;
    pub type Cmpcha01 = crate::EnumBitfieldStruct<u8, Cmpcha01_SPEC>;
    impl Cmpcha01 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha02_SPEC;
    pub type Cmpcha02 = crate::EnumBitfieldStruct<u8, Cmpcha02_SPEC>;
    impl Cmpcha02 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha03_SPEC;
    pub type Cmpcha03 = crate::EnumBitfieldStruct<u8, Cmpcha03_SPEC>;
    impl Cmpcha03 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha04_SPEC;
    pub type Cmpcha04 = crate::EnumBitfieldStruct<u8, Cmpcha04_SPEC>;
    impl Cmpcha04 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha05_SPEC;
    pub type Cmpcha05 = crate::EnumBitfieldStruct<u8, Cmpcha05_SPEC>;
    impl Cmpcha05 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha06_SPEC;
    pub type Cmpcha06 = crate::EnumBitfieldStruct<u8, Cmpcha06_SPEC>;
    impl Cmpcha06 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha07_SPEC;
    pub type Cmpcha07 = crate::EnumBitfieldStruct<u8, Cmpcha07_SPEC>;
    impl Cmpcha07 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha08_SPEC;
    pub type Cmpcha08 = crate::EnumBitfieldStruct<u8, Cmpcha08_SPEC>;
    impl Cmpcha08 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha09_SPEC;
    pub type Cmpcha09 = crate::EnumBitfieldStruct<u8, Cmpcha09_SPEC>;
    impl Cmpcha09 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha10_SPEC;
    pub type Cmpcha10 = crate::EnumBitfieldStruct<u8, Cmpcha10_SPEC>;
    impl Cmpcha10 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpansr1_SPEC;
impl crate::sealed::RegSpec for Adcmpansr1_SPEC {
    type DataType = u16;
}

pub type Adcmpansr1 = crate::RegValueT<Adcmpansr1_SPEC>;

impl Adcmpansr1 {
    #[inline(always)]
    pub fn cmpcha16(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adcmpansr1::Cmpcha16,
        adcmpansr1::Cmpcha16,
        Adcmpansr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adcmpansr1::Cmpcha16,
            adcmpansr1::Cmpcha16,
            Adcmpansr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpcha17(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adcmpansr1::Cmpcha17,
        adcmpansr1::Cmpcha17,
        Adcmpansr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adcmpansr1::Cmpcha17,
            adcmpansr1::Cmpcha17,
            Adcmpansr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpcha18(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adcmpansr1::Cmpcha18,
        adcmpansr1::Cmpcha18,
        Adcmpansr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adcmpansr1::Cmpcha18,
            adcmpansr1::Cmpcha18,
            Adcmpansr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpcha19(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adcmpansr1::Cmpcha19,
        adcmpansr1::Cmpcha19,
        Adcmpansr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adcmpansr1::Cmpcha19,
            adcmpansr1::Cmpcha19,
            Adcmpansr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpcha20(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adcmpansr1::Cmpcha20,
        adcmpansr1::Cmpcha20,
        Adcmpansr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adcmpansr1::Cmpcha20,
            adcmpansr1::Cmpcha20,
            Adcmpansr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpcha21(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adcmpansr1::Cmpcha21,
        adcmpansr1::Cmpcha21,
        Adcmpansr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adcmpansr1::Cmpcha21,
            adcmpansr1::Cmpcha21,
            Adcmpansr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpcha22(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adcmpansr1::Cmpcha22,
        adcmpansr1::Cmpcha22,
        Adcmpansr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adcmpansr1::Cmpcha22,
            adcmpansr1::Cmpcha22,
            Adcmpansr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcmpansr1 {
    #[inline(always)]
    fn default() -> Adcmpansr1 {
        <crate::RegValueT<Adcmpansr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmpansr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha16_SPEC;
    pub type Cmpcha16 = crate::EnumBitfieldStruct<u8, Cmpcha16_SPEC>;
    impl Cmpcha16 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha17_SPEC;
    pub type Cmpcha17 = crate::EnumBitfieldStruct<u8, Cmpcha17_SPEC>;
    impl Cmpcha17 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha18_SPEC;
    pub type Cmpcha18 = crate::EnumBitfieldStruct<u8, Cmpcha18_SPEC>;
    impl Cmpcha18 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha19_SPEC;
    pub type Cmpcha19 = crate::EnumBitfieldStruct<u8, Cmpcha19_SPEC>;
    impl Cmpcha19 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha20_SPEC;
    pub type Cmpcha20 = crate::EnumBitfieldStruct<u8, Cmpcha20_SPEC>;
    impl Cmpcha20 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha21_SPEC;
    pub type Cmpcha21 = crate::EnumBitfieldStruct<u8, Cmpcha21_SPEC>;
    impl Cmpcha21 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpcha22_SPEC;
    pub type Cmpcha22 = crate::EnumBitfieldStruct<u8, Cmpcha22_SPEC>;
    impl Cmpcha22 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmplr0_SPEC;
impl crate::sealed::RegSpec for Adcmplr0_SPEC {
    type DataType = u16;
}

pub type Adcmplr0 = crate::RegValueT<Adcmplr0_SPEC>;

impl Adcmplr0 {
    #[inline(always)]
    pub fn cmplcha00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adcmplr0::Cmplcha00,
        adcmplr0::Cmplcha00,
        Adcmplr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adcmplr0::Cmplcha00,
            adcmplr0::Cmplcha00,
            Adcmplr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmplcha01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adcmplr0::Cmplcha01,
        adcmplr0::Cmplcha01,
        Adcmplr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adcmplr0::Cmplcha01,
            adcmplr0::Cmplcha01,
            Adcmplr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmplcha02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adcmplr0::Cmplcha02,
        adcmplr0::Cmplcha02,
        Adcmplr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adcmplr0::Cmplcha02,
            adcmplr0::Cmplcha02,
            Adcmplr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmplcha03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adcmplr0::Cmplcha03,
        adcmplr0::Cmplcha03,
        Adcmplr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adcmplr0::Cmplcha03,
            adcmplr0::Cmplcha03,
            Adcmplr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmplcha04(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adcmplr0::Cmplcha04,
        adcmplr0::Cmplcha04,
        Adcmplr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adcmplr0::Cmplcha04,
            adcmplr0::Cmplcha04,
            Adcmplr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmplcha05(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adcmplr0::Cmplcha05,
        adcmplr0::Cmplcha05,
        Adcmplr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adcmplr0::Cmplcha05,
            adcmplr0::Cmplcha05,
            Adcmplr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmplcha06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adcmplr0::Cmplcha06,
        adcmplr0::Cmplcha06,
        Adcmplr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adcmplr0::Cmplcha06,
            adcmplr0::Cmplcha06,
            Adcmplr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmplcha07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adcmplr0::Cmplcha07,
        adcmplr0::Cmplcha07,
        Adcmplr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adcmplr0::Cmplcha07,
            adcmplr0::Cmplcha07,
            Adcmplr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmplcha08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adcmplr0::Cmplcha08,
        adcmplr0::Cmplcha08,
        Adcmplr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adcmplr0::Cmplcha08,
            adcmplr0::Cmplcha08,
            Adcmplr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmplcha09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        adcmplr0::Cmplcha09,
        adcmplr0::Cmplcha09,
        Adcmplr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            adcmplr0::Cmplcha09,
            adcmplr0::Cmplcha09,
            Adcmplr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmplcha10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        adcmplr0::Cmplcha10,
        adcmplr0::Cmplcha10,
        Adcmplr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            adcmplr0::Cmplcha10,
            adcmplr0::Cmplcha10,
            Adcmplr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcmplr0 {
    #[inline(always)]
    fn default() -> Adcmplr0 {
        <crate::RegValueT<Adcmplr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmplr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha00_SPEC;
    pub type Cmplcha00 = crate::EnumBitfieldStruct<u8, Cmplcha00_SPEC>;
    impl Cmplcha00 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha01_SPEC;
    pub type Cmplcha01 = crate::EnumBitfieldStruct<u8, Cmplcha01_SPEC>;
    impl Cmplcha01 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha02_SPEC;
    pub type Cmplcha02 = crate::EnumBitfieldStruct<u8, Cmplcha02_SPEC>;
    impl Cmplcha02 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha03_SPEC;
    pub type Cmplcha03 = crate::EnumBitfieldStruct<u8, Cmplcha03_SPEC>;
    impl Cmplcha03 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha04_SPEC;
    pub type Cmplcha04 = crate::EnumBitfieldStruct<u8, Cmplcha04_SPEC>;
    impl Cmplcha04 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha05_SPEC;
    pub type Cmplcha05 = crate::EnumBitfieldStruct<u8, Cmplcha05_SPEC>;
    impl Cmplcha05 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha06_SPEC;
    pub type Cmplcha06 = crate::EnumBitfieldStruct<u8, Cmplcha06_SPEC>;
    impl Cmplcha06 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha07_SPEC;
    pub type Cmplcha07 = crate::EnumBitfieldStruct<u8, Cmplcha07_SPEC>;
    impl Cmplcha07 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha08_SPEC;
    pub type Cmplcha08 = crate::EnumBitfieldStruct<u8, Cmplcha08_SPEC>;
    impl Cmplcha08 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha09_SPEC;
    pub type Cmplcha09 = crate::EnumBitfieldStruct<u8, Cmplcha09_SPEC>;
    impl Cmplcha09 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha10_SPEC;
    pub type Cmplcha10 = crate::EnumBitfieldStruct<u8, Cmplcha10_SPEC>;
    impl Cmplcha10 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmplr1_SPEC;
impl crate::sealed::RegSpec for Adcmplr1_SPEC {
    type DataType = u16;
}

pub type Adcmplr1 = crate::RegValueT<Adcmplr1_SPEC>;

impl Adcmplr1 {
    #[inline(always)]
    pub fn cmplcha16(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adcmplr1::Cmplcha16,
        adcmplr1::Cmplcha16,
        Adcmplr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adcmplr1::Cmplcha16,
            adcmplr1::Cmplcha16,
            Adcmplr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmplcha17(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adcmplr1::Cmplcha17,
        adcmplr1::Cmplcha17,
        Adcmplr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adcmplr1::Cmplcha17,
            adcmplr1::Cmplcha17,
            Adcmplr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmplcha18(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adcmplr1::Cmplcha18,
        adcmplr1::Cmplcha18,
        Adcmplr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adcmplr1::Cmplcha18,
            adcmplr1::Cmplcha18,
            Adcmplr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmplcha19(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adcmplr1::Cmplcha19,
        adcmplr1::Cmplcha19,
        Adcmplr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adcmplr1::Cmplcha19,
            adcmplr1::Cmplcha19,
            Adcmplr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmplcha20(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adcmplr1::Cmplcha20,
        adcmplr1::Cmplcha20,
        Adcmplr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adcmplr1::Cmplcha20,
            adcmplr1::Cmplcha20,
            Adcmplr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmplcha21(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adcmplr1::Cmplcha21,
        adcmplr1::Cmplcha21,
        Adcmplr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adcmplr1::Cmplcha21,
            adcmplr1::Cmplcha21,
            Adcmplr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmplcha22(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adcmplr1::Cmplcha22,
        adcmplr1::Cmplcha22,
        Adcmplr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adcmplr1::Cmplcha22,
            adcmplr1::Cmplcha22,
            Adcmplr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcmplr1 {
    #[inline(always)]
    fn default() -> Adcmplr1 {
        <crate::RegValueT<Adcmplr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmplr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha16_SPEC;
    pub type Cmplcha16 = crate::EnumBitfieldStruct<u8, Cmplcha16_SPEC>;
    impl Cmplcha16 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha17_SPEC;
    pub type Cmplcha17 = crate::EnumBitfieldStruct<u8, Cmplcha17_SPEC>;
    impl Cmplcha17 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha18_SPEC;
    pub type Cmplcha18 = crate::EnumBitfieldStruct<u8, Cmplcha18_SPEC>;
    impl Cmplcha18 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha19_SPEC;
    pub type Cmplcha19 = crate::EnumBitfieldStruct<u8, Cmplcha19_SPEC>;
    impl Cmplcha19 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha20_SPEC;
    pub type Cmplcha20 = crate::EnumBitfieldStruct<u8, Cmplcha20_SPEC>;
    impl Cmplcha20 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha21_SPEC;
    pub type Cmplcha21 = crate::EnumBitfieldStruct<u8, Cmplcha21_SPEC>;
    impl Cmplcha21 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplcha22_SPEC;
    pub type Cmplcha22 = crate::EnumBitfieldStruct<u8, Cmplcha22_SPEC>;
    impl Cmplcha22 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpdr_SPEC;
impl crate::sealed::RegSpec for Adcmpdr_SPEC {
    type DataType = u16;
}

pub type Adcmpdr = crate::RegValueT<Adcmpdr_SPEC>;

impl NoBitfieldReg<Adcmpdr_SPEC> for Adcmpdr {}
impl ::core::default::Default for Adcmpdr {
    #[inline(always)]
    fn default() -> Adcmpdr {
        <crate::RegValueT<Adcmpdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpsr0_SPEC;
impl crate::sealed::RegSpec for Adcmpsr0_SPEC {
    type DataType = u16;
}

pub type Adcmpsr0 = crate::RegValueT<Adcmpsr0_SPEC>;

impl Adcmpsr0 {
    #[inline(always)]
    pub fn cmpstcha00(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adcmpsr0::Cmpstcha00,
        adcmpsr0::Cmpstcha00,
        Adcmpsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adcmpsr0::Cmpstcha00,
            adcmpsr0::Cmpstcha00,
            Adcmpsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpstcha01(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adcmpsr0::Cmpstcha01,
        adcmpsr0::Cmpstcha01,
        Adcmpsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adcmpsr0::Cmpstcha01,
            adcmpsr0::Cmpstcha01,
            Adcmpsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpstcha02(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adcmpsr0::Cmpstcha02,
        adcmpsr0::Cmpstcha02,
        Adcmpsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adcmpsr0::Cmpstcha02,
            adcmpsr0::Cmpstcha02,
            Adcmpsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpstcha03(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adcmpsr0::Cmpstcha03,
        adcmpsr0::Cmpstcha03,
        Adcmpsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adcmpsr0::Cmpstcha03,
            adcmpsr0::Cmpstcha03,
            Adcmpsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpstcha04(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adcmpsr0::Cmpstcha04,
        adcmpsr0::Cmpstcha04,
        Adcmpsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adcmpsr0::Cmpstcha04,
            adcmpsr0::Cmpstcha04,
            Adcmpsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpstcha05(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adcmpsr0::Cmpstcha05,
        adcmpsr0::Cmpstcha05,
        Adcmpsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adcmpsr0::Cmpstcha05,
            adcmpsr0::Cmpstcha05,
            Adcmpsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpstcha06(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adcmpsr0::Cmpstcha06,
        adcmpsr0::Cmpstcha06,
        Adcmpsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adcmpsr0::Cmpstcha06,
            adcmpsr0::Cmpstcha06,
            Adcmpsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpstcha07(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adcmpsr0::Cmpstcha07,
        adcmpsr0::Cmpstcha07,
        Adcmpsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adcmpsr0::Cmpstcha07,
            adcmpsr0::Cmpstcha07,
            Adcmpsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpstcha08(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        adcmpsr0::Cmpstcha08,
        adcmpsr0::Cmpstcha08,
        Adcmpsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            adcmpsr0::Cmpstcha08,
            adcmpsr0::Cmpstcha08,
            Adcmpsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpstcha09(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        adcmpsr0::Cmpstcha09,
        adcmpsr0::Cmpstcha09,
        Adcmpsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            adcmpsr0::Cmpstcha09,
            adcmpsr0::Cmpstcha09,
            Adcmpsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpstcha10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        adcmpsr0::Cmpstcha10,
        adcmpsr0::Cmpstcha10,
        Adcmpsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            adcmpsr0::Cmpstcha10,
            adcmpsr0::Cmpstcha10,
            Adcmpsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcmpsr0 {
    #[inline(always)]
    fn default() -> Adcmpsr0 {
        <crate::RegValueT<Adcmpsr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmpsr0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha00_SPEC;
    pub type Cmpstcha00 = crate::EnumBitfieldStruct<u8, Cmpstcha00_SPEC>;
    impl Cmpstcha00 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha01_SPEC;
    pub type Cmpstcha01 = crate::EnumBitfieldStruct<u8, Cmpstcha01_SPEC>;
    impl Cmpstcha01 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha02_SPEC;
    pub type Cmpstcha02 = crate::EnumBitfieldStruct<u8, Cmpstcha02_SPEC>;
    impl Cmpstcha02 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha03_SPEC;
    pub type Cmpstcha03 = crate::EnumBitfieldStruct<u8, Cmpstcha03_SPEC>;
    impl Cmpstcha03 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha04_SPEC;
    pub type Cmpstcha04 = crate::EnumBitfieldStruct<u8, Cmpstcha04_SPEC>;
    impl Cmpstcha04 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha05_SPEC;
    pub type Cmpstcha05 = crate::EnumBitfieldStruct<u8, Cmpstcha05_SPEC>;
    impl Cmpstcha05 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha06_SPEC;
    pub type Cmpstcha06 = crate::EnumBitfieldStruct<u8, Cmpstcha06_SPEC>;
    impl Cmpstcha06 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha07_SPEC;
    pub type Cmpstcha07 = crate::EnumBitfieldStruct<u8, Cmpstcha07_SPEC>;
    impl Cmpstcha07 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha08_SPEC;
    pub type Cmpstcha08 = crate::EnumBitfieldStruct<u8, Cmpstcha08_SPEC>;
    impl Cmpstcha08 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha09_SPEC;
    pub type Cmpstcha09 = crate::EnumBitfieldStruct<u8, Cmpstcha09_SPEC>;
    impl Cmpstcha09 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha10_SPEC;
    pub type Cmpstcha10 = crate::EnumBitfieldStruct<u8, Cmpstcha10_SPEC>;
    impl Cmpstcha10 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpsr1_SPEC;
impl crate::sealed::RegSpec for Adcmpsr1_SPEC {
    type DataType = u16;
}

pub type Adcmpsr1 = crate::RegValueT<Adcmpsr1_SPEC>;

impl Adcmpsr1 {
    #[inline(always)]
    pub fn cmpstcha16(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adcmpsr1::Cmpstcha16,
        adcmpsr1::Cmpstcha16,
        Adcmpsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adcmpsr1::Cmpstcha16,
            adcmpsr1::Cmpstcha16,
            Adcmpsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpstcha17(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adcmpsr1::Cmpstcha17,
        adcmpsr1::Cmpstcha17,
        Adcmpsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adcmpsr1::Cmpstcha17,
            adcmpsr1::Cmpstcha17,
            Adcmpsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpstcha18(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        adcmpsr1::Cmpstcha18,
        adcmpsr1::Cmpstcha18,
        Adcmpsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            adcmpsr1::Cmpstcha18,
            adcmpsr1::Cmpstcha18,
            Adcmpsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpstcha19(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        adcmpsr1::Cmpstcha19,
        adcmpsr1::Cmpstcha19,
        Adcmpsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            adcmpsr1::Cmpstcha19,
            adcmpsr1::Cmpstcha19,
            Adcmpsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpstcha20(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        adcmpsr1::Cmpstcha20,
        adcmpsr1::Cmpstcha20,
        Adcmpsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            adcmpsr1::Cmpstcha20,
            adcmpsr1::Cmpstcha20,
            Adcmpsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpstcha21(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        adcmpsr1::Cmpstcha21,
        adcmpsr1::Cmpstcha21,
        Adcmpsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            adcmpsr1::Cmpstcha21,
            adcmpsr1::Cmpstcha21,
            Adcmpsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpstcha22(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        adcmpsr1::Cmpstcha22,
        adcmpsr1::Cmpstcha22,
        Adcmpsr1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            adcmpsr1::Cmpstcha22,
            adcmpsr1::Cmpstcha22,
            Adcmpsr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcmpsr1 {
    #[inline(always)]
    fn default() -> Adcmpsr1 {
        <crate::RegValueT<Adcmpsr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmpsr1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha16_SPEC;
    pub type Cmpstcha16 = crate::EnumBitfieldStruct<u8, Cmpstcha16_SPEC>;
    impl Cmpstcha16 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha17_SPEC;
    pub type Cmpstcha17 = crate::EnumBitfieldStruct<u8, Cmpstcha17_SPEC>;
    impl Cmpstcha17 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha18_SPEC;
    pub type Cmpstcha18 = crate::EnumBitfieldStruct<u8, Cmpstcha18_SPEC>;
    impl Cmpstcha18 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha19_SPEC;
    pub type Cmpstcha19 = crate::EnumBitfieldStruct<u8, Cmpstcha19_SPEC>;
    impl Cmpstcha19 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha20_SPEC;
    pub type Cmpstcha20 = crate::EnumBitfieldStruct<u8, Cmpstcha20_SPEC>;
    impl Cmpstcha20 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha21_SPEC;
    pub type Cmpstcha21 = crate::EnumBitfieldStruct<u8, Cmpstcha21_SPEC>;
    impl Cmpstcha21 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstcha22_SPEC;
    pub type Cmpstcha22 = crate::EnumBitfieldStruct<u8, Cmpstcha22_SPEC>;
    impl Cmpstcha22 {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpser_SPEC;
impl crate::sealed::RegSpec for Adcmpser_SPEC {
    type DataType = u8;
}

pub type Adcmpser = crate::RegValueT<Adcmpser_SPEC>;

impl Adcmpser {
    #[inline(always)]
    pub fn cmpsttsa(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adcmpser::Cmpsttsa,
        adcmpser::Cmpsttsa,
        Adcmpser_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adcmpser::Cmpsttsa,
            adcmpser::Cmpsttsa,
            Adcmpser_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmpstoca(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        adcmpser::Cmpstoca,
        adcmpser::Cmpstoca,
        Adcmpser_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            adcmpser::Cmpstoca,
            adcmpser::Cmpstoca,
            Adcmpser_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcmpser {
    #[inline(always)]
    fn default() -> Adcmpser {
        <crate::RegValueT<Adcmpser_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmpser {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpsttsa_SPEC;
    pub type Cmpsttsa = crate::EnumBitfieldStruct<u8, Cmpsttsa_SPEC>;
    impl Cmpsttsa {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstoca_SPEC;
    pub type Cmpstoca = crate::EnumBitfieldStruct<u8, Cmpstoca_SPEC>;
    impl Cmpstoca {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpbnsr_SPEC;
impl crate::sealed::RegSpec for Adcmpbnsr_SPEC {
    type DataType = u8;
}

pub type Adcmpbnsr = crate::RegValueT<Adcmpbnsr_SPEC>;

impl Adcmpbnsr {
    #[inline(always)]
    pub fn cmpchb(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, Adcmpbnsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,Adcmpbnsr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmplb(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        adcmpbnsr::Cmplb,
        adcmpbnsr::Cmplb,
        Adcmpbnsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            adcmpbnsr::Cmplb,
            adcmpbnsr::Cmplb,
            Adcmpbnsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcmpbnsr {
    #[inline(always)]
    fn default() -> Adcmpbnsr {
        <crate::RegValueT<Adcmpbnsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmpbnsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmplb_SPEC;
    pub type Cmplb = crate::EnumBitfieldStruct<u8, Cmplb_SPEC>;
    impl Cmplb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adwinllb_SPEC;
impl crate::sealed::RegSpec for Adwinllb_SPEC {
    type DataType = u16;
}

pub type Adwinllb = crate::RegValueT<Adwinllb_SPEC>;

impl NoBitfieldReg<Adwinllb_SPEC> for Adwinllb {}
impl ::core::default::Default for Adwinllb {
    #[inline(always)]
    fn default() -> Adwinllb {
        <crate::RegValueT<Adwinllb_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adwinulb_SPEC;
impl crate::sealed::RegSpec for Adwinulb_SPEC {
    type DataType = u16;
}

pub type Adwinulb = crate::RegValueT<Adwinulb_SPEC>;

impl NoBitfieldReg<Adwinulb_SPEC> for Adwinulb {}
impl ::core::default::Default for Adwinulb {
    #[inline(always)]
    fn default() -> Adwinulb {
        <crate::RegValueT<Adwinulb_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adcmpbsr_SPEC;
impl crate::sealed::RegSpec for Adcmpbsr_SPEC {
    type DataType = u8;
}

pub type Adcmpbsr = crate::RegValueT<Adcmpbsr_SPEC>;

impl Adcmpbsr {
    #[inline(always)]
    pub fn cmpstb(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        adcmpbsr::Cmpstb,
        adcmpbsr::Cmpstb,
        Adcmpbsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            adcmpbsr::Cmpstb,
            adcmpbsr::Cmpstb,
            Adcmpbsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Adcmpbsr {
    #[inline(always)]
    fn default() -> Adcmpbsr {
        <crate::RegValueT<Adcmpbsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod adcmpbsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmpstb_SPEC;
    pub type Cmpstb = crate::EnumBitfieldStruct<u8, Cmpstb_SPEC>;
    impl Cmpstb {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adsstrl_SPEC;
impl crate::sealed::RegSpec for Adsstrl_SPEC {
    type DataType = u8;
}

pub type Adsstrl = crate::RegValueT<Adsstrl_SPEC>;

impl Adsstrl {
    #[inline(always)]
    pub fn sst(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Adsstrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Adsstrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adsstrl {
    #[inline(always)]
    fn default() -> Adsstrl {
        <crate::RegValueT<Adsstrl_SPEC> as RegisterValue<_>>::new(13)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adsstrt_SPEC;
impl crate::sealed::RegSpec for Adsstrt_SPEC {
    type DataType = u8;
}

pub type Adsstrt = crate::RegValueT<Adsstrt_SPEC>;

impl Adsstrt {
    #[inline(always)]
    pub fn sst(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Adsstrt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Adsstrt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adsstrt {
    #[inline(always)]
    fn default() -> Adsstrt {
        <crate::RegValueT<Adsstrt_SPEC> as RegisterValue<_>>::new(13)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adsstro_SPEC;
impl crate::sealed::RegSpec for Adsstro_SPEC {
    type DataType = u8;
}

pub type Adsstro = crate::RegValueT<Adsstro_SPEC>;

impl Adsstro {
    #[inline(always)]
    pub fn sst(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Adsstro_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Adsstro_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adsstro {
    #[inline(always)]
    fn default() -> Adsstro {
        <crate::RegValueT<Adsstro_SPEC> as RegisterValue<_>>::new(13)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adsstr_SPEC;
impl crate::sealed::RegSpec for Adsstr_SPEC {
    type DataType = u8;
}

pub type Adsstr = crate::RegValueT<Adsstr_SPEC>;

impl Adsstr {
    #[inline(always)]
    pub fn sst(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Adsstr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Adsstr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adsstr {
    #[inline(always)]
    fn default() -> Adsstr {
        <crate::RegValueT<Adsstr_SPEC> as RegisterValue<_>>::new(13)
    }
}
