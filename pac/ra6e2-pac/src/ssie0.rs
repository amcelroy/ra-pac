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
// Generated from SVD 1.30.00, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:19:41 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Serial Sound Interface Enhanced (SSIE)"]
unsafe impl ::core::marker::Send for super::Ssie0 {}
unsafe impl ::core::marker::Sync for super::Ssie0 {}
impl super::Ssie0 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn ssicr(&self) -> &'static crate::common::Reg<self::Ssicr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ssicr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ssisr(&self) -> &'static crate::common::Reg<self::Ssisr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ssisr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ssifcr(
        &self,
    ) -> &'static crate::common::Reg<self::Ssifcr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ssifcr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ssifsr(
        &self,
    ) -> &'static crate::common::Reg<self::Ssifsr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ssifsr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ssiftdr(
        &self,
    ) -> &'static crate::common::Reg<self::Ssiftdr_SPEC, crate::common::W> {
        unsafe {
            crate::common::Reg::<self::Ssiftdr_SPEC, crate::common::W>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ssifrdr(
        &self,
    ) -> &'static crate::common::Reg<self::Ssifrdr_SPEC, crate::common::R> {
        unsafe {
            crate::common::Reg::<self::Ssifrdr_SPEC, crate::common::R>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ssiofr(
        &self,
    ) -> &'static crate::common::Reg<self::Ssiofr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ssiofr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ssiscr(
        &self,
    ) -> &'static crate::common::Reg<self::Ssiscr_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Ssiscr_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssicr_SPEC;
impl crate::sealed::RegSpec for Ssicr_SPEC {
    type DataType = u32;
}

pub type Ssicr = crate::RegValueT<Ssicr_SPEC>;

impl Ssicr {
    #[inline(always)]
    pub fn ren(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ssicr::Ren,
        ssicr::Ren,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ssicr::Ren,
            ssicr::Ren,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ten(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ssicr::Ten,
        ssicr::Ten,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ssicr::Ten,
            ssicr::Ten,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn muen(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ssicr::Muen,
        ssicr::Muen,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ssicr::Muen,
            ssicr::Muen,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ckdv(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xf,
        1,
        0,
        ssicr::Ckdv,
        ssicr::Ckdv,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0xf,
            1,
            0,
            ssicr::Ckdv,
            ssicr::Ckdv,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn del(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        ssicr::Del,
        ssicr::Del,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            ssicr::Del,
            ssicr::Del,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pdta(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ssicr::Pdta,
        ssicr::Pdta,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ssicr::Pdta,
            ssicr::Pdta,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sdta(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        ssicr::Sdta,
        ssicr::Sdta,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            ssicr::Sdta,
            ssicr::Sdta,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn spdp(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        ssicr::Spdp,
        ssicr::Spdp,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            ssicr::Spdp,
            ssicr::Spdp,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lrckp(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        ssicr::Lrckp,
        ssicr::Lrckp,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            ssicr::Lrckp,
            ssicr::Lrckp,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bckp(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        ssicr::Bckp,
        ssicr::Bckp,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            ssicr::Bckp,
            ssicr::Bckp,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn mst(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        ssicr::Mst,
        ssicr::Mst,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            ssicr::Mst,
            ssicr::Mst,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn swl(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        ssicr::Swl,
        ssicr::Swl,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            ssicr::Swl,
            ssicr::Swl,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dwl(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x7,
        1,
        0,
        ssicr::Dwl,
        ssicr::Dwl,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x7,
            1,
            0,
            ssicr::Dwl,
            ssicr::Dwl,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn frm(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, u8, Ssicr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0x3,1,0,u8,u8,Ssicr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn iien(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        ssicr::Iien,
        ssicr::Iien,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            ssicr::Iien,
            ssicr::Iien,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn roien(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        ssicr::Roien,
        ssicr::Roien,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            ssicr::Roien,
            ssicr::Roien,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ruien(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        ssicr::Ruien,
        ssicr::Ruien,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            ssicr::Ruien,
            ssicr::Ruien,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn toien(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        ssicr::Toien,
        ssicr::Toien,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            ssicr::Toien,
            ssicr::Toien,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tuien(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        ssicr::Tuien,
        ssicr::Tuien,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            ssicr::Tuien,
            ssicr::Tuien,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cks(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        ssicr::Cks,
        ssicr::Cks,
        Ssicr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            ssicr::Cks,
            ssicr::Cks,
            Ssicr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ssicr {
    #[inline(always)]
    fn default() -> Ssicr {
        <crate::RegValueT<Ssicr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ssicr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ren_SPEC;
    pub type Ren = crate::EnumBitfieldStruct<u8, Ren_SPEC>;
    impl Ren {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ten_SPEC;
    pub type Ten = crate::EnumBitfieldStruct<u8, Ten_SPEC>;
    impl Ten {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Muen_SPEC;
    pub type Muen = crate::EnumBitfieldStruct<u8, Muen_SPEC>;
    impl Muen {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ckdv_SPEC;
    pub type Ckdv = crate::EnumBitfieldStruct<u8, Ckdv_SPEC>;
    impl Ckdv {
        pub const _0_X_0: Self = Self::new(0);

        pub const _0_X_1: Self = Self::new(1);

        pub const _0_X_2: Self = Self::new(2);

        pub const _0_X_3: Self = Self::new(3);

        pub const _0_X_4: Self = Self::new(4);

        pub const _0_X_5: Self = Self::new(5);

        pub const _0_X_6: Self = Self::new(6);

        pub const _0_X_7: Self = Self::new(7);

        pub const _0_X_8: Self = Self::new(8);

        pub const _0_X_9: Self = Self::new(9);

        pub const _0_X_A: Self = Self::new(10);

        pub const _0_X_B: Self = Self::new(11);

        pub const _0_X_C: Self = Self::new(12);

        pub const OTHERS: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Del_SPEC;
    pub type Del = crate::EnumBitfieldStruct<u8, Del_SPEC>;
    impl Del {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pdta_SPEC;
    pub type Pdta = crate::EnumBitfieldStruct<u8, Pdta_SPEC>;
    impl Pdta {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sdta_SPEC;
    pub type Sdta = crate::EnumBitfieldStruct<u8, Sdta_SPEC>;
    impl Sdta {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Spdp_SPEC;
    pub type Spdp = crate::EnumBitfieldStruct<u8, Spdp_SPEC>;
    impl Spdp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lrckp_SPEC;
    pub type Lrckp = crate::EnumBitfieldStruct<u8, Lrckp_SPEC>;
    impl Lrckp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bckp_SPEC;
    pub type Bckp = crate::EnumBitfieldStruct<u8, Bckp_SPEC>;
    impl Bckp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mst_SPEC;
    pub type Mst = crate::EnumBitfieldStruct<u8, Mst_SPEC>;
    impl Mst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Swl_SPEC;
    pub type Swl = crate::EnumBitfieldStruct<u8, Swl_SPEC>;
    impl Swl {
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
    pub struct Dwl_SPEC;
    pub type Dwl = crate::EnumBitfieldStruct<u8, Dwl_SPEC>;
    impl Dwl {
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
    pub struct Iien_SPEC;
    pub type Iien = crate::EnumBitfieldStruct<u8, Iien_SPEC>;
    impl Iien {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Roien_SPEC;
    pub type Roien = crate::EnumBitfieldStruct<u8, Roien_SPEC>;
    impl Roien {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ruien_SPEC;
    pub type Ruien = crate::EnumBitfieldStruct<u8, Ruien_SPEC>;
    impl Ruien {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Toien_SPEC;
    pub type Toien = crate::EnumBitfieldStruct<u8, Toien_SPEC>;
    impl Toien {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tuien_SPEC;
    pub type Tuien = crate::EnumBitfieldStruct<u8, Tuien_SPEC>;
    impl Tuien {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cks_SPEC;
    pub type Cks = crate::EnumBitfieldStruct<u8, Cks_SPEC>;
    impl Cks {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssisr_SPEC;
impl crate::sealed::RegSpec for Ssisr_SPEC {
    type DataType = u32;
}

pub type Ssisr = crate::RegValueT<Ssisr_SPEC>;

impl Ssisr {
    #[inline(always)]
    pub fn iirq(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        ssisr::Iirq,
        ssisr::Iirq,
        Ssisr_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            ssisr::Iirq,
            ssisr::Iirq,
            Ssisr_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn roirq(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        ssisr::Roirq,
        ssisr::Roirq,
        Ssisr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            ssisr::Roirq,
            ssisr::Roirq,
            Ssisr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ruirq(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        ssisr::Ruirq,
        ssisr::Ruirq,
        Ssisr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            ssisr::Ruirq,
            ssisr::Ruirq,
            Ssisr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn toirq(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        ssisr::Toirq,
        ssisr::Toirq,
        Ssisr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            ssisr::Toirq,
            ssisr::Toirq,
            Ssisr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tuirq(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        ssisr::Tuirq,
        ssisr::Tuirq,
        Ssisr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            ssisr::Tuirq,
            ssisr::Tuirq,
            Ssisr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ssisr {
    #[inline(always)]
    fn default() -> Ssisr {
        <crate::RegValueT<Ssisr_SPEC> as RegisterValue<_>>::new(33554432)
    }
}
pub mod ssisr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Iirq_SPEC;
    pub type Iirq = crate::EnumBitfieldStruct<u8, Iirq_SPEC>;
    impl Iirq {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Roirq_SPEC;
    pub type Roirq = crate::EnumBitfieldStruct<u8, Roirq_SPEC>;
    impl Roirq {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ruirq_SPEC;
    pub type Ruirq = crate::EnumBitfieldStruct<u8, Ruirq_SPEC>;
    impl Ruirq {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Toirq_SPEC;
    pub type Toirq = crate::EnumBitfieldStruct<u8, Toirq_SPEC>;
    impl Toirq {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tuirq_SPEC;
    pub type Tuirq = crate::EnumBitfieldStruct<u8, Tuirq_SPEC>;
    impl Tuirq {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssifcr_SPEC;
impl crate::sealed::RegSpec for Ssifcr_SPEC {
    type DataType = u32;
}

pub type Ssifcr = crate::RegValueT<Ssifcr_SPEC>;

impl Ssifcr {
    #[inline(always)]
    pub fn rfrst(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ssifcr::Rfrst,
        ssifcr::Rfrst,
        Ssifcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ssifcr::Rfrst,
            ssifcr::Rfrst,
            Ssifcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tfrst(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ssifcr::Tfrst,
        ssifcr::Tfrst,
        Ssifcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ssifcr::Tfrst,
            ssifcr::Tfrst,
            Ssifcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rie(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ssifcr::Rie,
        ssifcr::Rie,
        Ssifcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ssifcr::Rie,
            ssifcr::Rie,
            Ssifcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tie(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ssifcr::Tie,
        ssifcr::Tie,
        Ssifcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ssifcr::Tie,
            ssifcr::Tie,
            Ssifcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bsw(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        ssifcr::Bsw,
        ssifcr::Bsw,
        Ssifcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            ssifcr::Bsw,
            ssifcr::Bsw,
            Ssifcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ssirst(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ssifcr::Ssirst,
        ssifcr::Ssirst,
        Ssifcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ssifcr::Ssirst,
            ssifcr::Ssirst,
            Ssifcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn aucke(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        ssifcr::Aucke,
        ssifcr::Aucke,
        Ssifcr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            ssifcr::Aucke,
            ssifcr::Aucke,
            Ssifcr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ssifcr {
    #[inline(always)]
    fn default() -> Ssifcr {
        <crate::RegValueT<Ssifcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ssifcr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rfrst_SPEC;
    pub type Rfrst = crate::EnumBitfieldStruct<u8, Rfrst_SPEC>;
    impl Rfrst {
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
    pub struct Bsw_SPEC;
    pub type Bsw = crate::EnumBitfieldStruct<u8, Bsw_SPEC>;
    impl Bsw {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ssirst_SPEC;
    pub type Ssirst = crate::EnumBitfieldStruct<u8, Ssirst_SPEC>;
    impl Ssirst {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Aucke_SPEC;
    pub type Aucke = crate::EnumBitfieldStruct<u8, Aucke_SPEC>;
    impl Aucke {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssifsr_SPEC;
impl crate::sealed::RegSpec for Ssifsr_SPEC {
    type DataType = u32;
}

pub type Ssifsr = crate::RegValueT<Ssifsr_SPEC>;

impl Ssifsr {
    #[inline(always)]
    pub fn rdf(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ssifsr::Rdf,
        ssifsr::Rdf,
        Ssifsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ssifsr::Rdf,
            ssifsr::Rdf,
            Ssifsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rdc(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, u8, Ssifsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x3f,1,0,u8,u8,Ssifsr_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tde(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        ssifsr::Tde,
        ssifsr::Tde,
        Ssifsr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            ssifsr::Tde,
            ssifsr::Tde,
            Ssifsr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tdc(
        self,
    ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, u8, Ssifsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x3f,1,0,u8,u8,Ssifsr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Ssifsr {
    #[inline(always)]
    fn default() -> Ssifsr {
        <crate::RegValueT<Ssifsr_SPEC> as RegisterValue<_>>::new(65536)
    }
}
pub mod ssifsr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdf_SPEC;
    pub type Rdf = crate::EnumBitfieldStruct<u8, Rdf_SPEC>;
    impl Rdf {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tde_SPEC;
    pub type Tde = crate::EnumBitfieldStruct<u8, Tde_SPEC>;
    impl Tde {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssiftdr_SPEC;
impl crate::sealed::RegSpec for Ssiftdr_SPEC {
    type DataType = u32;
}

pub type Ssiftdr = crate::RegValueT<Ssiftdr_SPEC>;

impl Ssiftdr {
    #[inline(always)]
    pub fn ssiftdr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Ssiftdr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Ssiftdr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Ssiftdr {
    #[inline(always)]
    fn default() -> Ssiftdr {
        <crate::RegValueT<Ssiftdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssifrdr_SPEC;
impl crate::sealed::RegSpec for Ssifrdr_SPEC {
    type DataType = u32;
}

pub type Ssifrdr = crate::RegValueT<Ssifrdr_SPEC>;

impl Ssifrdr {
    #[inline(always)]
    pub fn ssifrdr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, u32, Ssifrdr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32,u32,Ssifrdr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Ssifrdr {
    #[inline(always)]
    fn default() -> Ssifrdr {
        <crate::RegValueT<Ssifrdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssiofr_SPEC;
impl crate::sealed::RegSpec for Ssiofr_SPEC {
    type DataType = u32;
}

pub type Ssiofr = crate::RegValueT<Ssiofr_SPEC>;

impl Ssiofr {
    #[inline(always)]
    pub fn omod(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        ssiofr::Omod,
        ssiofr::Omod,
        Ssiofr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            ssiofr::Omod,
            ssiofr::Omod,
            Ssiofr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lrcont(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        ssiofr::Lrcont,
        ssiofr::Lrcont,
        Ssiofr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            ssiofr::Lrcont,
            ssiofr::Lrcont,
            Ssiofr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bckastp(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ssiofr::Bckastp,
        ssiofr::Bckastp,
        Ssiofr_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ssiofr::Bckastp,
            ssiofr::Bckastp,
            Ssiofr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Ssiofr {
    #[inline(always)]
    fn default() -> Ssiofr {
        <crate::RegValueT<Ssiofr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ssiofr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Omod_SPEC;
    pub type Omod = crate::EnumBitfieldStruct<u8, Omod_SPEC>;
    impl Omod {
        pub const _00: Self = Self::new(0);

        pub const _01: Self = Self::new(1);

        pub const _10: Self = Self::new(2);

        pub const _11: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lrcont_SPEC;
    pub type Lrcont = crate::EnumBitfieldStruct<u8, Lrcont_SPEC>;
    impl Lrcont {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bckastp_SPEC;
    pub type Bckastp = crate::EnumBitfieldStruct<u8, Bckastp_SPEC>;
    impl Bckastp {
        pub const _0: Self = Self::new(0);

        pub const _1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssiscr_SPEC;
impl crate::sealed::RegSpec for Ssiscr_SPEC {
    type DataType = u32;
}

pub type Ssiscr = crate::RegValueT<Ssiscr_SPEC>;

impl Ssiscr {
    #[inline(always)]
    pub fn rdfs(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, Ssiscr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,Ssiscr_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tdes(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, Ssiscr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,Ssiscr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ssiscr {
    #[inline(always)]
    fn default() -> Ssiscr {
        <crate::RegValueT<Ssiscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
