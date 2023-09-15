pub mod f64 {
    pub fn pow2(n: f64) -> f64 {
        2.0f64.powf(n)
    }

    pub fn exp(n: f64) -> f64 {
        n.exp()
    }

    pub fn cos(n: f64) -> f64 {
        n.cos()
    }

    pub fn sigmoid(n: f64) -> f64 {
        (1. + (-n).exp()).recip()
    }
}

pub mod f32 {
    pub fn pow2(n: f32) -> f32 {
        2.0f32.powf(n)
    }

    pub fn exp(n: f32) -> f32 {
        n.exp()
    }

    pub fn cos(n: f32) -> f32 {
        n.cos()
    }

    pub fn sigmoid(n: f32) -> f32 {
        (1. + (-n).exp()).recip()
    }
}