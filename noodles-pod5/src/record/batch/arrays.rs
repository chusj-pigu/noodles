// standard
use std::{
    error::Error,
    fmt::{
        Display,
        Formatter,
        Result,
    },
};
// third party
use arrow::datatypes::DataType;
// local


mod epoch_millis_array;
mod index_list_array;
mod large_dataset;
mod signal_list_array;
mod simple_arrays;
mod slice_map_array;
mod string_dictionary;
mod uuid_array;

pub use {
    epoch_millis_array::*,
    index_list_array::*,
    large_dataset::*,
    signal_list_array::*,
    simple_arrays::*,
    slice_map_array::*,
    string_dictionary::*,
    uuid_array::*,
};

#[derive(Debug, Clone, PartialEq, Eq)]
/// An error where the user passes in a row index which is too big.
pub struct DownCastFailure {
    /// The Arrow DataType you tried to cast to
    pub expected: DataType,
    /// The actual Arrow DataType of the array
    pub actual: DataType,
}

impl Display for DownCastFailure {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Downcast failed: expected array of type {:?}, but got {:?}.",
            self.expected, self.actual
        )
    }
}

impl Error for DownCastFailure {}