// standard

// third party

// local
use crate::io::reader::{Local, Atomic};

pub(crate) mod internal {
    // standard

    // third party
    use arrow::{
        array::*,
        datatypes::Int32Type,
    };
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
            batch::Batch,
        }
    };

    /*
    could potentially do subdivision like this;
    // --- Acquisition Metadata ---
    acquisition_id
    acquisition_start_time
    experiment_name
    sample_rate

    // --- Hardware Metadata ---
    adc_max
    adc_min
    flow_cell_id
    flow_cell_product_code
    sequencer_position
    sequencer_position_type

    // --- Protocol Metadata ---
    protocol_name
    protocol_run_id
    protocol_start_time
    sequencing_kit
    sample_id

    // --- Software & System Metadata ---
    software
    system_name
    system_type

    // --- Compatibility Metadata ---
    context_tags
    tracking_id
    
    issue is this would reorganize the columns in a different order than their 
    internal order of appearence, which I feel would wind up more confusing than anything
    */
    /// Defines the operations of a `SignalBatch`.
    pub trait RunInfoBatch<M: ConcurrencyMode>: Batch{
        /// The [`RunInfoRecord`] implementation returned by [`single_row()`](Self::single_row).
        type RunInfoRecord: RunInfoRecord<M>;

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

        // --- Core Column Accessors ---

        /// Returns the acquisition identifier column.
        fn acquisition_id_column(&self) -> &DictionaryArray<Int32Type>;

        /// Returns the acquisition start time column.
        fn acquisition_start_time_column(&self) -> &TimestampMillisecondArray;

        /// Returns the maximum ADC value column.
        fn adc_max_column(&self) -> &Int16Array;

        /// Returns the minimum ADC value column.
        fn adc_min_column(&self) -> &Int16Array;

        /// Returns the run context tags column.
        fn context_tags_column(&self) -> &MapArray;

        /// Returns the experiment name column.
        fn experiment_name_column(&self) -> &DictionaryArray<Int32Type>;

        /// Returns the flow cell identifier column.
        fn flow_cell_id_column(&self) -> &DictionaryArray<Int32Type>;

        /// Returns the flow cell product code column.
        fn flow_cell_product_code_column(&self) -> &DictionaryArray<Int32Type>;

        /// Returns the protocol name column.
        fn protocol_name_column(&self) -> &DictionaryArray<Int32Type>;

        /// Returns the protocol run identifier column.
        fn protocol_run_id_column(&self) -> &DictionaryArray<Int32Type>;

        /// Returns the protocol start time column.
        fn protocol_start_time_column(&self) -> &TimestampMillisecondArray;

        /// Returns the sample identifier column.
        fn sample_id_column(&self) -> &DictionaryArray<Int32Type>;

        /// Returns the acquisition sample rate column.
        fn sample_rate_column(&self) -> &UInt16Array;

        /// Returns the sequencing kit column.
        fn sequencing_kit_column(&self) -> &DictionaryArray<Int32Type>;

        /// Returns the sequencer position column.
        fn sequencer_position_column(&self) -> &DictionaryArray<Int32Type>;

        /// Returns the sequencer position type column.
        fn sequencer_position_type_column(&self) -> &DictionaryArray<Int32Type>;

        /// Returns the acquisition software description column.
        fn software_column(&self) -> &DictionaryArray<Int32Type>;

        /// Returns the system name column.
        fn system_name_column(&self) -> &DictionaryArray<Int32Type>;

        /// Returns the system type column.
        fn system_type_column(&self) -> &DictionaryArray<Int32Type>;

        /// Returns the run tracking information column.
        fn tracking_id_column(&self) -> &MapArray;
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