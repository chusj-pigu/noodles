// standard

// local
use crate::{
    file::{FileRowIndex, RowCount},
    io::reader::ConcurrencyMode
};
// third party
use arrow::array::RecordBatch;

mod run_info;
mod read;
mod signal;
pub mod arrays;
pub mod types;

pub use self::{
    read::{
        ConcurrentReadBatch,
        ReadBatch,
    },
    run_info::{
        ConcurrentRunInfoBatch,
        RunInfoBatch,
    },
    signal::{
        ConcurrentSignalBatch,
        SignalBatch,
    },
};


pub(crate) mod internal {
    pub mod backend {
        pub use self::super::super::{
            read::internal::*,
            run_info::internal::*,
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
#[cfg_attr(feature = "backend", doc = "\n\nThis trait is sealed for [`RunInfoBatchCore`](backend::RunInfoBatchCore), [`ReadBatchCore`](backend::ReadBatchCore) and [`SignalBatchCore`](backend::SignalBatchCore).")]
#[cfg_attr(not(feature = "backend"), doc = "\n\nThis trait is sealed for [`RunInfoBatch`]/[`ConcurrentRunInfoBatch`], [`ReadBatch`]/[`ConcurrentReadBatch`] and [`SignalBatch`]/[`ConcurrentSignalBatch`].")]
pub trait Batch: sealed::Seal {
    /// Returns a new `Batch`.
    fn new(record_batch: RecordBatch, start_row: FileRowIndex, num_rows: RowCount) -> Self;

    /// Returns the [`Arrow`](arrow) [`RecordBatch`] the `Batch` wraps around.
    fn as_record_batch(&self) -> &RecordBatch;

    /// Returns `true` if the [`RowIndex`](FileRowIndex) is within this `Batch`.
    fn contains(&self, global_row: FileRowIndex) -> bool;
}

impl<M: ConcurrencyMode> sealed::Seal for internal::backend::RunInfoBatchCore<M> {}

impl<M: ConcurrencyMode> sealed::Seal for internal::backend::ReadBatchCore<M> {}

impl<M: ConcurrencyMode> sealed::Seal for internal::backend::SignalBatchCore<M> {}


/*
columns requirement, X:for required, O for required but technically recoverable.

see: The list belows shows the columns which cannot be null, via X, but also shows the ones which are in theory recoverable with O.

Read columns
[X]: read_id
[O]: signal {via read_id filtration of signals (plus signal count can be used to validate after filtration search)}
[X]: channel
[X]: well
[ ]: pore_type
[X]: calibration_offset
[X]: calibration_scale
[X]: read_number
[X]: start
[ ]: median_before
[ ]: tracked_scaling_scale
[ ]: tracked_scaling_shift
[ ]: predicted_scaling_scale
[ ]: predicted_scaling_shift
[ ]: num_reads_since_mux_change
[ ]: time_since_mux_change
[ ]: num_minknow_events
[ ]: end_reason
[ ]: end_reason_forced
[X]: run_info
[O]: num_samples  {via the sum of the signal sample counts}
[ ]: open_pore_level

Run Info columns
[X]: acquisition_id
[X]: acquisition_start_time
[O]: adc_max {recoverable via system_type, since hardcoded value}
[O]: adc_min {recoverable via system_type, since hardcoded value}
[ ]: context_tags
[ ]: experiment_name
[X]: flow_cell_id
[X]: flow_cell_product_code
[ ]: protocol_name
[X]: protocol_run_id
[X]: protocol_start_time
[ ]: sample_id
[X]: sample_rate
[X]: sequencing_kit
[X]: sequencer_position
[ ]: sequencer_position_type
[ ]: software
[ ]: system_name
[X]: system_type
[ ]: tracking_id

Signal columns
[X]: read_id
[X]: signal
[O]: samples {via the length of the decompressed signal array(can be decompressed on a max-length array, aka 102400)}
    */