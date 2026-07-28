// standard

use std::iter::Map;
// third party
use std::sync::Arc;
use arrow::{
    array::{
        MapArray,
        Array,
        StringArray,
    },
    datatypes::{
        DataType,
        Field,
        Fields,
        FieldRef,
    },
};
use arrow::array::ArrayRef;
// local
use crate::{
    file::{
        BatchRowIndex,
        RowIndexOutOfBounds,
    },
    record::batch::{
        arrays::DownCastFailure,
        types::FlatMap,
    },
};
use crate::record::batch::arrays::UuidArray;

/// Thin wrapper around Arrow's [`MapArray`].
///
/// This wrapper removes Arrow types from the public API while providing a
/// consistent indexing interface shared by the crate's array wrappers.
pub struct SliceMapArray {
    /// The wrapped array.
    wrapped_array: MapArray,
    /// The downcasted keys array.
    keys_array: StringArray,
    /// The downcasted values array.
    values_array: StringArray
}

impl SliceMapArray {
    /// Verifies if the [`data_type`](DataType) corresponds to a `SliceMapArray`,
    /// returning [`DownCastFailure`] otherwise.
    #[must_use]
    fn is_valid_slice_map_array(data_type: &DataType) -> Result<(), DownCastFailure> {
        if let DataType::List(field_ref) = data_type {
            if field_ref.data_type() == &DataType::UInt64 {
                return Ok(());
            }
        }
        if let DataType::Map(field_ref, _) = data_type {
            if let DataType::Struct(fields) = field_ref.data_type() {
                if fields.len() == 2
                    && *fields[0].data_type() == DataType::Utf8
                    && *fields[1].data_type() == DataType::Utf8 {
                    return Ok(());
                }
            }
        }

        let expected = DataType::Map(
            FieldRef::new(Field::new(
                "entries",
                DataType::Struct(Fields::from(vec![
                    Field::new("key", DataType::Utf8, false),
                    Field::new("value", DataType::Utf8, true),
                ])),
                false,
            )),
            false,
        );

        Err(DownCastFailure {
            expected,
            actual: data_type.clone(),
        })
    }

    /// Attempts to create a new [`SliceMapArray`] from an Arrow [`ArrayRef`],
    /// returning [`DownCastFailure`] otherwise.
    #[inline]
    #[must_use]
    pub fn try_from_array_ref(array_ref: &ArrayRef) -> Result<Self, DownCastFailure> {
        Self::is_valid_slice_map_array(array_ref.data_type())?;
        let array: MapArray = array_ref.to_data().into();
        let keys = array.keys().to_data().clone();
        let values = array.values().to_data().clone();
        Ok(Self::from_raw_parts(array, keys.into(), values.into()))
    }

    /// Creates a new wrapper around an Arrow [`MapArray`], [`StringArray`] and [`StringArray`].
    #[inline]
    #[must_use]
    pub fn from_raw_parts(wrapped_array: MapArray, keys_array: StringArray, values_array: StringArray) -> Self {
        Self {
            wrapped_array,
            keys_array,
            values_array,
        }
    }

    /// Consumes the wrapper and returns the underlying Arrow [`MapArray`], [`StringArray`] and [`StringArray`].
    #[inline]
    #[must_use]
    pub fn to_raw_parts(self) -> (MapArray, StringArray, StringArray) {
        (self.wrapped_array, self.keys_array, self.values_array)
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
    pub fn index(&self, index: BatchRowIndex) -> Result<Option<FlatMap>, RowIndexOutOfBounds> {
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

        let data = (start..start+len)
            .map(|i| (self.keys_array.value(i), self.values_array.value(i))).collect();
        Ok(Some(FlatMap::new(data)))
    }
}