// standard

// third party
use arrow::array::{
    LargeBinaryArray,
    LargeListArray,
};
// local
use crate::io::reader::{Local, Atomic};

pub(crate) mod internal {
    // standard

    // third party
    use arrow::array::*;
    // local
    use crate::{
        io::reader::ConcurrencyMode,
        file::{
            FileRowIndex,
            RowIndexOutOfBounds,
        },
        record::{
            iter::internal::backend::SignalBuffer,
            internal::backend::SignalRecord,
            batch::Batch,
        }
    };

    /// Defines the operations of a `SignalBatch`.
    pub trait SignalBatch<M: ConcurrencyMode>: Batch {
        /// The [`SignalRecord`] implementation returned by [`single_row()`](Self::single_row).
        type SignalRecord: SignalRecord<M>;

        /// The [`SignalBuffer`] implementation returned by [`records()`](Self::records).
        type SignalBuffer: SignalBuffer<M>;

        /// Returns a single [`SignalRecord`](Self::SignalRecord) by
        /// its [`FileRowIndex`].
        ///
        /// # Errors
        ///
        /// Returns [`RowIndexOutOfBounds`] if `global_row` points outside the
        /// current batch's boundaries.
        fn single_row(&self, global_row: FileRowIndex) -> Result<Self::SignalRecord, RowIndexOutOfBounds>;

        /// Returns a variant of a [`SignalBuffer`](Self::SignalBuffer) of the records in this batch.
        fn records(&self) -> Self::SignalBuffer;

        // --- Core Column Accessors ---

        /// Returns the `read_id` column.
        fn read_id_column(&self) -> &FixedSizeBinaryArray;

        /// Returns the `read_id` column.
        fn signal_column(&self) -> &super::LargeDataSet;

        /// Returns the `read_id` column.
        fn sample_column(&self) -> &UInt32Array;
    }

    pub struct SignalBatchCore<M: ConcurrencyMode>{
        concurrency_mode:M,
    }
}

/// Abstracts over the actual form of the signal data, be it compressed or not.
pub enum LargeDataSet{
    Raw(LargeListArray),
    VBZ(LargeBinaryArray),
}

#[cfg_attr(feature = "backend", doc = "The single-threaded variant of the [`SignalBatch`](internal::SignalBatchCore) generic.")]
#[cfg_attr(not(feature = "backend"), doc = "The single-threaded variant of a `SignalBatch`.")]
pub type SignalBatch = internal::SignalBatchCore<Local>;

#[cfg_attr(feature = "backend", doc = "The concurrent variant of the [`SignalBatch`](internal::SignalBatchCore) generic.")]
#[cfg_attr(not(feature = "backend"), doc = "The concurrent variant of a `SignalBatch`.")]
pub type ConcurrentSignalBatch = internal::SignalBatchCore<Atomic>;