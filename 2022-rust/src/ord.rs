use std::cmp::{min, Ordering};
use std::collections::HashMap;
use std::hash::Hash;
use std::mem::swap;
use std::ops::Deref;

#[derive(Debug)]
#[repr(transparent)]
pub struct Top<T, const N: usize>([T; N])
where
    T: Ord;

impl<T, const N: usize> Top<T, N>
where
    T: Ord,
{
    pub fn insert(&mut self, value: T) {
        let (_, position) = binary_search(&self.0, &value);
        let mut v = value;
        for i in (0..position).rev() {
            swap(&mut v, &mut self.0[i]);
        }
    }
}

impl<T, const N: usize> Top<T, N>
where
    T: Copy + Ord,
{
    pub fn new(value: T) -> Self {
        Self([value; N])
    }
}

impl<T, const N: usize> Default for Top<T, N>
where
    T: Copy + Default + Ord,
{
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T, const N: usize> Deref for Top<T, N>
where
    T: Ord,
{
    type Target = [T; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug, Default)]
pub struct ProximityMap<K, V>(HashMap<K, V>)
where
    V: Ord;

impl<K, V> ProximityMap<K, V>
where
    V: Ord,
{
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

impl<K, V> ProximityMap<K, V>
where
    K: Eq + Hash,
    V: Copy + Ord,
{
    pub fn insert(&mut self, k: K, v: V) {
        if let Some(existing) = self.0.get_mut(&k) {
            *existing = min(*existing, v);
        } else {
            self.0.insert(k, v);
        }
    }
    pub fn remove(&mut self, k: &K) -> Option<V> {
        self.0.remove(k)
    }
}

impl<K, V> Deref for ProximityMap<K, V>
where
    V: Ord,
{
    type Target = HashMap<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn binary_search<T>(slice: &[T], value: &T) -> (bool, usize)
where
    T: Ord,
{
    if slice.is_empty() {
        return (false, 0);
    }

    let mut left: usize = 0;
    let mut right: usize = slice.len() - 1;

    while left <= right {
        let middle = left + (right - left) / 2;
        match slice[middle].cmp(value) {
            Ordering::Equal => {
                return (true, middle);
            }
            Ordering::Less => {
                left = middle + 1;
            }
            Ordering::Greater => {
                if middle == 0 {
                    return (false, left);
                }
                right = middle - 1;
            }
        }
    }

    (false, left)
}

pub fn binary_search_by<T, U, F>(slice: &[T], value: &U, mut compare: F) -> (bool, usize)
where
    F: FnMut(&T, &U) -> Ordering,
{
    if slice.is_empty() {
        return (false, 0);
    }

    let mut left: usize = 0;
    let mut right: usize = slice.len() - 1;

    while left <= right {
        let middle = left + (right - left) / 2;
        match compare(&slice[middle], value) {
            Ordering::Equal => {
                return (true, middle);
            }
            Ordering::Less => {
                left = middle + 1;
            }
            Ordering::Greater => {
                if middle == 0 {
                    return (false, left);
                }
                right = middle - 1;
            }
        }
    }

    (false, left)
}
