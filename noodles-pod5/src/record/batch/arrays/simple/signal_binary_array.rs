// standard

// third party
use arrow::array::{
    LargeBinaryArray,
    Array,
};
// local
use crate::file::{
    BatchRowIndex,
    RowIndexOutOfBounds,
};

/// Thin wrapper around Arrow's [`LargeSizeBinaryArray`].
///
/// This wrapper removes Arrow types from the public API while providing a
/// consistent indexing interface shared by the crate's array wrappers.
#[repr(transparent)]
pub struct SignalBinaryArray(LargeBinaryArray);

impl SignalBinaryArray {
    /// Creates a new [`SignalBinaryArray`] from an Arrow array.
    #[inline]
    #[must_use]
    pub fn new(array: LargeBinaryArray) -> Self {
        SignalBinaryArray(array)
    }

    /// Returns the underlying Arrow array.
    #[inline]
    #[must_use]
    pub fn as_large_binary_array(&self) -> &LargeBinaryArray {
        &self.0
    }

    /// Consumes the wrapper and returns the underlying Arrow array.
    #[inline]
    #[must_use]
    pub fn to_large_binary_array(self) -> LargeBinaryArray {
        self.0
    }

    /// Returns the UUID stored at the given batch row.
    ///
    /// Returns `Ok(None)` if the value is null.
    ///
    /// # Errors
    ///
    /// Returns an error if the row index is out of bounds.
    #[inline]
    pub fn index(&self, index: BatchRowIndex) -> Result<Option<&[u8]>, RowIndexOutOfBounds> {
        let array = &self.0;
        let index: usize = index.into();

        if index >= array.len() {
            return Err(RowIndexOutOfBounds);
        }

        if array.is_null(index) {
            return Ok(None);
        }

        Ok(Some(array.value(index)))
    }
}