use crate::collections::fxhash::FxHashMap;
use crate::numbers::num_traits::{add_sub::AddSub, zero_one::ZeroOne};
use std::{
    cmp::min,
    ops::{Bound, RangeBounds},
};
// use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct SparsePrefix<T> {
    d: usize,
    len: usize,
    pre: Vec<T>,
    // changes: BTreeMap<usize, T>,
    changes: FxHashMap<usize, T>,
}

impl<T> SparsePrefix<T>
where
    T: AddSub + ZeroOne + PartialEq,
{
    pub fn new(it: impl IntoIterator<Item = T>, len: usize, d: usize) -> Self {
        let mut pre = vec![T::zero(); (len + d - 1) / d + 1];

        // let mut changes = BTreeMap::new();
        let mut changes = FxHashMap::default();
        let mut chunk = T::zero();

        for (i, x) in it.into_iter().enumerate() {
            if x != T::zero() {
                changes.insert(i, x);
            }
            chunk += x;
            if (i + 1) % d == 0 {
                pre[(i + 1) / d] = pre[i / d] + chunk;
                chunk = T::zero();
            }
        }

        if len % d != 0 {
            pre[len / d + 1] = pre[len / d] + chunk;
        }

        Self {
            d,
            len,
            pre,
            changes,
        }
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
            Bound::Unbounded => self.len,
        };

        debug_assert!(end <= self.len);
        debug_assert!(start <= end);

        // get from start (inclusive) to end (exclusive)
        let front = (start + self.d - 1) / self.d;
        let back = end / self.d;

        let mut total = T::zero();
        for i in start..min(self.len, front * self.d) {
            if let Some(&x) = self.changes.get(&i) {
                total += x;
            }
        }
        // for (_i, &x) in self.changes.range(start..min(self.len, front * self.d)) {
        //     total += x;
        // }

        for i in back * self.d..end {
            if let Some(&x) = self.changes.get(&i) {
                total += x;
            }
        }
        // for (_i, &x) in self.changes.range(back * self.d..end) {
        //     total += x;
        // }

        total += self.pre[back];
        total -= self.pre[front];

        total
    }
}
