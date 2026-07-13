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


mod slice_map_array;
mod index_list_array;
mod signal_list_array;
mod string_dictionnary;
pub mod uuid_array;
mod large_dataset;

pub use {
    index_list_array::*,
    signal_list_array::*,
    slice_map_array::*,
    string_dictionnary::*,
    uuid_array::*,
    large_dataset::*,
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