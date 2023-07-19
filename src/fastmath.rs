//! A collection of fast (often approximate) mathematical functions for accelerating mathematical functions

// Optimisation note: lookup tables become faster when calculation takes > ~1ms

use std::f32::consts as f32_consts;
use std::f64::consts as f64_consts;
use crate::lookup::*;

pub trait FastMath: FastCos + FastPow2 + FastExp + FastSigmoid {}
impl FastMath for f32 {}
impl FastMath for f64 {}


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
        const BITAND: u32 = u32::MAX / 2;
        const ONE: f32 = 1.0;
        let mod_x = (((self + f32_consts::PI).abs()) % f32_consts::TAU) - f32_consts::PI;
        let v = mod_x.to_bits() & BITAND;
        let qpprox = ONE - f32_consts::FRAC_2_PI * f32::from_bits(v);
        qpprox + f32_consts::FRAC_PI_6 * qpprox * (ONE - qpprox * qpprox)
    }
}
impl FastCos for f64 {
    #[inline]
    fn fast_cos(self: Self) -> f64 {
        const BITAND: u64 = u64::MAX / 2;
        const ONE: f64 = 1.0;
        let mod_x = (((self + f64_consts::PI).abs()) % f64_consts::TAU) - f64_consts::PI;
        let v = mod_x.to_bits() & BITAND;
        let qpprox = ONE - f64_consts::FRAC_2_PI * f64::from_bits(v);
        qpprox + f64_consts::FRAC_PI_6 * qpprox * (ONE - qpprox * qpprox)
    }
}

pub trait FastPow2 {
    fn fast_pow2(self: Self) -> Self;
}
impl FastPow2 for f32 {
    #[inline]
    fn fast_pow2(self: Self) -> f32 {
        // Khinchins constant over 3. IDK why it gives the best fit, but it does
        const KHINCHIN_3: f32 = 2.68545200106530644530971483548179569382038229399446295305115234555721885953715200280114117493184769799515 / 3.0;
        const CLIPP_THRESH: f32 = 0.12847338;
        const V_SCALE: f32 = 8388608.0; // (1_i32 << 23) as f32
        const CLIPP_SHIFT: f32 = 126.67740855;
        let abs_p = self.abs();
        let clipp = abs_p.max(CLIPP_THRESH); // if abs_p < CLIPP_THRESH { CLIPP_THRESH } else { abs_p };
        let v = (V_SCALE * (clipp + CLIPP_SHIFT)) as u32;
        f32::from_bits(v) - KHINCHIN_3
    }
}
impl FastPow2 for f64 {
    #[inline]
    fn fast_pow2(self: Self) -> f64 {
        const KHINCHIN_3: f64 = 2.68545200106530644530971483548179569382038229399446295305115234555721885953715200280114117493184769799515 / 3.0;
        const CLIPP_THRESH: f64 = -45774.9247660416;
        const V_SCALE: f64 = 4503599627370496.0; // (1i64 << 52) as f64
        const CLIPP_SHIFT: f64 = 1022.6769200000002;
        const ZERO: f64 = 0.;
        let abs_p = self.abs();
        let clipp = abs_p.max(CLIPP_THRESH); // if abs_p < CLIPP_THRESH { CLIPP_THRESH } else { abs_p };
        let v = (V_SCALE * (clipp + CLIPP_SHIFT)) as u64;
        let y = f64::from_bits(v) - KHINCHIN_3;
        if y.is_sign_positive() {
            y
        } else {
            ZERO
        }
    }
}

pub trait FastExp {
    fn fast_exp(self: Self) -> Self;
}
impl FastExp for f32 {
    #[inline]
    fn fast_exp(self: Self) -> f32 {
        const CLIPP_THRESH: f32 = -126.0; // 0.12847338;
        const V_SCALE: f32 = 8388608.0; // (1_i32 << 23) as f32
        const CLIPP_SHIFT: f32 = 126.94269504; // 126.67740855;

        let scaled_p = f32_consts::LOG2_E * self;
        let clipp = scaled_p.max(CLIPP_THRESH); // if scaled_p < CLIPP_THRESH { CLIPP_THRESH } else { scaled_p };
        let v = (V_SCALE * (clipp + CLIPP_SHIFT)) as u32;
        f32::from_bits(v)
    }
}
impl FastExp for f64 {
    #[inline]
    fn fast_exp(self: Self) -> f64 {
        const CLIPP_THRESH: f64 = -180335.51911105003;
        const V_SCALE: f64 = 4524653012949098.0;
        const CLIPP_SHIFT: f64 = 1018.1563534409383;

        let scaled_p = f64_consts::LOG2_E * self;
        let clipp = scaled_p.max(CLIPP_THRESH); // let clipp = if scaled_p < CLIPP_THRESH { CLIPP_THRESH } else { scaled_p };
        let v = (V_SCALE * (clipp + CLIPP_SHIFT)) as u64;
        f64::from_bits(v)
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

// A trait for testing and improving implementations of fast functions
pub trait Test {
    fn test(self: Self) -> Self;
}
impl Test for f32 {
    #[inline]
    fn test(self: Self) -> f32 {
        // Khinchins constant over 3. IDK why it gives the best fit, but it does
        // const KHINCHIN_3: f32 = 2.68545200106530644530971483548179569382038229399446295305115234555721885953715200280114117493184769799515 / 3.0;
        const CLIPP_THRESH: f32 = -126.0; // 0.12847338;
        const V_SCALE: f32 = 8388608.0; // (1_i32 << 23) as f32
        const CLIPP_SHIFT: f32 = 126.94269504; // 126.67740855;

        let scaled_p = f32_consts::LOG2_E * self;
        let clipp = if scaled_p < CLIPP_THRESH {
            CLIPP_THRESH
        } else {
            scaled_p
        };
        let v = (V_SCALE * (clipp + CLIPP_SHIFT)) as u32;
        f32::from_bits(v) // - KHINCHIN_3
    }
}
impl Test for f64 {
    #[inline]
    fn test(self: Self) -> f64 {
        const CLIPP_THRESH: f64 = -180335.51911105003;
        const V_SCALE: f64 = 4524653012949098.0;
        const CLIPP_SHIFT: f64 = 1018.1563534409383;

        let scaled_p = f64_consts::LOG2_E * self;
        let clipp = if scaled_p < CLIPP_THRESH {
            CLIPP_THRESH
        } else {
            scaled_p
        };
        let v = (V_SCALE * (clipp + CLIPP_SHIFT)) as u64;
        f64::from_bits(v)
    }
}

#[allow(non_snake_case, dead_code)]
pub fn optimizing(p: f64, CLIPP_THRESH: f64, V_SCALE: f64, CLIPP_SHIFT: f64) -> f64 {
    // const CLIPP_THRESH: f64 = -45774.9247660416;
    // const V_SCALE: f64 = 4503599627370496.0;
    // const CLIPP_SHIFT: f64 = 1022.6769200000002;

    let scaled_p = f64_consts::LOG2_E * p;
    let clipp = if scaled_p < CLIPP_THRESH {
        CLIPP_THRESH
    } else {
        scaled_p
    };
    let v = (V_SCALE * (clipp + CLIPP_SHIFT)) as u64;
    f64::from_bits(v)
}

#[inline]
pub fn sigmoid_builtin_f32(p: f32) -> f32 {
    (1. + (-p).exp()).recip()
}

#[inline]
pub fn sigmoid_builtin_f64(p: f64) -> f64 {
    (1. + (-p).exp()).recip()
}
