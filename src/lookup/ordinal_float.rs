use num_traits::float::Float;

#[derive(Default, Debug, Clone, Copy, PartialEq,  PartialOrd)]
pub struct FloatOrd<T: Float>(pub T);
impl<T: Float> FloatOrd<T> {
    #[allow(dead_code)]
    pub fn new() -> Self {
        FloatOrd(T::zero())
    }
}
impl<T: Float> Eq for FloatOrd<T> {}
impl<T: Float> Ord for FloatOrd<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}