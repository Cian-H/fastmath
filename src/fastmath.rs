//! A collection of fast (often approximate) mathematical functions for accelerating mathematical functions

// Optimisation note: lookup tables become faster when calculation takes > ~400us

use std::f32::consts as f32_consts;
use std::f64::consts as f64_consts;
use crate::lookup::{EndoCosLookupTable, EndoSinLookupTable};

const SIN_LOOKUP_F32: EndoSinLookupTable<f32> = EndoSinLookupTable::<f32>::new();
const SIN_LOOKUP_F64: EndoSinLookupTable<f64> = EndoSinLookupTable::<f64>::new();
const COS_LOOKUP_F32: EndoCosLookupTable<f32> = EndoCosLookupTable::<f32>::new();
const COS_LOOKUP_F64: EndoCosLookupTable<f64> = EndoCosLookupTable::<f64>::new();

pub trait FastMath: FastCos + FastExp + FastSigmoid {}
impl FastMath for f32 {}
impl FastMath for f64 {}

const V_SCALE_F32: f32 = 8388608.0; // the largest possible mantissa of an f32
const V_SCALE_F64: f64 = 4503599627370496.0; // the largest possible mantissa of an f64


pub trait LookupSin {
    fn lookup_sin(self: Self) -> Self;
}
impl LookupSin for f64 {
    #[inline]
    fn lookup_sin(self: Self) -> f64 {
        // Look up the value in the table
        SIN_LOOKUP_F64.lookup(self)
    }
}
impl LookupSin for f32 {
    #[inline]
    fn lookup_sin(self: Self) -> f32 {
        // Look up the value in the table
        SIN_LOOKUP_F32.lookup(self)
    }
}


pub trait LookupCos {
    fn lookup_cos(self: Self) -> Self;
}
impl LookupCos for f64 {
    #[inline]
    fn lookup_cos(self: Self) -> f64 {
        // Look up the value in the table
        COS_LOOKUP_F64.lookup(self)
    }
}
impl LookupCos for f32 {
    #[inline]
    fn lookup_cos(self: Self) -> f32 {
        // Look up the value in the table
        COS_LOOKUP_F32.lookup(self)
    }
}

pub trait FastCos {
    fn fast_cos(self: Self) -> Self;
}
impl FastCos for f32 {
    #[inline]
    fn fast_cos(self: Self) -> f32 {
        const ONE: f32 = 1.0;
        let v = ((((self + f32_consts::PI).abs()) % f32_consts::TAU) - f32_consts::PI).abs();
        let qpprox = ONE - f32_consts::FRAC_2_PI * v;
        qpprox + f32_consts::FRAC_PI_6 * qpprox * (ONE - qpprox * qpprox)
    }
}
impl FastCos for f64 {
    #[inline]
    fn fast_cos(self: Self) -> f64 {
        const ONE: f64 = 1.0;
        let v = ((((self + f64_consts::PI).abs()) % f64_consts::TAU) - f64_consts::PI).abs();
        let qpprox = ONE - f64_consts::FRAC_2_PI * v;
        qpprox + f64_consts::FRAC_PI_6 * qpprox * (ONE - qpprox * qpprox)
    }
}

pub trait FastExp {
    fn fast_exp(self: Self) -> Self;
}
impl FastExp for f32 {
    #[inline]
    fn fast_exp(self: Self) -> f32 {
        const CLIPP_THRESH: f32 = -126.0; // exponent of smallest possible f32 to prevent underflow
        const CLIPP_SHIFT: f32 = 126.94269504; // shift to align curve, found by regression

        let scaled_p = f32_consts::LOG2_E * self;
        let clipp = scaled_p.max(CLIPP_THRESH);
        let v = (V_SCALE_F32 * (clipp + CLIPP_SHIFT)) as u32;
        f32::from_bits(v)
    }
}
impl FastExp for f64 {
    #[inline]
    fn fast_exp(self: Self) -> f64 {
        const CLIPP_THRESH: f64 = -1022.0; // exponent of smallest possible f64 to prevent underflow
        const CLIPP_SHIFT: f64 = 1022.9349439517318; // shift to align curve, found by regression

        let scaled_p = f64_consts::LOG2_E * self;
        let clipp = scaled_p.max(CLIPP_THRESH);
        let v = (V_SCALE_F64 * (clipp + CLIPP_SHIFT)) as u64;
        f64::from_bits(v)
    }
}

pub trait FastPow2 {
    fn fast_pow2(self: Self) -> Self;
}
impl FastPow2 for f32 {
    #[inline]
    fn fast_pow2(self: Self) -> f32 {
        (f32_consts::LN_2 * self).fast_exp()
    }
}
impl FastPow2 for f64 {
    #[inline]
    fn fast_pow2(self: Self) -> f64 {
        (f64_consts::LN_2 * self).fast_exp()
    }
}

pub trait FastSigmoid {
    fn fast_sigmoid(self: Self) -> Self;
}
impl FastSigmoid for f32 {
    #[inline]
    fn fast_sigmoid(self: Self) -> f32 {
        const ONE: f32 = 1.0;
        (ONE + (-self).fast_exp()).recip()
    }
}
impl FastSigmoid for f64 {
    #[inline]
    fn fast_sigmoid(self: Self) -> f64 {
        const ONE: f64 = 1.0;
        (ONE + (-self).fast_exp()).recip()
    }
}