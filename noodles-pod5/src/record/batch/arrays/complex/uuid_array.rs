// standard
use std::{
    fmt, 
    error::Error
};
// third party
use arrow::array::{FixedSizeBinaryArray, Array, ArrayRef, ListArray, UInt64Array};
use arrow::datatypes::DataType;
// local
use crate::{
    file::{
        BatchRowIndex,
        RowIndexOutOfBounds,
    },
    record::batch::types::Uuid,
};
use crate::record::batch::arrays::DownCastFailure;

/// A non-null UUID value did not contain exactly 16 bytes.
///
/// This indicates that the underlying Arrow array contains invalid data.
/// The contained value is the number of bytes that were found.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UuidArrayLenghtError(usize);

impl fmt::Display for UuidArrayLenghtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UUID value contains {} bytes instead of 16.", self.0)
    }
}

impl Error for UuidArrayLenghtError {}

/// Errors returned when accessing values from a [`UuidArray`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UuidArrayError {
    /// The requested row index is outside the bounds of the array.
    RowIndexOutOfBounds(RowIndexOutOfBounds),

    /// A non-null UUID value did not contain exactly 16 bytes.
    InvalidLength(UuidArrayLenghtError),
}

impl fmt::Display for UuidArrayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RowIndexOutOfBounds(e) => write!(f, "Uuid array index error: {}", e),
            Self::InvalidLength(e) => write!(f, "Uuid array length error: {}", e),
        }
    }
}

impl Error for UuidArrayError {}

/// Thin wrapper around Arrow's [`FixedSizeBinaryArray`].
///
/// This wrapper removes Arrow types from the public API while providing a
/// consistent indexing interface shared by the crate's array wrappers.
///
/// Each non-null element is expected to contain the data of a 16-byte UUID.
#[repr(transparent)]
pub struct UuidArray(FixedSizeBinaryArray);

impl UuidArray {
    /// Attempts to create a new [`UuidArray`] from an Arrow [`ArrayRef`],
    /// returning [`DownCastFailure`] otherwise.
    #[inline]
    #[must_use]
    pub fn try_from_array_ref(array_ref: &ArrayRef) -> Result<Self, DownCastFailure> {
        let expected = DataType::FixedSizeBinary(16);
        if array_ref.data_type() != &expected {
            return Err(DownCastFailure{
                actual: array_ref.data_type().clone(),
                expected
            });
        }
        Ok(Self::from_raw_parts(array_ref.to_data().into()))
    }

    /// Creates a new wrapper arround an Arrow [`FixedSizeBinaryArray`].
    #[inline]
    #[must_use]
    pub fn from_raw_parts(array: FixedSizeBinaryArray) -> Self {
        Self(array)
    }

    /// Consumes the wrapper and returns the underlying Arrow [`FixedSizeBinaryArray`].
    #[inline]
    #[must_use]
    pub fn to_raw_parts(self) -> (FixedSizeBinaryArray) {
        (self.0)
    }

    /// Returns the UUID stored at the given batch row.
    ///
    /// Returns `Ok(None)` if the value is null.
    ///
    /// # Errors
    ///
    /// Returns an error if the row index is out of bounds or if a non-null
    /// value does not contain exactly 16 bytes.
    #[inline]
    #[must_use]
    pub fn index(&self, index: BatchRowIndex) -> Result<Option<Uuid>, UuidArrayError> {
        let array = &self.0;
        let index: usize = index.into();

        if index >= array.len() {
            return Err(UuidArrayError::RowIndexOutOfBounds(RowIndexOutOfBounds));
        }

        if array.is_null(index) {
            return Ok(None);
        }

        let value = array.value(index);
        let len = value.len();

        if let Ok(data) = <&[u8; 16]>::try_from(value) {
            return Ok(Some(Uuid::from_bytes(data)));
        }

        return Err(UuidArrayError::InvalidLength(UuidArrayLenghtError(len)));
    }
}