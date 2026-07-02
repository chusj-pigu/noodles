// standard

// third party

// local
use crate::record::batch::Batch;

pub(crate) trait BatchTracker<B: Batch>{}

pub(crate) struct LocalTracker<B: Batch>{
    batch: B,
}

pub(crate) struct AtomicTracker<B: Batch>{
    batch: B,
}