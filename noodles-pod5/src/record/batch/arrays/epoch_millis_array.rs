// standard

// third party
use arrow::{
    array::{
        Array,
        ArrayRef,
        TimestampMillisecondArray,
    },
    datatypes::{
        DataType,
        TimeUnit,
    },
};
// local
use crate::{
    file::{
        BatchRowIndex,
        RowIndexOutOfBounds,
    },
    record::batch::arrays::DownCastFailure,
};

/// Thin wrapper around Arrow's [`TimestampMillisecondArray`].
///
/// This wrapper removes Arrow types from the public API while providing a
/// consistent indexing interface shared by the crate's array wrappers.
#[repr(transparent)]
#[must_use]
pub struct EpochMillisArray(TimestampMillisecondArray);

impl EpochMillisArray {
    /// Attempts to create a new [`EpochMillisArray`] from an Arrow [`ArrayRef`],
    /// returning [`DownCastFailure`] otherwise.
    #[inline]
    pub fn try_from_array_ref(array_ref: &ArrayRef) -> Result<Self, DownCastFailure> {
        if let DataType::Timestamp(TimeUnit::Millisecond, _) = array_ref.data_type() {
            return Ok(Self::from_raw_parts(array_ref.to_data().into()))
        }
        Err(DownCastFailure{
            actual: array_ref.data_type().clone(),
            expected: DataType::Timestamp(TimeUnit::Millisecond, None)
        })
    }

    /// Creates a new wrapper around an Arrow [`TimestampMillisecondArray`].
    #[inline]
    pub fn from_raw_parts(array: TimestampMillisecondArray) -> Self {
        Self(array)
    }

    /// Consumes the wrapper and returns the underlying Arrow [`TimestampMillisecondArray`].
    #[inline]
    #[must_use]
    pub fn to_raw_parts(self) -> TimestampMillisecondArray {
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