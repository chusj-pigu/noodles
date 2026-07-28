// standard

// third party
use arrow::array::{self, Array, ArrayRef};
use arrow::datatypes::DataType;
// local
use crate::file::{
    BatchRowIndex,
    RowIndexOutOfBounds,
};
use crate::record::batch::arrays::{DownCastFailure};

/// Thin wrapper around Arrow's [`UInt32Array`].
///
/// This wrapper removes Arrow types from the public API while providing a
/// consistent indexing interface shared by the crate's array wrappers.
#[repr(transparent)]
pub struct UInt32Array(array::UInt32Array);

impl UInt32Array {

    /// Attempts to create a new [`UInt32Array`] from an Arrow [`ArrayRef`],
    /// returning [`DownCastFailure`] otherwise.
    #[inline]
    #[must_use]
    pub fn try_from_array_ref(array_ref: &ArrayRef) -> Result<Self, DownCastFailure> {
        let expected = DataType::UInt32;
        if array_ref.data_type() != &expected {
            return Err(DownCastFailure{
                actual: array_ref.data_type().clone(),
                expected
            });
        }
        Ok(Self::from_raw_parts(array_ref.to_data().into()))
    }

    /// Creates a new wrapper arround an Arrow [`UInt32Array`](array::UInt32Array).
    #[inline]
    #[must_use]
    pub fn from_raw_parts(array: array::UInt32Array) -> Self {
        Self(array)
    }

    /// Consumes the wrapper and returns the underlying Arrow [`UInt32Array`](array::UInt32Array).
    #[inline]
    #[must_use]
    pub fn to_raw_parts(self) -> (array::UInt32Array) {
        (self.0)
    }

    /// Returns the value stored at the given batch row.
    ///
    /// Returns `Ok(None)` if the value is null.
    ///
    /// # Errors
    ///
    /// Returns an error if the row index is out of bounds.
    #[inline]
    pub fn index(&self, index: BatchRowIndex) -> Result<Option<u32>, RowIndexOutOfBounds> {
        let array = &self.0;
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