// standard

// third party

// local
use crate::io::reader::{Local, Atomic};

mod internal {
    pub(crate) mod backend {
        use crate::io::reader::ConcurrencyMode;

        pub trait RunInfoBatch<M: ConcurrencyMode>{}

        pub struct RunInfoBatchCore<M: ConcurrencyMode>{
            concurrency_mode:M,
        }
    }
}

#[cfg(feature = "backend")]
pub(crate) use internal::backend;

pub type RunInfoBatch = internal::backend::RunInfoBatchCore<Local>;
pub type ConcurrentRunInfoBatch = internal::backend::RunInfoBatchCore<Atomic>;