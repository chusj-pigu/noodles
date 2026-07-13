/// todo
pub struct FlatMap<'a> {
    data: Vec<(&'a str, &'a str)>,
}

impl<'a> FlatMap<'a> {
    /// Returns a new `FlatMap`.
    pub fn new(data: Vec<(&'a str, &'a str)>) -> Self {
        Self {
            data
        }
    }

    /// Explicit lookup returning an Option
    pub fn get(&self, key: &str) -> Option<&'a str> {
        self.data
            .iter()
            .find(|(a, _)| *a == key)
            .map(|(_, b)| *b)
    }

    /// Check if key exists
    pub fn contains_key(&self, key: &str) -> bool {
        self.data.iter().any(|(a, _)| a == &key)
    }

    /// Returns the keys.
    pub fn keys(&self) -> impl Iterator<Item = &'a str> {
        self.data.iter().map(|(a, _)| a.clone())
    }

    /// returns the values.
    pub fn values(&self) -> impl Iterator<Item = &'a str> {
        self.data.iter().map(|(_, value)| value.clone())
    }

    /// Returns the number of elements
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if map is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns an iterator over the key-value pairs
    pub fn iter(&self) -> impl Iterator<Item = &(&'a str, &'a str)> {
        self.data.iter()
    }
}