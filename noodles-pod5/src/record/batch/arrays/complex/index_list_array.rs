// standard
use std::sync::Arc;
// third party
use arrow::{
    array::{
        Array,
        ArrayRef,
        ListArray,
        PrimitiveArray,
        UInt64Array,
    },
    datatypes::{
        DataType,
        Field,
    },
};
use arrow::datatypes::FieldRef;
// local
use crate::{
    file::{
        BatchRowIndex,
        RowIndexOutOfBounds,
    },
    record::batch::arrays::DownCastFailure,
};

/// Thin wrapper around Arrow's [`ListArray`].
///
/// This wrapper removes Arrow types from the public API while providing a
/// consistent indexing interface shared by the crate's array wrappers.
pub struct IndexListArray {
    /// The wrapped array.
    wrapped_array: ListArray,
    /// The downcasted data array.
    data_array: UInt64Array,
}

impl IndexListArray {
    /// Verifies if the [`data_type`](DataType) corresponds to a `IndexListArray`,
    /// returning [`DownCastFailure`] otherwise.
    #[must_use]
    fn is_valid_index_list_array(data_type: &DataType) -> Result<(), DownCastFailure> {
        if let DataType::List(field_ref) = data_type {
            if *field_ref.data_type() == DataType::UInt64 {
                return Ok(());
            }
        }

        let expected = DataType::List(FieldRef::new(Field::new(
            "item",
            DataType::UInt64,
            false,
        )));

        Err(DownCastFailure {
            expected,
            actual: data_type.clone(),
        })
    }

    /// Attempts to create a new [`IndexListArray`] from an Arrow [`ArrayRef`],
    /// returning [`DownCastFailure`] otherwise.
    #[inline]
    #[must_use]
    pub fn try_from_array_ref(array_ref: &ArrayRef) -> Result<Self, DownCastFailure> {
        Self::is_valid_index_list_array(array_ref.data_type())?;
        let array: ListArray = array_ref.to_data().into();
        let data = array.values().to_data().clone();
        Ok(Self::from_raw_parts(array, PrimitiveArray::from(data).into()))
    }

    /// Creates a new wrapper around an Arrow [`ListArray`] and [`UInt64Array`].
    #[inline]
    #[must_use]
    pub fn from_raw_parts(wrapped_array: ListArray, data_array: UInt64Array) -> Self {
        Self {
            wrapped_array,
            data_array,
        }
    }

    /// Consumes the wrapper and returns the underlying Arrow [`ListArray`] and [`UInt64Array`].
    #[inline]
    #[must_use]
    pub fn to_raw_parts(self) -> (ListArray, UInt64Array) {
        (self.wrapped_array, self.data_array)
    }

    /// Returns the value stored at the given batch row.
    ///
    /// Returns `Ok(None)` if the value is null.
    ///
    /// # Errors
    ///
    /// Returns an error if the row index is out of bounds.
    #[inline]
    #[must_use]
    pub fn index(&self, index: BatchRowIndex) -> Result<Option<&[u64]>, RowIndexOutOfBounds> {
        let array = &self.wrapped_array;
        let index: usize = index.into();

        if index >= array.len() {
            return Err(RowIndexOutOfBounds);
        }

        if array.is_null(index) {
            return Ok(None);
        }

        let offsets = array.offsets();
        let start = offsets[index] as usize;
        let len = array.value_length(index) as usize;

        Ok(Some(&self.data_array.values()[start..start + len]))
    }
}