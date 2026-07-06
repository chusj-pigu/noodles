// standard
use std::{
    fmt,
    error::Error,
};
use std::fmt::{Display, Formatter};
// third party

// local

pub struct FileRowIndex;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// The `RowIndex` within a single [`Batch`](crate::record::batch::Batch).
pub struct BatchRowIndex(u32);

impl BatchRowIndex {
    /// Returns a new `BatchRowIndex`.
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    /// Converts a copied slice of `u32` into `BatchRowIndexes`.
    pub fn from_slice(values: &[u32]) -> Vec<Self> {
        values.iter().copied().map(BatchRowIndex::from).collect()
    }
}

impl From<u32> for BatchRowIndex {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<BatchRowIndex> for usize {
    fn from(wrapper: BatchRowIndex) -> Self {
        wrapper.0 as usize
    }
}

pub struct RowCount;

#[derive(Debug)]
/// An error where the user passes in a row index which is too big.
pub struct RowIndexOutOfBounds;

impl Display for RowIndexOutOfBounds {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Row index is out of bounds")
    }
}

impl Error for RowIndexOutOfBounds {}