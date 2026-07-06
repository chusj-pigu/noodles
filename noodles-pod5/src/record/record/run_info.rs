// standard

// third party

// local
use crate::io::reader::{Local, Atomic};

pub(crate) mod internal {
    use crate::io::reader::ConcurrencyMode;

    pub trait RunInfoRecord<M: ConcurrencyMode>{}

    pub struct RunInfoRecordCore<M: ConcurrencyMode>{
        concurrency_mode:M, 
    }
}

pub type RunInfoRecord = internal::RunInfoRecordCore<Local>;
pub type ConcurrentRunInfoRecord = internal::RunInfoRecordCore<Atomic>;