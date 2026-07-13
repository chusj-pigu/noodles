// standard

// third party
use arrow::array::{
    self, 
    Array,
};
// local
use crate::file::{
    BatchRowIndex, 
    RowIndexOutOfBounds,
};

/// Thin wrapper around Arrow's [`UInt64Array`].
///
/// This wrapper removes Arrow types from the public API while providing a
/// consistent indexing interface shared by the crate's array wrappers.
#[repr(transparent)]
pub struct UInt64Array(array::UInt64Array);

impl UInt64Array {
    /// Creates a new wrapper around an Arrow array.
    #[inline]
    #[must_use]
    pub fn new(array: array::UInt64Array) -> Self {
        UInt64Array(array)
    }

    /// Returns the underlying Arrow array.
    #[inline]
    #[must_use]
    pub fn as_uint64_array(&self) -> &array::UInt64Array {
        &self.0
    }

    /// Consumes the wrapper and returns the underlying Arrow array.
    #[inline]
    #[must_use]
    pub fn to_uint64_array(self) -> array::UInt64Array {
        self.0
    }

    /// Returns the value stored at the given batch row.
    ///
    /// Returns `Ok(None)` if the value is null.
    ///
    /// # Errors
    ///
    /// Returns an error if the row index is out of bounds.
    #[inline]
    pub fn index(&self, index: BatchRowIndex) -> Result<Option<u64>, RowIndexOutOfBounds> {
        let array = self.as_uint64_array();
        let index: usize = index.into();

        if index >= array.len() {
            return Err(RowIndexOutOfBounds);
        }

        if array.is_null(index) {
            return Ok(None);
        }

        return Ok(Some(array.value(index)));
    }
}