fn bool_to_full_byte<T>(b: bool) -> T
where
    T: WrappingSub + One + From<u8> + std::ops::Not<Output=T>,
    
{
    !(
        ( T::from( unsafe { std::mem::transmute::<bool, u8>(b) } ) )
        .wrapping_sub(&T::one())
    )
}

pub trait AndBool {
    fn and(self: Self, b: bool) -> Self;
}
impl AndBool for f32 {
    fn and(self: f32, b: bool) -> f32 {
        let b_byte: u32 = bool_to_full_byte(b);
        unsafe {
            std::mem::transmute(
                std::mem::transmute::<f32, u32>(self) & b_byte
            )
        }
    }
}
impl AndBool for f64 {
    fn and(self: f64, b: bool) -> f64 {
        let b_byte: u64 = bool_to_full_byte(b);
        unsafe {
            std::mem::transmute(
                std::mem::transmute::<f64, u64>(self) & b_byte
            )
        }
    }
}


pub trait GetSign {
    fn sign(self: Self) -> Self;
}
impl GetSign for f32 {
    fn sign(self: f32) -> f32 {
        let x_bytes: [u8; 4] = unsafe { std::mem::transmute(self) };
        unsafe {
            std::mem::transmute(
                [
                    0u8,
                    0u8,
                    128u8,
                    (x_bytes[3] & 128u8) | 63u8,
                ]
            )
        }
    }
}
impl GetSign for f64 {
    fn sign(self: f64) -> f64 {
        let x_bytes: [u8; 8] = unsafe { std::mem::transmute(self) };
        unsafe {
            std::mem::transmute(
                [
                    0u8,
                    0u8,
                    0u8,
                    0u8,
                    0u8,
                    0u8,
                    240u8,
                    (x_bytes[7] & 128u8) | 63u8,
                ]
            )
        }
    }
}