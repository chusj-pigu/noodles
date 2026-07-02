// standard

// third party

// local
use crate::io::reader::{Local, Atomic};

mod internal {
    pub(crate) mod backend {
        use crate::io::reader::ConcurrencyMode;

        pub trait RunInfoRecord<M: ConcurrencyMode>{}

        pub struct RunInfoRecordCore<M: ConcurrencyMode>{
            concurrency_mode:M,
        }
    }
}

#[cfg(feature = "backend")]
pub(crate) use internal::backend;

pub type RunInfoRecord = internal::backend::RunInfoRecordCore<Local>;
pub type ConcurrentRunInfoRecord = internal::backend::RunInfoRecordCore<Atomic>;