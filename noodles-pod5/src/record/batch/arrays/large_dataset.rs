// standard

// third party
use arrow::array::ArrayRef;
// local
use crate::{
    file::{
        BatchRowIndex,
        RowIndexOutOfBounds,
    },
    record::batch::{
        types::LargeData,
        arrays::{
            SignalBinaryArray, 
            SignalListArray,
            DownCastFailure,
        },
    },
};

/// Abstracts over the actual form of the signal data column, be it compressed or not.
#[must_use]
pub enum LargeDataSet {
    /// The compressed, unreadable form of the data.
    VBZ(SignalBinaryArray),

    /// The raw form of the data, directly readable.
    Raw(SignalListArray),
}

impl LargeDataSet {
    /// Attempts to create a new [`LargeDataSet`] from an Arrow [`ArrayRef`]
    /// based on the compression state of the table, returning [`DownCastFailure`]
    /// if it fails.
    #[inline]
    fn try_from_array_ref(array_ref: &ArrayRef) -> Result<Self, DownCastFailure> {
        let inner = SignalBinaryArray::try_from_array_ref(array_ref);
        if let Ok(inner) = inner{
            return Ok(LargeDataSet::VBZ(inner))
        }
        match  SignalListArray::try_from_array_ref(array_ref) {
            Ok(inner) => Ok(LargeDataSet::Raw(inner)),
            Err(down_cast_failure) => Err(down_cast_failure),
        }
    }


    /// Returns the value stored at the given batch row.
    ///
    /// Returns `Ok(None)` if the value is null.
    ///
    /// # Errors
    ///
    /// Returns an error if the row index is out of bounds.
    pub fn index(&self, index: BatchRowIndex) -> Result<Option<LargeData>, RowIndexOutOfBounds> {
        match self {
            LargeDataSet::VBZ(array) => {
                match array.index(index) {
                    Ok(Some(data)) => Ok(Some(LargeData::VBZ(data))),
                    Ok(None) => Ok(None),
                    Err(e) => Err(e),
                }
            }
            LargeDataSet::Raw(array) => {
                match array.index(index) {
                    Ok(Some(data)) => Ok(Some(LargeData::Raw(data))),
                    Ok(None) => Ok(None),
                    Err(e) => Err(e),
                }
            }
        }
    }
}