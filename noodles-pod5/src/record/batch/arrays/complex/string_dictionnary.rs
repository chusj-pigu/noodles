// standard

// third party
use arrow::{
    array::{
        Array,
        DictionaryArray,
        StringArray,
        types::{
            Int8Type,
            Int16Type,
            Int32Type,
            Int64Type,
        },
    },
    datatypes::DataType,
};
// local
use crate::file::{
    BatchRowIndex,
    RowIndexOutOfBounds,
};

/// Thin wrapper around Arrow's [`DictionaryArray`].
///
/// This wrapper removes Arrow types from the public API while providing a
/// consistent indexing interface shared by the crate's array wrappers.
pub enum StringDictionary {
    /// variant keyed with signed 8-bit integers
    Int8(DictionaryArray<Int8Type>, StringArray),

    /// variant keyed with signed 16-bit integers
    Int16(DictionaryArray<Int16Type>, StringArray),

    /// variant keyed with signed 32-bit integers
    Int32(DictionaryArray<Int32Type>, StringArray),

    /// variant keyed with signed 64-bit integers
    Int64(DictionaryArray<Int64Type>, StringArray),
}

impl StringDictionary {

    /// Creates a new wrapper around an Arrow array.
    pub fn new(dict: &dyn Array) -> Option<Self> {
        if let DataType::Dictionary(key_type, value_type) = dict.data_type() {
            if **value_type != DataType::Utf8 {
                return None;
            }

            let dict = dict.to_data();
            let array: StringArray = StringArray::from(dict.child_data()[0].clone());

            if **key_type == DataType::Int8 {
                return Some(Self::Int8(DictionaryArray::<Int8Type>::from(dict), array))
            } else if **key_type == DataType::Int16 {
                return Some(Self::Int16(DictionaryArray::<Int16Type>::from(dict), array))
            } else if **key_type == DataType::Int32 {
                return Some(Self::Int32(DictionaryArray::<Int32Type>::from(dict), array))
            } else if **key_type == DataType::Int64 {
                return Some(Self::Int64(DictionaryArray::<Int64Type>::from(dict), array))
            }
        }
        None
    }

    /// Returns the value stored at the given batch row.
    ///
    /// Returns `Ok(None)` if the value is null.
    ///
    /// # Errors
    ///
    /// Returns an error if the row index is out of bounds.
    pub fn index(&self, index: BatchRowIndex) -> Result<Option<&str>, RowIndexOutOfBounds> {
        let index: usize = index.into();

        if index >= match self {
            Self::Int8(dict,_) => dict.len(),
            Self::Int16(dict,_) => dict.len(),
            Self::Int32(dict,_) => dict.len(),
            Self::Int64(dict,_) => dict.len(),
        } {
            return Err(RowIndexOutOfBounds);
        }
        if match self {
            Self::Int8(dict,_) => dict.is_null(index),
            Self::Int16(dict,_) => dict.is_null(index),
            Self::Int32(dict,_) => dict.is_null(index),
            Self::Int64(dict,_) => dict.is_null(index),
        } {
            return Ok(None);
        }

        match self {
            Self::Int8(dict, array) => {
                let key = dict.keys().value(index) as usize;
                Ok(Some(array.value(key)))
            },
            Self::Int16(dict, array) => {
                let key = dict.keys().value(index) as usize;
                Ok(Some(array.value(key)))
            },
            Self::Int32(dict, array) => {
                let key = dict.keys().value(index) as usize;
                Ok(Some(array.value(key)))
            },
            Self::Int64(dict, array) => {
                let key = dict.keys().value(index) as usize;
                Ok(Some(array.value(key)))
            },
        }
    }
}

