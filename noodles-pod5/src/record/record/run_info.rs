// standard
use std::{
    error::Error,
    fmt::{
        self,
        Display,
        Formatter,
    },
};
// third party

// local
use crate::{
    file::RowIndexOutOfBounds,
    io::reader::{
        LocalBatchHandle,
        AtomicBatchHandle,
        LeasedBatchRef,
    },
};


pub(crate) mod internal {
    use std::marker::PhantomData;
    use crate::{
        file::{
            FileRowIndex,
            RowIndexOutOfBounds
        },
        record::{
            record::run_info::{
                RunInfoError,
            },
            batch::{
                types::FlatMap,
                internal::backend::SignalBatch,
            },
            Record,
        },
        io::reader::BatchAccess,
    };

    /// Defines the operations of a `RunInfoRecord`.
    pub trait RunInfoRecord<A: BatchAccess>: Record<A> {
        /// The underlying columnar dataset batch backing this specific record type.
        type ReadBatch: SignalBatch<A::ConcurrencyMode>;

        /// Returns a zero-copy record for the specific index of the batch
        fn new(batch: Self::ReadBatch, row: FileRowIndex) -> Self;

        // --- Core Column ---
        /// Returns the stored down-casted acquisition identifier.
        fn acquisition_id(&self) -> Result<&str, RunInfoError>;

        // --- Core Column ---
        /// Returns the stored down-casted acquisition start time.
        fn acquisition_start_time(&self) -> Result<&i64, RunInfoError>;

        // --- Core Column (Recoverable) ---
        /// Returns the stored down-casted maximum ADC value.
        fn adc_max(&self) -> Result<i16, RunInfoError>;

        // --- Core Column (Recoverable) ---
        /// Returns the stored down-casted minimum ADC value.
        fn adc_min(&self) -> Result<i16, RunInfoError>;

        /// Returns the stored down-casted run context tags.
        fn context_tags(&self) -> Result<Option<FlatMap>, RowIndexOutOfBounds>;

        /// Returns the stored down-casted experiment name.
        fn experiment_name(&self) -> Result<Option<&str>, RowIndexOutOfBounds>;

        // --- Core Column ---
        /// Returns the stored down-casted flow cell identifier.
        fn flow_cell_id(&self) -> Result<&str, RunInfoError>;

        // --- Core Column ---
        /// Returns the stored down-casted flow cell product code.
        fn flow_cell_product_code(&self) -> Result<&str, RunInfoError>;

        /// Returns the stored down-casted protocol name.
        fn protocol_name(&self) -> Result<Option<&str>, RowIndexOutOfBounds>;

        // --- Core Column ---
        /// Returns the stored down-casted protocol run identifier.
        fn protocol_run_id(&self) -> Result<&str, RunInfoError>;

        // --- Core Column ---
        /// Returns the stored down-casted protocol start time.
        fn protocol_start_time(&self) -> Result<i64, RunInfoError>;

        /// Returns the stored down-casted sample identifier.
        fn sample_id(&self) -> Result<Option<&str>, RowIndexOutOfBounds>;

        // --- Core Column ---
        /// Returns the stored down-casted acquisition sample rate.
        fn sample_rate(&self) -> Result<u16, RunInfoError>;

        // --- Core Column ---
        /// Returns the stored down-casted sequencing kit.
        fn sequencing_kit(&self) -> Result<&str, RunInfoError>;

        // --- Core Column ---
        /// Returns the stored down-casted sequencer position.
        fn sequencer_position(&self) -> Result<&str, RunInfoError>;

        /// Returns the stored down-casted sequencer position type.
        fn sequencer_position_type(&self) -> Result<Option<&str>, RowIndexOutOfBounds>;

        /// Returns the stored down-casted acquisition software description.
        fn software(&self) -> Result<Option<&str>, RowIndexOutOfBounds>;

        /// Returns the stored down-casted system name.
        fn system_name(&self) -> Result<Option<&str>, RowIndexOutOfBounds>;

        // --- Core Column ---
        /// Returns the stored down-casted system type.
        fn system_type(&self) -> Result<&str, RunInfoError>;

        /// Returns the stored down-casted run tracking information.
        fn tracking_id(&self) -> Result<Option<FlatMap>, RowIndexOutOfBounds>;
    }

    pub struct RunInfoRecordCore<A: BatchAccess>{
        phantom: PhantomData<A>,
    }
}

pub type RunInfoRecord = internal::RunInfoRecordCore<LocalBatchHandle>;
pub type LeasedRunInfoRecord = internal::RunInfoRecordCore<LeasedBatchRef>;
pub type ConcurrentRunInfoRecord = internal::RunInfoRecordCore<AtomicBatchHandle>;

#[derive(Debug)]
/// Error raised when a mandatory column in a `ReadBatch` contains an invalid null value.
///
/// Pod5 data layout requires these fields to be fully populated. A null value implies
/// either file corruption or a non-compliant file writer.
pub enum RunInfoColumnError {
    // --- Structural Layout Columns ---

    /// The acquisition identifier column contains an invalid null value.
    AcquisitionId,

    // --- Core Data Columns ---

    /// The acquisition start time column contains an invalid null value.
    AcquisitionStartTime,

    /// The maximum ADC column contains an invalid null value.
    AdcMax,

    /// The minimum ADC column contains an invalid null value.
    AdcMin,

    /// The flow cell identifier column contains an invalid null value.
    FlowCellId,

    /// The flow cell product code column contains an invalid null value.
    FlowCellProductCode,

    /// The protocol run identifier column contains an invalid null value.
    ProtocolRunId,

    /// The protocol start time column contains an invalid null value.
    ProtocolStartTime,

    /// The acquisition sample rate column contains an invalid null value.
    SampleRate,

    /// The sequencing kit column contains an invalid null value.
    SequencingKit,

    /// The sequencer position column contains an invalid null value.
    SequencerPosition,

    /// The system type column contains an invalid null value.
    SystemType
}

impl Display for RunInfoColumnError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let variant = match self {
            Self::AcquisitionId => "Structural",
            _ => "Core data",
        };
        let name = match self {
            Self::AcquisitionId => "acquisition_id",
            Self::AcquisitionStartTime => "acquisition_start_time",
            Self::AdcMax => "adc_max",
            Self::AdcMin => "adc_min",
            Self::FlowCellId => "flow_cell_id",
            Self::FlowCellProductCode => "flow_cell_product_code",
            Self::ProtocolRunId => "protocol_run_id",
            Self::ProtocolStartTime => "protocol_start_time",
            Self::SampleRate => "sample_rate",
            Self::SequencingKit => "sequencing_kit",
            Self::SequencerPosition => "sequencer_position",
            Self::SystemType => "system_type",
        };
        write!(f, "{} column {} cannot contain null values", variant, name)
    }
}

impl Error for RunInfoColumnError {}



#[derive(Debug)]
/// Error raised when a mandatory column in a `RunInfoBatch` contains an invalid null value.
///
/// Pod5 data layout requires these fields to be fully populated. A null value implies
/// either file corruption or a non-compliant file writer.
pub enum RunInfoError {
    /// A mandatory column contains an invalid null value.
    FieldMissing(RunInfoColumnError),
    
    /// The requested row index falls outside the valid bounds of the batch.
    RowIndexOOB(RowIndexOutOfBounds),
}

impl Display for RunInfoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::FieldMissing(e) => write!(f, "Missing field: {}", e),
            Self::RowIndexOOB(e) => write!(f, "{}", e),
        }
    }
}

impl Error for RunInfoError {}