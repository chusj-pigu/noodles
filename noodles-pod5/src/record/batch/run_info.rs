// standard

// third party

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
            iter::internal::backend::RunInfoIter,
            internal::backend::RunInfoRecord,
            batch::{
                Batch,
                arrays::*,
            },
        }
    };
    
    /// Defines the operations of a `SignalBatch`.
    pub trait RunInfoBatch<M: ConcurrencyMode>: Batch{
        /// The [`RunInfoRecord`] implementation returned by [`single_row()`](Self::single_row).
        type RunInfoRecord: RunInfoRecord<M::LocalAccess>;

        /// The [`RunInfoIter`] implementation returned by [`records()`](Self::records).
        type RunInfoIter: RunInfoIter<M>;

        /// Returns a single [`RunInfoRecord`](Self::RunInfoRecord) by
        /// its [`FileRowIndex`].
        ///
        /// # Errors
        ///
        /// Returns [`RowIndexOutOfBounds`] if `global_row` points outside the
        /// current batch's boundaries.
        fn single_row(&self, global_row: FileRowIndex) -> Result<Self::RunInfoRecord, RowIndexOutOfBounds>;

        /// Returns a variant of a [`RunInfoIter`](Self::RunInfoIter) of the records in this batch.
        fn records(&self) -> Self::RunInfoIter;

        // --- Core Column ---
        /// Returns the stored down-casted acquisition identifier column.
        fn acquisition_id_column(&self) -> &StringArray;

        // --- Core Column ---
        /// Returns the stored down-casted acquisition start time column.
        fn acquisition_start_time_column(&self) -> &EpochMillisArray;

        // --- Core Column (Recoverable) ---
        /// Returns the stored down-casted maximum ADC value column.
        fn adc_max_column(&self) -> &Int16Array;

        // --- Core Column (Recoverable) ---
        /// Returns the stored down-casted minimum ADC value column.
        fn adc_min_column(&self) -> &Int16Array;

        /// Returns the stored down-casted run context tags column.
        fn context_tags_column(&self) -> &SliceMapArray;

        /// Returns the stored down-casted experiment name column.
        fn experiment_name_column(&self) -> &StringDictionary;

        // --- Core Column ---
        /// Returns the stored down-casted flow cell identifier column.
        fn flow_cell_id_column(&self) -> &StringDictionary;

        // --- Core Column ---
        /// Returns the stored down-casted flow cell product code column.
        fn flow_cell_product_code_column(&self) -> &StringDictionary;

        /// Returns the stored down-casted protocol name column.
        fn protocol_name_column(&self) -> &StringDictionary;

        // --- Core Column ---
        /// Returns the stored down-casted protocol run identifier column.
        fn protocol_run_id_column(&self) -> &StringDictionary;

        // --- Core Column ---
        /// Returns the stored down-casted protocol start time column.
        fn protocol_start_time_column(&self) -> &EpochMillisArray;

        /// Returns the stored down-casted sample identifier column.
        fn sample_id_column(&self) -> &StringDictionary;

        // --- Core Column ---
        /// Returns the stored down-casted acquisition sample rate column.
        fn sample_rate_column(&self) -> &UInt16Array;

        // --- Core Column ---
        /// Returns the stored down-casted sequencing kit column.
        fn sequencing_kit_column(&self) -> &StringDictionary;

        // --- Core Column ---
        /// Returns the stored down-casted sequencer position column.
        fn sequencer_position_column(&self) -> &StringDictionary;

        /// Returns the stored down-casted sequencer position type column.
        fn sequencer_position_type_column(&self) -> &StringDictionary;

        /// Returns the stored down-casted acquisition software description column.
        fn software_column(&self) -> &StringDictionary;

        /// Returns the stored down-casted system name column.
        fn system_name_column(&self) -> &StringDictionary;

        // --- Core Column ---
        /// Returns the stored down-casted system type column.
        fn system_type_column(&self) -> &StringDictionary;

        /// Returns the stored down-casted run tracking information column.
        fn tracking_id_column(&self) -> &SliceMapArray;
    }

    pub struct RunInfoBatchCore<M: ConcurrencyMode>{
        concurrency_mode:M,
    }
}

#[cfg_attr(feature = "backend", doc = "The single-threaded variant of the [`RunInfoBatch`](internal::RunInfoBatchCore) generic.")]
#[cfg_attr(not(feature = "backend"), doc = "The single-threaded variant of a `RunInfoBatch`.")]
pub type RunInfoBatch = internal::RunInfoBatchCore<Local>;

#[cfg_attr(feature = "backend", doc = "The concurrent variant of the [`RunInfoBatch`](internal::RunInfoBatchCore) generic.")]
#[cfg_attr(not(feature = "backend"), doc = "The concurrent variant of a `RunInfoBatch`.")]
pub type ConcurrentRunInfoBatch = internal::RunInfoBatchCore<Atomic>;