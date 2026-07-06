// standard

// third party

// local
use crate::io::reader::{Local, Atomic};

pub(crate) mod internal {
    use crate::io::reader::ConcurrencyMode;

    pub trait RunInfoIter<M: ConcurrencyMode>{}

    pub struct RunInfoIterCore<M: ConcurrencyMode>{
        concurrency_mode:M,
    }
}

pub type RunInfoIter = internal::RunInfoIterCore<Local>;
pub type ConcurrentRunInfoIter = internal::RunInfoIterCore<Atomic>;