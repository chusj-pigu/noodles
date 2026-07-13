// standard

// third party

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
        },
    },
};

/// Abstracts over the actual form of the signal data column, be it compressed or not.
pub enum LargeDataSet {
    /// The raw form of the data, directly readable.
    Raw(SignalListArray),

    /// The compressed, unreadable form of the data.
    VBZ(SignalBinaryArray),
}

impl LargeDataSet {
    /// Returns the value stored at the given batch row.
    ///
    /// Returns `Ok(None)` if the value is null.
    ///
    /// # Errors
    ///
    /// Returns an error if the row index is out of bounds.
    pub fn index(&'_ self, index: BatchRowIndex) -> Result<Option<LargeData<'_>>, RowIndexOutOfBounds> {
        match self {
            LargeDataSet::Raw(array)
            => {
                match array.index(index) {
                    Ok(Some(data)) => Ok(Some(LargeData::Raw(data))),
                    Ok(None) => Ok(None),
                    Err(e) => Err(e),
                }
            }
            LargeDataSet::VBZ(array)
            => {
                match array.index(index) {
                    Ok(Some(data)) => Ok(Some(LargeData::VBZ(data))),
                    Ok(None) => Ok(None),
                    Err(e) => Err(e),
                }
            }
        }
    }
}