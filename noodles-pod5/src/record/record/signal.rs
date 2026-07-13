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
             record::signal::{
                 SignalError,
             },
             batch::{
                 types::Uuid,
                 internal::backend::SignalBatch,
             },
             Record,
         },
         io::reader::BatchAccess,
     };
     use crate::record::batch::types::LargeData;

     /// Defines the operations of a `SignalRecord`.
     pub trait SignalRecord<A: BatchAccess>: Record<A> {
         /// The underlying columnar dataset batch backing this specific record type.
         type ReadBatch: SignalBatch<A::ConcurrencyMode>;

         /// Returns a zero-copy record for the specific index of the batch
         fn new(batch: Self::ReadBatch, row: FileRowIndex) -> Self;

         /// The unique alphanumeric string identifying the specific read event.
         fn read_id(&self) -> Result<Uuid, SignalError>;

         /// The row indexes to the associated signal records.
         fn signal(&self) -> Result<LargeData, SignalError>;

         /// Number of signal items present within the `Signal`.
         fn samples(&self) -> Result<u32, SignalError>;
     }

     pub struct SignalRecordCore<A: BatchAccess> {
         phantom: PhantomData<A>,
     }
 }

pub type SignalRecord = internal::SignalRecordCore<LocalBatchHandle>;
pub type LeasedSignalRecord = internal::SignalRecordCore<LeasedBatchRef>;
pub type ConcurrentSignalRecord = internal::SignalRecordCore<AtomicBatchHandle>;

#[derive(Debug)] 
/// Error raised when a mandatory column in a `ReadBatch` contains an invalid null value. 
///  
/// Pod5 data layout requires these fields to be fully populated. A null value implies 
/// either file corruption or a non-compliant file writer. 
pub enum SignalColumnError {
    // --- Structural Layout Columns ---

    /// The read identifier column contains an invalid null value.
    ReadId,

    // --- Core Data Columns ---

    /// The signal column contains an invalid null value.
    Signal,

    /// The samples column contains an invalid null value.
    Samples,
}

impl Display for SignalColumnError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let variant = match self {
            Self::ReadId => "Structural",
            _ => "Core data",
        };
        let name = match self {
            Self::ReadId => "read_id",
            Self::Signal => "signal",
            Self::Samples => "samples",
        };
        write!(f, "{} column {} cannot contain null values", variant, name)
    }
}

impl Error for SignalColumnError {}


#[derive(Debug)]
/// Error raised when a mandatory column in a `ReadBatch` contains an invalid null value.
///
/// Pod5 data layout requires these fields to be fully populated. A null value implies
/// either file corruption or a non-compliant file writer.
pub enum SignalError {
    /// A mandatory column contains an invalid null value.
    FieldMissing(SignalColumnError),

    /// The requested row index falls outside the valid bounds of the batch.
    RowIndexOOB(RowIndexOutOfBounds),
}

impl Display for SignalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::FieldMissing(e) => write!(f, "Missing field: {}", e),
            Self::RowIndexOOB(e) => write!(f, "{}", e),
        }
    }
}

impl Error for SignalError {}