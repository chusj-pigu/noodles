// standard

// third party
use crate::record::batch::arrays::{DownCastFailure, SignalListArray};
use arrow::{
    array::{
        ArrayRef,
        ArrayData,
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
use arrow::array::{Array, Int16Array, ListArray, PrimitiveArray};
// local
use crate::file::{
    BatchRowIndex,
    RowIndexOutOfBounds,
};

/// A type-erased wrapper around [`DictionaryArray`] that unifies its index types.
pub enum StringDictionaryArray {
    /// variant keyed with signed 8-bit integers
    Int8(DictionaryArray<Int8Type>),

    /// variant keyed with signed 16-bit integers
    Int16(DictionaryArray<Int16Type>),

    /// variant keyed with signed 32-bit integers
    Int32(DictionaryArray<Int32Type>),

    /// variant keyed with signed 64-bit integers
    Int64(DictionaryArray<Int64Type>),
}

impl StringDictionaryArray {
    /// Returns the length of the [`StringDictionaryArray`].
    pub fn len(&self) -> usize {
        match self {
            Self::Int8(dictionary) => dictionary.len(),
            Self::Int16(dictionary) => dictionary.len(),
            Self::Int32(dictionary) => dictionary.len(),
            Self::Int64(dictionary) => dictionary.len(),
        }
    }
    
    /// Returns whether the element of the [`StringDictionaryArray`] at `index` is null.
    pub fn is_null(&self, index: usize) -> bool {
        match self {
            Self::Int8(dictionary) => dictionary.is_null(index),
            Self::Int16(dictionary) => dictionary.is_null(index),
            Self::Int32(dictionary) => dictionary.is_null(index),
            Self::Int64(dictionary) => dictionary.is_null(index),
        }
    }

    /// Returns the index for the element of the [`StringDictionaryArray`] at `index`.
    /// 
    /// (The elements are stored in a separate array)
    pub fn key(&self, index: usize) -> usize {
        match self {
            Self::Int8(dictionary) => dictionary.keys().value(index) as usize,
            Self::Int16(dictionary) => dictionary.keys().value(index) as usize,
            Self::Int32(dictionary) => dictionary.keys().value(index) as usize,
            Self::Int64(dictionary) => dictionary.keys().value(index) as usize,
        }
    }
}

/// Thin wrapper around Arrow's [`DictionaryArray`].
///
/// This wrapper removes Arrow types from the public API while providing a
/// consistent indexing interface shared by the crate's array wrappers.
pub struct StringDictionary {
    /// The wrapped array.
    wrapped_array: StringDictionaryArray,
    /// The downcasted data array.
    data_array: StringArray,
}

impl StringDictionary {

    /// Verifies if the [`data_type`](DataType) corresponds to a `SignalListArray`,
    /// returning [`DownCastFailure`] otherwise.
    #[must_use]
    fn is_valid_signal_list_array(data_type: &DataType) -> Result<&DataType, DownCastFailure> {
        let key_type = if let DataType::Dictionary(key_type, value_type) = data_type {
            if **value_type == DataType::Utf8 {
                match **key_type {
                    DataType::Int8 | DataType::Int16 | DataType::Int32 | DataType::Int64 => return Ok(key_type),
                    _ => {}
                }
            }
            match **key_type {
                DataType::Int8 => DataType::Int8,
                DataType::Int16 => DataType::Int16,
                DataType::Int64 => DataType::Int64,
                DataType::Int32 | _ => DataType::Int32,
            }
        } else {
            DataType::Int32
        };

        let expected = DataType::Dictionary(Box::new(key_type), Box::new(DataType::Utf8));

        Err(DownCastFailure {
            expected,
            actual: data_type.clone(),
        })
    }

    /// Attempts to create a new [`SignalListArray`] from an Arrow [`ArrayRef`],
    /// returning [`DownCastFailure`] otherwise.
    #[inline]
    #[must_use]
    pub fn try_from_array_ref(array_ref: &ArrayRef) -> Result<Self, DownCastFailure> {
        let index_type = Self::is_valid_signal_list_array(array_ref.data_type())?;
        let dictionnary = array_ref.to_data();
        let data = dictionnary.child_data()[0].clone();
        let array: StringDictionaryArray = match index_type {
            DataType::Int8 => StringDictionaryArray::Int8(dictionnary.into()),
            DataType::Int16 => StringDictionaryArray::Int16(dictionnary.into()),
            DataType::Int32 => StringDictionaryArray::Int32(dictionnary.into()),
            DataType::Int64 => StringDictionaryArray::Int64(dictionnary.into()),
            _ => unimplemented!(),
        };
        Ok(Self::from_raw_parts(array, data.into()))
    }

    /// Creates a new wrapper around an Arrow [`StringDictionaryArray`] and [`StringArray`].
    #[inline]
    #[must_use]
    pub fn from_raw_parts(wrapped_array: StringDictionaryArray, data_array: StringArray) -> Self {
        Self {
            wrapped_array,
            data_array,
        }
    }

    /// Consumes the wrapper and returns the underlying Arrow [`StringDictionaryArray`] and [`StringArray`].
    #[inline]
    #[must_use]
    pub fn to_raw_parts(self) -> (StringDictionaryArray, StringArray) {
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
    pub fn index(&self, index: BatchRowIndex) -> Result<Option<&str>, RowIndexOutOfBounds> {
        let array = &self.wrapped_array;
        let index: usize = index.into();

        if index >= array.len() {
            return Err(RowIndexOutOfBounds);
        }

        if array.is_null(index) {
            return Ok(None);
        }

        let key = array.key(index);
        Ok(Some(self.data_array.value(key)))
    }
}

