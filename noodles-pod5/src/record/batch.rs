// standard

// third party
use arrow::array::RecordBatch;
// local
use crate::file::{FileRowIndex, RowCount};

mod run_info;
mod read;
mod signal;

pub use self::{
    run_info::{
        RunInfoBatch,
        ConcurrentRunInfoBatch,
    },
    read::{
        ReadBatch,
        ConcurrentReadBatch,
        ListArrayWrapper,
    },
    signal::{
        SignalBatch,
        ConcurrentSignalBatch,
    },
};


pub(crate) mod internal {
    pub mod backend {
        pub use self::super::super::{
            run_info::internal::*,
            read::internal::*,
            signal::internal::*,
        };
    }
}

#[cfg(feature = "backend")]
pub use self::internal::backend;

mod sealed {
    pub trait Seal {}
}

/// The `Batch` is the fundamental unit of work for processing.
/// It represents a sequential collection of [`Records`](crate::record::common::Record)
/// grouped together to optimize throughput and memory usage during bulk operations.
///
/// This trait is sealed for [`RunInfoBatch`], [`ReadBatch`] and [`SignalBatch`].
pub trait Batch: sealed::Seal {
    /// Returns a new `Batch`.
    fn new(record_batch: RecordBatch, start_row: FileRowIndex, num_rows: RowCount) -> Self;

    /// Returns the [`Arrow`](arrow) [`RecordBatch`] the `Batch` wraps around.
    fn as_record_batch(&self) -> &RecordBatch;

    /// Returns `true` if the [`RowIndex`](FileRowIndex) is within this `Batch`.
    fn contains(&self, global_row: FileRowIndex) -> bool;
}

impl sealed::Seal for RunInfoBatch {}

impl sealed::Seal for ReadBatch {}

impl sealed::Seal for SignalBatch {}