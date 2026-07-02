mod run_info;
mod read;
mod signal;

pub use self::{
    run_info::*,
    read::*,
    signal::*,
};
#[cfg(feature = "backend")]
pub mod backend {
    pub use self::super::{
        run_info::backend::*,
        read::backend::*,
        signal::backend::*,
    };
}

mod sealed {
    pub trait Seal {}
}

/// A trait used to restrict generic parameters to proper batches.
/// It is implemented for the [`RunInfoBatch`], the [`ReadBatch`] and the [`SignalBatch`].
pub trait Batch: sealed::Seal {}

impl sealed::Seal for RunInfoBatch {}

impl Batch for RunInfoBatch {}

impl sealed::Seal for ReadBatch {}

impl Batch for ReadBatch {}

impl sealed::Seal for SignalBatch {}

impl Batch for SignalBatch {}