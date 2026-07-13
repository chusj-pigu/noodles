// standard
use std::sync::Arc;
// third party
use arrow::{
    array::{
        Array,
        Int16Array
    },
    datatypes::{
        DataType,
        Field,
    },
    array::{ListArray}
};
use arrow::array::PrimitiveArray;
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
pub struct SignalListArray(ListArray, Int16Array);

impl SignalListArray {
    fn is_valid_list(dt: &DataType) -> bool {
        if let DataType::List(n) = dt {
            return *n.data_type() == DataType::Int16;
        }
        false
    }

    /// Creates a new wrapper around an Arrow array.
    ///
    /// # Errors
    ///
    /// Returns an error if the array.
    #[inline]
    pub fn new(array: ListArray) -> Result<Self, DownCastFailure> {
        if Self::is_valid_list(array.data_type()) {
            let ex = DataType::List(
                Arc::new(Field::new(
                    "item",
                    DataType::Int16,
                    false,
                ))
            );
            let er = DownCastFailure { expected: ex, actual: array.data_type().clone() };
            return Err(er);
        }

        let data = array.values().to_data().clone();

        // The from() calls will not panic if the data has been validated in advance.
        // The decoding process guarantees structural correctness for instantiation
        // but not for satisfaction of data semantic invariants.
        let signales = Int16Array::from(PrimitiveArray::from(data));

        Ok(SignalListArray(array, signales))
    }

    /// Returns the underlying Arrow array.
    #[inline]
    #[must_use]
    pub fn as_slice_map_array(&self) -> &ListArray {
        &self.0
    }

    /// Consumes the wrapper and returns the underlying Arrow array.
    #[inline]
    #[must_use]
    pub fn to_slice_map_array(self) -> ListArray {
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
    pub fn index(&self, index: BatchRowIndex) -> Result<Option<&[i16]>, RowIndexOutOfBounds> {
        let array = self.as_slice_map_array();
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

        let end = start + len;

        Ok(Some(&self.1.values()[start..end]))
    }
}