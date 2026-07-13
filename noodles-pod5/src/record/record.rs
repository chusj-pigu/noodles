// standard

// third party

use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
// local
use crate::{
    io::reader::BatchAccess,
    record::batch::Batch,
};
use crate::io::reader::ConcurrencyMode;

mod run_info;
mod read;
mod signal;


pub use self::{
    run_info::{
        RunInfoRecord,
        ConcurrentRunInfoRecord,
    },
    read::{
        ReadRecord,
        ConcurrentReadRecord,
        ReadColumnError,
    },
    signal::{
        SignalRecord,
        ConcurrentSignalRecord,
    },
};

pub(crate) mod internal {
    pub mod backend {
        pub use self::super::super::{
            run_info::internal::*,
            read::internal::{
                ReadRecord,
                ReadRecordCore,
            },
            signal::internal::*,
        };
    }
}

#[cfg(feature = "backend")]
pub use self::internal::backend;


/// The `Record` is the fundamental logical unit for reasoning.
/// It represents a single row within a table of the [`POD5 file`](crate::file::Pod5).
///
/// This trait defines the conversion contract of `Records`, to turn them into owned variants.
/// This trait is implemented for the [`RunInfoRecord`](RunInfoRecord), the
/// [`ReadRecord`](ReadRecord), and the [`SignalRecord`](SignalRecord).
pub trait Record<A: BatchAccess>
{
    /// The type of [`Batch`] the record references.
    type Batch: Batch;

    /// The longer lived version of the [`BatchAccess`] pointer of the record.
    type OwnedRecord: Record<A::BatchHandle<Self::Batch>>;

    /// Converts a record from local access to a longer-lived owned version.
    fn to_owned(self) -> Self::OwnedRecord;
}