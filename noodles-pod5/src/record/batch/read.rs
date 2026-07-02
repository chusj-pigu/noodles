// standard

// third party

// local
use crate::io::reader::{Local, Atomic};

mod internal {
    pub(crate) mod backend {
        use crate::io::reader::ConcurrencyMode;

        pub trait ReadBatch<M: ConcurrencyMode>{}

        pub struct ReadBatchCore<M: ConcurrencyMode>{
            concurrency_mode:M,
        }
    }
}

#[cfg(feature = "backend")]
pub(crate) use internal::backend;

pub type ReadBatch = internal::backend::ReadBatchCore<Local>;
pub type ConcurrentReadBatch = internal::backend::ReadBatchCore<Atomic>;