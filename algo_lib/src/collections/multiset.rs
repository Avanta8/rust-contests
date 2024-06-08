use std::collections::btree_map::{BTreeMap, Iter, Keys, Range};
use std::iter::Map;
use std::ops::RangeBounds;

macro_rules! remove {
    ($self:ident, $expr:expr) => {
        if let Some((key, count)) = $expr {
            let key = key.clone();
            if *count == 1 {
                $self.map.remove(&key);
            } else {
                *count -= 1;
            }
            $self.len -= 1;
            Some(key)
        } else {
            None
        }
    };
}

#[derive(Debug, Default, Clone)]
pub struct MultiSet<T> {
    map: BTreeMap<T, u32>,
    len: usize,
}

impl<T: Ord> MultiSet<T> {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            len: 0,
        }
    }

    pub fn insert(&mut self, key: T) {
        *self.map.entry(key).or_default() += 1;
        self.len += 1;
    }

    pub fn remove(&mut self, key: &T) -> bool {
        if let Some(cnt) = self.map.get_mut(key) {
            if *cnt == 1 {
                self.map.remove(key);
            } else {
                *cnt -= 1;
            }
            self.len -= 1;
            true
        } else {
            false
        }
    }

    pub fn first(&self) -> Option<&T> {
        self.map.iter().next().map(|(key, _cnt)| key)
    }

    pub fn last(&self) -> Option<&T> {
        self.map.iter().next_back().map(|(key, _cnt)| key)
    }

    pub fn remove_first(&mut self) -> Option<T>
    where
        T: Clone,
    {
        remove!(self, self.map.iter_mut().next())
    }

    pub fn remove_last(&mut self) -> Option<T>
    where
        T: Clone,
    {
        remove!(self, self.map.iter_mut().next_back())
    }

    pub fn remove_range_last(&mut self, range: impl RangeBounds<T>) -> Option<T>
    where
        T: Clone,
    {
        remove!(self, self.map.range_mut(range).next_back())
    }

    pub fn remove_range_first(&mut self, range: impl RangeBounds<T>) -> Option<T>
    where
        T: Clone,
    {
        remove!(self, self.map.range_mut(range).next())
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn len_total(&self) -> usize {
        self.len
    }

    pub fn len_groups(&self) -> usize {
        self.map.len()
    }

    pub fn iter(&self) -> Keys<T, u32> {
        self.map.keys()
    }

    pub fn iter_counts(&self) -> Map<Iter<T, u32>, for<'a> fn((&'a T, &u32)) -> (&'a T, usize)> {
        self.map.iter().map(|(key, val)| (key, *val as usize))
    }

    pub fn range(
        &self,
        range: impl RangeBounds<T>,
    ) -> Map<Range<T, u32>, for<'t> fn((&'t T, &u32)) -> &'t T> {
        self.map.range(range).map(|(key, _)| key)
    }

    pub fn range_counts(
        &self,
        range: impl RangeBounds<T>,
    ) -> Map<Range<T, u32>, for<'t> fn((&'t T, &u32)) -> (&'t T, usize)> {
        self.map.range(range).map(|(key, val)| (key, *val as usize))
    }

    pub fn get_count(&self, key: &T) -> usize {
        *self.map.get(key).unwrap_or(&0) as usize
    }
}

impl<T> std::iter::FromIterator<T> for MultiSet<T>
where
    T: Ord,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = Self::new();
        for x in iter {
            set.insert(x);
        }
        set
    }
}
