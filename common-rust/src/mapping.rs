use std::{
    borrow::Borrow, collections::HashMap, fmt::Debug, hash::Hash, iter::Zip, ops::RangeFrom,
};

#[derive(Clone)]
pub struct Mapping<T> {
    labels_to_indices: HashMap<T, usize>,
    indices_to_labels: Vec<T>,
}

impl<T> Mapping<T> {
    pub fn len(&self) -> usize {
        self.indices_to_labels.len()
    }

    pub fn label(&self, index: usize) -> &T {
        &self.indices_to_labels[index]
    }

    pub fn iter(&self) -> impl Iterator<Item = (&T, usize)> {
        self.indices_to_labels.iter().zip(0usize..)
    }
}

impl<T> Mapping<T>
where
    T: Clone + Eq + Hash,
{
    pub fn new(indices_to_labels: Vec<T>) -> Self {
        Self {
            labels_to_indices: indices_to_labels
                .iter()
                .map(|label| label.clone())
                .zip(0usize..)
                .collect(),
            indices_to_labels,
        }
    }

    pub fn index<Q>(&self, label: &Q) -> Option<usize>
    where
        T: Borrow<Q>,
        Q: Eq + Hash,
    {
        self.labels_to_indices.get(label).copied()
    }
}

impl<T> FromIterator<T> for Mapping<T>
where
    T: Clone + Eq + Hash,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut mapping = Self {
            labels_to_indices: HashMap::new(),
            indices_to_labels: Vec::new(),
        };
        for (label, index) in iter.into_iter().zip(0usize..) {
            mapping.labels_to_indices.insert(label.clone(), index);
            mapping.indices_to_labels.push(label);
        }
        mapping
    }
}

impl<T> IntoIterator for Mapping<T> {
    type Item = (T, usize);

    type IntoIter = Zip<<Vec<T> as IntoIterator>::IntoIter, RangeFrom<usize>>;

    fn into_iter(self) -> Self::IntoIter {
        self.indices_to_labels.into_iter().zip(0usize..)
    }
}

impl<T> Debug for Mapping<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Mapping ")?;
        f.debug_map().entries(self.iter()).finish()
    }
}
