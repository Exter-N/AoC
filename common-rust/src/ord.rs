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
        let position = match self.0.binary_search(&value) {
            Ok(index) => index,
            Err(index) => index,
        };
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

pub fn insert_sorted<T>(vec: &mut Vec<T>, element: T) -> usize
where
    T: Ord,
{
    let index = match vec.binary_search(&element) {
        Ok(index) => index,
        Err(index) => index,
    };
    vec.insert(index, element);
    index
}

pub fn insert_sorted_by_key<T, U, F>(vec: &mut Vec<T>, element: T, mut key_extractor: F) -> usize
where
    U: Ord,
    F: FnMut(&T) -> U,
{
    let index = match vec.binary_search_by_key(&key_extractor(&element), key_extractor) {
        Ok(index) => index,
        Err(index) => index,
    };
    vec.insert(index, element);
    index
}

pub fn remove_sorted<T>(vec: &mut Vec<T>, element: &T) -> Option<T>
where
    T: Ord,
{
    if let Ok(index) = vec.binary_search(element) {
        Some(vec.remove(index))
    } else {
        None
    }
}

pub fn remove_sorted_by_key<T, U, F>(vec: &mut Vec<T>, key: &U, key_extractor: F) -> Option<T>
where
    U: Ord,
    F: FnMut(&T) -> U,
{
    if let Ok(index) = vec.binary_search_by_key(key, key_extractor) {
        Some(vec.remove(index))
    } else {
        None
    }
}

pub fn binary_search<T>(slice: &[T], value: &T) -> (bool, usize)
where
    T: Ord,
{
    match slice.binary_search(value) {
        Ok(pos) => (true, pos),
        Err(pos) => (false, pos),
    }
}

pub fn binary_search_by<T, U, F>(slice: &[T], value: &U, mut compare: F) -> (bool, usize)
where
    F: FnMut(&T, &U) -> Ordering,
{
    match slice.binary_search_by(move |element| compare(element, value)) {
        Ok(pos) => (true, pos),
        Err(pos) => (false, pos),
    }
}
