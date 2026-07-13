// standard

// third party
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

/// Thin wrapper around Arrow's [`MapArray`].
///
/// This wrapper removes Arrow types from the public API while providing a
/// consistent indexing interface shared by the crate's array wrappers.
pub struct SliceMapArray(MapArray, StringArray, StringArray);

impl SliceMapArray {
    fn is_valid_map(dt: &DataType) -> bool {
        if let DataType::Map(entries_field, _) = dt {
            if let DataType::Struct(fields) = entries_field.data_type() {
                return fields.len() == 2
                    && *fields[0].data_type() == DataType::Utf8
                    && *fields[1].data_type() == DataType::Utf8;
            }
        }
        false
    }

    /// Creates a new wrapper around an Arrow array.
    #[inline]
    pub fn new(array: MapArray) -> Result<Self, DownCastFailure> {
        if Self::is_valid_map(array.data_type()) {
            let ex = DataType::Map(
                FieldRef::new(Field::new(
                    "entries",
                    DataType::Struct(Fields::from(vec![
                        Field::new("key", DataType::Utf8, false),
                        Field::new("value", DataType::UInt8, true),
                    ])),
                    false,
                )),
                false, 
            );
            let er = DownCastFailure { expected: ex, actual: array.data_type().clone() };
            return Err(er);
        }

        let keys_data = array.keys().to_data().clone();
        let values_data = array.values().to_data().clone();

        // The from() calls will not panic if the data has been validated in advance.
        // The decoding process guarantees structural correctness for instantiation
        // but not for satisfaction of data semantic invariants.
        let keys = StringArray::from(keys_data);
        let values = StringArray::from(values_data);

        Ok(SliceMapArray(array, keys, values))
    }

    /// Returns the underlying Arrow array.
    #[inline]
    #[must_use]
    pub fn as_slice_map_array(&self) -> &MapArray {
        &self.0
    }

    /// Consumes the wrapper and returns the underlying Arrow array.
    #[inline]
    #[must_use]
    pub fn to_slice_map_array(self) -> MapArray {
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
    pub fn index(&self, index: BatchRowIndex) -> Result<Option<FlatMap>, RowIndexOutOfBounds> {
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

        let data = (start..start+len)
            .map(|i| (self.1.value(i), self.2.value(i))).collect();
        Ok(Some(FlatMap::new(data)))
    }
}