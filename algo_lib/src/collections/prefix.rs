use crate::numbers::num_traits::{add_sub::AddSub, zero_one::ZeroOne};
use std::ops::{Bound, RangeBounds};

#[derive(Clone, Debug)]
pub struct Prefix<T> {
    pre: Vec<T>,
}

impl<T> Prefix<T>
where
    T: AddSub + ZeroOne + PartialEq,
{
    pub fn new(it: impl IntoIterator<Item = T>, len: usize) -> Self {
        let mut pre = vec![T::zero(); len + 1];

        for (i, x) in it.into_iter().enumerate() {
            pre[i + 1] = pre[i] + x;
        }

        Self { pre }
    }

    pub fn get(&self, range: impl RangeBounds<usize>) -> T {
        let start = match range.start_bound() {
            Bound::Included(&s) => s,
            Bound::Excluded(&s) => s + 1,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(&e) => e + 1,
            Bound::Excluded(&e) => e,
            Bound::Unbounded => self.pre.len() - 1,
        };

        debug_assert!(end < self.pre.len());
        debug_assert!(start <= end);

        self.pre[end] - self.pre[start]
    }
}
