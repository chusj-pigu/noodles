// standard

// third party

// local
use crate::{
    io::reader::BatchAccess,
    record::batch::Batch,
};

/// The `Record` is the fundamental logical unit for reasoning.
/// It represents a single row within a table of the [`POD5 file`](crate::file::Pod5).
///
/// This trait defines the conversion contract of `Records`, to turn them into owned variants.
/// This trait is implemented for the [`RunInfoRecord`](crate::record::RunInfoRecord), the
/// [`ReadRecord`](crate::record::ReadRecord), and the [`SignalRecord`](crate::record::SignalRecord).
pub trait Record<B: BatchAccess>
{
    /// The type of [`Batch`] the record references.
    type Batch: Batch;

    /// The longer lived version of the [`BatchAccess`] pointer of the record.
    type OwnedRecord: Record<B::BatchHandle<Self::Batch>>;

    /// Converts a record from local access to a longer-lived owned version.
    fn to_owned(self) -> Self::OwnedRecord;
}
