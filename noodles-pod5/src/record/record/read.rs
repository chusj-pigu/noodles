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
            RowIndexOutOfBounds,
        },
        record::{
            record::read::{
                ReadError,
            },
            batch::{
                types::Uuid,
                internal::backend::ReadBatch,
            },
            iter::internal::backend::SignalBuffer,
            Record,
        },
        io::reader::BatchAccess,
    };

    /// Defines the operations of a `ReadRecord`.
    pub trait ReadRecord<A: BatchAccess>: Record<A> {
        /// The [`SignalBuffer`] implementation returned by [`records()`](Self::records).
        type SignalBuffer: SignalBuffer<A::ConcurrencyMode>;

        /// The underlying columnar dataset batch backing this specific record type.
        type ReadBatch: ReadBatch<A::ConcurrencyMode>;

        /// Returns a zero-copy record for the specific index of the batch.
        fn new(batch: Self::ReadBatch, row: FileRowIndex) -> Self;

        /// Returns a variant of a [`SignalBuffer`](Self::SignalBuffer) of the records in this batch.
        fn signals() -> Self::SignalBuffer;

        /// The unique alphanumeric string identifying the specific read event.
        fn read_id(&self) -> Result<Uuid, ReadError>;

        /// The row indexes to the associated signal records.
        fn signal_row_indexes(&self) -> Result<&[u64], ReadError>;

        /// The hardware sequencer channel identifier.
        fn channel(&self) -> Result<u16, ReadError>;

        /// The hardware pore well location identifier.
        fn well(&self) -> Result<u8, ReadError>;

        /// The programmatic or state classification of the pore.
        fn pore_type(&self) -> Result<Option<&str>, RowIndexOutOfBounds>;

        /// Digitisation offset scalar applied to normalize raw signal integers.
        fn calibration_offset(&self) -> Result<f32, ReadError>;

        /// Scaling range factor applied to normalize raw signal integers.
        fn calibration_scale(&self) -> Result<f32, ReadError>;

        /// Chronological execution index number of this read sequence.
        fn read_number(&self) -> Result<u32, ReadError>;

        /// Absolute starting point (in samples) relative to the stream beginning.
        fn start(&self) -> Result<u64, ReadError>;

        /// Contextual baseline median signal sequence captured prior to read activity.
        fn median_before(&self) -> Result<Option<f32>,RowIndexOutOfBounds>;

        /// Dynamic scale variance adaptation tracking used by basecallers.
        fn tracked_scaling_scale(&self) -> Result<Option<f32>,RowIndexOutOfBounds>;

        /// Dynamic local shift drift adaptation tracking used by basecallers.
        fn tracked_scaling_shift(&self) -> Result<Option<f32>,RowIndexOutOfBounds>;

        /// Mathematically predicted scale properties for mapping algorithms.
        fn predicted_scaling_scale(&self) -> Result<Option<f32>,RowIndexOutOfBounds>;

        /// Mathematically predicted offset shift properties for mapping algorithms.
        fn predicted_scaling_shift(&self) -> Result<Option<f32>,RowIndexOutOfBounds>;

        /// Tally counts of distinct reads completed since the latest physical mux change.
        fn num_reads_since_mux_change(&self) -> Result<Option<u32>,RowIndexOutOfBounds>;

        /// Exact running time elapsed since the physical mux layout transitioned.
        fn time_since_mux_change(&self) -> Result<Option<f32>,RowIndexOutOfBounds>;

        /// Total count of discrete internal MinKNOW capture cycle notifications.
        fn num_minknow_events(&self) -> Result<Option<u64>,RowIndexOutOfBounds>;

        /// Reason description detailing why the signal acquisition phase halted.
        fn end_reason(&self) -> Result<Option<&str>,RowIndexOutOfBounds>;

        /// Indicates if data capture termination was forcefully overridden by system policy.
        fn end_reason_forced(&self) -> Result<Option<bool>,RowIndexOutOfBounds>;

        /// Unique key referencing parent context details in global metadata arrays.
        fn run_info(&self) -> Result<&str, ReadError>;

        /// Grand total size bounds of signal items present within the `Read`.
        fn num_samples(&self) -> Result<u64, ReadError>;

        /// Metric score recording stable non-blocked open channel flow levels.
        fn open_pore_level(&self) -> Result<Option<f32>,RowIndexOutOfBounds>;
    }

    pub struct ReadRecordCore<A: BatchAccess>{
        phantom: PhantomData<A>,
    }
}

pub type ReadRecord = internal::ReadRecordCore<LocalBatchHandle>;
pub type LeasedReadRecord = internal::ReadRecordCore<LeasedBatchRef>;
pub type ConcurrentReadRecord = internal::ReadRecordCore<AtomicBatchHandle>;

#[derive(Debug)]
/// Error raised when a mandatory column in a `ReadBatch` contains an invalid null value.
///
/// Pod5 data layout requires these fields to be fully populated. A null value implies
/// either file corruption or a non-compliant file writer.
pub enum ReadColumnError {
    // --- Structural Layout Columns ---

    /// The read identifier column contains an invalid null value.
    ReadId,


    /// The row indexes column contains an invalid null value.
    Signal,

    /// The run_info identifier column contains an invalid null value.
    RunInfo,

    // --- Core Data Columns ---

    /// The channel identifier column contains an invalid null value.
    Channel,

    /// The pore well identifier column contains an invalid null value.
    Well,

    /// The calibration offset column contains an invalid null value.
    CalibrationOffset,

    /// The calibration scale column contains an invalid null value.
    CalibrationScale,

    /// The samples before read start column contains an invalid null value.
    Start,

    /// The number of samples column contains an invalid null value.
    NumSamples,
}

impl Display for ReadColumnError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let variant = match self {
            Self::ReadId|Self::Signal|Self::RunInfo => "Structural",
            _ => "Core data",
        };
        let name = match self {
            Self::ReadId => "read_id",
            Self::Signal => "signal",
            Self::Channel => "channel",
            Self::Well => "well",
            Self::CalibrationOffset => "calibration_offset",
            Self::CalibrationScale => "calibration_scale",
            Self::Start => "start",
            Self::RunInfo => "run_info",
            Self::NumSamples => "num_samples",
        };
        write!(f, "{} column {} cannot contain null values", variant, name)
    }
}

impl Error for ReadColumnError {}



#[derive(Debug)]
/// Error raised when a mandatory column in a `ReadBatch` contains an invalid null value.
///
/// Pod5 data layout requires these fields to be fully populated. A null value implies
/// either file corruption or a non-compliant file writer.
pub enum ReadError {
    /// A mandatory column contains an invalid null value.
    FieldMissing(ReadColumnError),

    /// The requested row index falls outside the valid bounds of the batch.
    RowIndexOOB(RowIndexOutOfBounds),
}

impl Display for ReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::FieldMissing(e) => write!(f, "Missing field: {}", e),
            Self::RowIndexOOB(e) => write!(f, "{}", e),
        }
    }
}

impl Error for ReadError {}