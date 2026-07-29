// standard

// third party
use arrow::{
    array::{
        Array,
        ArrayRef,
        self,
    },
    datatypes::DataType,
};
// local
use crate::{
    file::{
        BatchRowIndex,
        RowIndexOutOfBounds,
    },
    record::batch::arrays::DownCastFailure,
};

macro_rules! simple_array {
    (
        $name:ident,
        $datatype:ident,
        $value:ty
    ) => {
        simple_array_dynamic!($name, $name, $datatype, $value);
    };
}

macro_rules! simple_array_dynamic {
    (
        $wrapper:ident,
        $arrow:ident,
        $datatype:ident,
        $value:ty
    ) => {
        #[doc = concat!("Thin wrapper around Arrow's [", stringify!($arrow), "](array::", stringify!($arrow), ").")]
        ///
        /// This wrapper removes Arrow types from the public API while providing a
        /// consistent indexing interface shared by the crate's array wrappers.
        #[repr(transparent)]
        #[must_use]
        pub struct $wrapper(array::$arrow);

        impl $wrapper {
            #[doc = concat!("Attempts to create a new [", stringify!($wrapper), "] from an Arrow [`ArrayRef`],")]
            /// returning [`DownCastFailure`] otherwise.
            #[inline]
            pub fn try_from_array_ref(
                array_ref: &ArrayRef,
            ) -> Result<Self, DownCastFailure> {
                let expected = DataType::$datatype;

                if array_ref.data_type() != &expected {
                    return Err(DownCastFailure {
                        actual: array_ref.data_type().clone(),
                        expected,
                    });
                }

                Ok(Self::from_raw_parts(array_ref.to_data().into()))
            }

            #[doc = concat!("Creates a new wrapper around an Arrow [", stringify!($arrow), "](array::", stringify!($arrow), ").")]
            #[inline]
            pub fn from_raw_parts(array: array::$arrow) -> Self {
                Self(array)
            }

            #[doc = concat!("Consumes the wrapper and returns the underlying Arrow [", stringify!($arrow), "](array::", stringify!($arrow), ").")]
            #[inline]
            #[must_use]
            pub fn to_raw_parts(self) -> array::$arrow {
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
            pub fn index(
                &self,
                index: BatchRowIndex,
            ) -> Result<Option<$value>, RowIndexOutOfBounds> {
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
    };
}

simple_array!(BooleanArray,Boolean,bool);
simple_array!(Float32Array,Float32,f32);
simple_array!(Int16Array,Int16,i16);
simple_array_dynamic!(SignalBinaryArray, LargeBinaryArray, LargeBinary, &[u8]);
simple_array!(StringArray,Utf8,&str);
simple_array!(UInt8Array,UInt8,u8);
simple_array!(UInt16Array,UInt16,u16);
simple_array!(UInt32Array,UInt32,u32);
simple_array!(UInt64Array,UInt64,u64);