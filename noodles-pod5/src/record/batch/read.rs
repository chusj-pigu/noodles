// standard

// third party
use arrow::array::{Array, AsArray, ListArray, UInt64Array};
// local
use crate::{
    io::reader::{
        Local,
        Atomic,
    },
    file::{
        BatchRowIndex,
        RowIndexOutOfBounds,
    },
};

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
            iter::internal::backend::ReadIter,
            internal::backend::ReadRecord,
            batch::Batch,
        }
    };

    /// Defines the operations of a `ReadBatch`.
    pub trait ReadBatch<M: ConcurrencyMode>: Batch {
        /// The [`ReadRecord`] implementation returned by [`single_row()`](Self::single_row).
        type ReadRecord: ReadRecord<M>;

        /// The [`ReadIter`] implementation returned by [`records()`](Self::records).
        type ReadIter: ReadIter<M>;

        /// Returns a single [`ReadRecord`](Self::ReadRecord) by
        /// its [`FileRowIndex`].
        ///
        /// # Errors
        ///
        /// Returns [`RowIndexOutOfBounds`] if `global_row` points outside the
        /// current batch's boundaries.
        fn single_row(&self, global_row: FileRowIndex) -> Result<Self::ReadRecord, RowIndexOutOfBounds>;

        /// Returns a variant of a [`ReadIter`](Self::ReadIter) of the records in this batch.
        fn records(&self) -> Self::ReadIter;

        // --- Core Column Accessors ---

        /// Returns the `read_id` column.
        fn read_id_column(&self) -> &FixedSizeBinaryArray;

        /// Returns the signal [`FileRowIndex`] column.
        fn signal_column(&self) -> &super::ListArrayWrapper;

        /// Returns the sequencing channel identifier column.
        fn channel_column(&self) -> &UInt16Array;

        /// Returns the pore well identifier column.
        fn well_column(&self) -> &UInt8Array;

        /// Returns the pore type classification column.
        fn pore_type_column(&self) -> &StringArray;

        // --- Calibration Metadata Columns ---

        /// Returns the calibration offset applied to signal values.
        fn calibration_offset_column(&self) -> &Float32Array;

        /// Returns the calibration scale applied to signal values.
        fn calibration_scale_column(&self) -> &Float32Array;

        // --- Tracking & Slicing Columns ---

        /// Returns the sequential read number column.
        fn read_number_column(&self) -> &UInt32Array;

        /// Returns the start position (in samples) of each read.
        fn start_column(&self) -> &UInt64Array;

        /// Returns the median signal value before each read begins.
        fn median_before_column(&self) -> &Float32Array;

        // --- Basecalling Model Scaling Columns ---

        /// Returns the tracked scaling factor applied during basecalling.
        fn tracked_scaling_scale_column(&self) -> &Float32Array;

        /// Returns the tracked scaling shift applied during basecalling.
        fn tracked_scaling_shift_column(&self) -> &Float32Array;

        /// Returns the predicted scaling factor.
        fn predicted_scaling_scale_column(&self) -> &Float32Array;

        /// Returns the predicted scaling shift.
        fn predicted_scaling_shift_column(&self) -> &Float32Array;

        // --- Mux (Multiplexer) Execution Columns ---

        /// Returns the number of reads since the last mux change.
        fn num_reads_since_mux_change_column(&self) -> &UInt32Array;

        /// Returns the time elapsed since the last mux change.
        fn time_since_mux_change_column(&self) -> &Float32Array;

        // --- Quality & Execution State Columns ---

        /// Returns the number of MinKNOW events associated with each read.
        fn num_minknow_events_column(&self) -> &UInt64Array;

        /// Returns the end reason classification for each read.
        fn end_reason_column(&self) -> &StringArray;

        /// Returns whether the end reason was forced.
        fn end_reason_forced_column(&self) -> &BooleanArray;

        /// Returns the run information identifier column.
        fn run_info_column(&self) -> &StringArray;

        /// Returns the number of samples in each read.
        fn num_samples_column(&self) -> &UInt64Array;

        /// Returns the open pore level measurement column.
        fn open_pore_level_column(&self) -> &Float32Array;
    }

    pub struct ReadBatchCore<M: ConcurrencyMode>{
        concurrency_mode:M,
    }
}



#[repr(transparent)]
/// Wraps an [`ListArray`] and exposes it as a pseudo-indexable type.
///
/// This wrapper guarantees that the underlying list elements can be efficiently
/// accessed as slices of raw `u64` integers.
pub struct ListArrayWrapper(ListArray);

impl ListArrayWrapper {
    /// Returns a reference to the underlying [`ListArray`].
    pub fn as_list_array(&self) -> &ListArray {
        &self.0
    }

    /// Returns the nth list of `u64` from the array as a primitive slice.
    ///
    /// # Errors
    ///
    /// Returns [`RowIndexOutOfBounds`] if the provided `index` is greater than
    /// or equal to the total length of the underlying array.
    ///
    /// # Panics
    ///
    /// Panics if the underlying data array cannot be downcast to a [`UInt64Array`],
    /// or if the internal Arrow offsets are malformed.
    pub fn value(&self, index: BatchRowIndex) -> Result<&[u64], RowIndexOutOfBounds> {
        let list_array = self.as_list_array();
        let index: usize = index.into();

        if index >= list_array.len() {
            return Err(RowIndexOutOfBounds);
        }

        let data_array: &UInt64Array = list_array.values().as_primitive();
        let offsets = list_array.offsets();
        let start = offsets[index] as usize;
        let end = offsets[index + 1] as usize;
        Ok(&data_array.values()[start..end])
    }
}

#[cfg_attr(feature = "backend", doc = "The single-threaded variant of the [`ReadBatch`](internal::ReadBatchCore) generic.")]
#[cfg_attr(not(feature = "backend"), doc = "The single-threaded variant of a `ReadBatch`.")]
pub type ReadBatch = internal::ReadBatchCore<Local>;

#[cfg_attr(feature = "backend", doc = "The concurrent variant of the [`ReadBatch`](internal::ReadBatchCore) generic.")]
#[cfg_attr(not(feature = "backend"), doc = "The concurrent variant of a `ReadBatch`.")]
pub type ConcurrentReadBatch = internal::ReadBatchCore<Atomic>;

