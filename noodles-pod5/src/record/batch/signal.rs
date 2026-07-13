// standard

// third party
use arrow::array::{
    LargeBinaryArray,
    LargeListArray,
};
use crate::file::{FileRowIndex, RowIndexOutOfBounds};
// local
use crate::io::reader::{Local, Atomic};

pub(crate) mod internal {
    // standard

    // third party

    // local
    use crate::{
        io::reader::ConcurrencyMode,
        file::{
            FileRowIndex,
            RowIndexOutOfBounds,
        },
        record::{
            internal::backend::SignalRecord,
            batch::{
                Batch,
                arrays::*,
            },
        }
    };

    /// Defines the operations of a `SignalBatch`.
    pub trait SignalBatch<M: ConcurrencyMode>: Batch {
        /// The [`SignalRecord`] implementation returned by [`single_row()`](Self::single_row).
        type SignalRecord: SignalRecord<M::LocalAccess>;

        /// Returns a single [`SignalRecord`](Self::SignalRecord) by
        /// its [`FileRowIndex`].
        ///
        /// # Errors
        ///
        /// Returns [`RowIndexOutOfBounds`] if `global_row` points outside the
        /// current batch's boundaries.
        fn single_row(&self, global_row: FileRowIndex) -> Result<Self::SignalRecord, RowIndexOutOfBounds>;

        // --- Core Column ---
        /// Returns the stored down-casted `read_id` column.
        fn read_id_column(&self) -> &UuidArray;

        // --- Core Column ---
        /// Returns the stored down-casted `read_id` column.
        fn signal_column(&self) -> &LargeDataSet;

        // --- Core Column (Recoverable) ---
        /// Returns the stored down-casted `read_id` column.
        fn sample_column(&self) -> &UInt32Array;
    }

    pub struct SignalBatchCore<M: ConcurrencyMode>{
        concurrency_mode:M,
    }
}

#[cfg_attr(feature = "backend", doc = "The single-threaded variant of the [`SignalBatch`](internal::SignalBatchCore) generic.")]
#[cfg_attr(not(feature = "backend"), doc = "The single-threaded variant of a `SignalBatch`.")]
pub type SignalBatch = internal::SignalBatchCore<Local>;

#[cfg_attr(feature = "backend", doc = "The concurrent variant of the [`SignalBatch`](internal::SignalBatchCore) generic.")]
#[cfg_attr(not(feature = "backend"), doc = "The concurrent variant of a `SignalBatch`.")]
pub type ConcurrentSignalBatch = internal::SignalBatchCore<Atomic>;