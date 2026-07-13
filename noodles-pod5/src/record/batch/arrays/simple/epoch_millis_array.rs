// standard

// third party
use arrow::array::{
    TimestampMillisecondArray, 
    Array,
};
// local
use crate::file::{
    BatchRowIndex, 
    RowIndexOutOfBounds,
};

/// Thin wrapper around Arrow's [`TimestampMillisecondArray`].
///
/// This wrapper removes Arrow types from the public API while providing a
/// consistent indexing interface shared by the crate's array wrappers.
#[repr(transparent)]
pub struct EpochMillisArray(TimestampMillisecondArray);

impl EpochMillisArray {
    /// Creates a new wrapper around an Arrow array.
    #[inline]
    #[must_use]
    pub fn new(array: TimestampMillisecondArray) -> Self {
        EpochMillisArray(array)
    }

    /// Returns the underlying Arrow array.
    #[inline]
    #[must_use]
    pub fn as_epoch_millis_array(&self) -> &TimestampMillisecondArray {
        &self.0
    }

    /// Consumes the wrapper and returns the underlying Arrow array.
    #[inline]
    #[must_use]
    pub fn to_epoch_millis_array(self) -> TimestampMillisecondArray {
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
    pub fn index(&self, index: BatchRowIndex) -> Result<Option<i64>, RowIndexOutOfBounds> {
        let array = self.as_epoch_millis_array();
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