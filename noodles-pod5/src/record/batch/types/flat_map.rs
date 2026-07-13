/// A lightweight, contiguous vector-backed map.
///
/// It mainly serves to give convenient access to the data.
/// It makes no promises as regards to the uniqueness of keys,
/// nor does it defend against such.
pub struct FlatMap<'a> {
    data: Vec<(&'a str, &'a str)>,
}

impl<'a> FlatMap<'a> {
    /// Returns a new `FlatMap` from a vector of key-value tuples.
    pub fn new(data: Vec<(&'a str, &'a str)>) -> Self {
        Self {
            data
        }
    }

    /// Returns the value corresponding to the supplied key.
    ///
    /// If multiple identical keys exist, this returns the value of the first matching pair.
    pub fn get(&self, key: &str) -> Option<&'a str> {
        self.data
            .iter()
            .find(|(a, _)| *a == key)
            .map(|(_, b)| *b)
    }

    /// Returns `true` if the map contains a value for the specified key.
    pub fn contains_key(&self, key: &str) -> bool {
        self.data.iter().any(|(a, _)| a == &key)
    }

    /// Returns an iterator visiting all keys in insertion order.
    pub fn keys(&self) -> impl Iterator<Item = &'a str> {
        self.data.iter().map(|(a, _)| a.clone())
    }

    /// Returns an iterator visiting all values in insertion order.
    pub fn values(&self) -> impl Iterator<Item = &'a str> {
        self.data.iter().map(|(_, value)| value.clone())
    }

    /// Returns the number of elements in the map.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns `true` if the map contains no elements.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns an iterator over the key-value reference pairs.
    pub fn iter(&self) -> impl Iterator<Item = &(&'a str, &'a str)> {
        self.data.iter()
    }
}