// standard

// third party

// local
use crate::{
    io::reader::{
        Local,
        Atomic,
    },
};

pub(crate) mod internal {
    // standard
    use std::marker::PhantomData;
    // third party
    use arrow::array::{RecordBatch};
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
            batch::{
                Batch,
                arrays::{
                    UuidArray,
                    IndexListArray,
                    UInt16Array,
                    UInt8Array,
                    StringDictionary,
                    Float32Array,
                    UInt32Array,
                    UInt64Array,
                    BooleanArray,
                },
                columns::ReadBatchColumns,
            },
        }
    };
    use crate::record::batch::{BatchError, InvalidColumnSet};

    /// Defines the operations of a `ReadBatch`.
    pub trait ReadBatch<M: ConcurrencyMode>: Batch {
        /// The [`ReadRecord`] implementation returned by [`single_row()`](Self::single_row).
        type ReadRecord: ReadRecord<M::LocalAccess>;

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

        // --- Core Column ---
        /// Returns the stored down-casted `read_id` column.
        fn read_id_column(&self) -> Result<&UuidArray, BatchError>;

        // --- Core Column (Recoverable) ---
        /// Returns the stored down-casted signal [`FileRowIndexes`](FileRowIndex) column.
        fn signal_column(&self) -> Result<&IndexListArray, BatchError>;

        // --- Core Column ---
        /// Returns the stored down-casted sequencing channel identifier column.
        fn channel_column(&self) -> Result<&UInt16Array, BatchError>;

        // --- Core Column ---
        /// Returns the stored down-casted pore well identifier column.
        fn well_column(&self) -> Result<&UInt8Array, BatchError>;

        /// Returns the stored down-casted pore type classification column.
        fn pore_type_column(&self) -> Result<&StringDictionary, BatchError>;

        // --- Core Column ---
        /// Returns the stored down-casted calibration offset (applied to signal values) column.
        fn calibration_offset_column(&self) -> Result<&Float32Array, BatchError>;

        // --- Core Column ---
        /// Returns the stored down-casted calibration scale (applied to signal values) column.
        fn calibration_scale_column(&self) -> Result<&Float32Array, BatchError>;

        // --- Core Column ---
        /// Returns the stored down-casted sequential read number column.
        fn read_number_column(&self) -> Result<&UInt32Array, BatchError>;

        // --- Core Column ---
        /// Returns the stored down-casted start position (in samples) column.
        fn start_column(&self) -> Result<&UInt64Array, BatchError>;

        /// Returns the stored down-casted median signal value (before each read begins) column.
        fn median_before_column(&self) -> Result<&Float32Array, BatchError>;

        /// Returns the stored down-casted tracked scaling factor (applied during basecalling) column.
        fn tracked_scaling_scale_column(&self) -> Result<&Float32Array, BatchError>;

        /// Returns the stored down-casted tracked scaling shift (applied during basecalling) column.
        fn tracked_scaling_shift_column(&self) -> Result<&Float32Array, BatchError>;

        /// Returns the stored down-casted predicted scaling factor column.
        fn predicted_scaling_scale_column(&self) -> Result<&Float32Array, BatchError>;

        /// Returns the stored down-casted predicted scaling shift column.
        fn predicted_scaling_shift_column(&self) -> Result<&Float32Array, BatchError>;

        /// Returns the stored down-casted column of the number of reads since the last mux change.
        fn num_reads_since_mux_change_column(&self) -> Result<&UInt32Array, BatchError>;

        /// Returns the stored down-casted column of the time elapsed since the last mux change.
        fn time_since_mux_change_column(&self) -> Result<&Float32Array, BatchError>;

        /// Returns the stored down-casted column of the number of MinKNOW events associated with each read.
        fn num_minknow_events_column(&self) -> Result<&UInt64Array, BatchError>;

        /// Returns the stored down-casted column of the end reason classification for each read.
        fn end_reason_column(&self) -> Result<&StringDictionary, BatchError>;

        /// Returns the stored down-casted column of whether the end reason was forced.
        fn end_reason_forced_column(&self) -> Result<&BooleanArray, BatchError>;

        // --- Core Column ---
        /// Returns the stored down-casted run information identifier column.
        fn run_info_column(&self) -> Result<&StringDictionary, BatchError>;

        // --- Core Column (Recoverable) ---
        /// Returns the stored down-casted column of the number of samples in each read.
        fn num_samples_column(&self) -> Result<&UInt64Array, BatchError>;

        /// Returns the stored down-casted open pore level measurement column.
        fn open_pore_level_column(&self) -> Result<&Float32Array, BatchError>;
    }

    /// The core implementation of the [`ReadBatch`], it owns a source
    /// [`RecordBatch`], but also the downcasted column refferences
    /// to avoid downcasting repeatidly.
    pub struct ReadBatchCore<M: ConcurrencyMode>{
        record: RecordBatch,
        columns: Result<ReadBatchColumns, InvalidColumnSet>,
        phantom: PhantomData<M>,
    }

    impl<M: ConcurrencyMode> ReadBatchCore<M> {
        pub(crate) fn new(source: RecordBatch) -> Self {
            let data = source.columns();
            let columns = ReadBatchColumns::new(data);
            match columns {
                Some(columns) => {
                    Self {
                        record: source,
                        columns: Ok(columns),
                        phantom: PhantomData,
                    }
                },
                None => {
                    let set = data.to_vec();
                    Self {
                        record: source,
                        columns: Err(InvalidColumnSet(set)),
                        phantom: PhantomData,
                    }
                }
            }
        }
    }

    impl<M: ConcurrencyMode> Batch for ReadBatchCore<M> {
        fn as_record_batch(&self) -> &RecordBatch {
            &self.record
        }
    }
}

#[cfg_attr(feature = "backend", doc = "The single-threaded variant of the [`ReadBatch`](internal::ReadBatchCore) generic.")]
#[cfg_attr(not(feature = "backend"), doc = "The single-threaded variant of a `ReadBatch`.")]
pub type ReadBatch = internal::ReadBatchCore<Local>;

#[cfg_attr(feature = "backend", doc = "The concurrent variant of the [`ReadBatch`](internal::ReadBatchCore) generic.")]
#[cfg_attr(not(feature = "backend"), doc = "The concurrent variant of a `ReadBatch`.")]
pub type ConcurrentReadBatch = internal::ReadBatchCore<Atomic>;