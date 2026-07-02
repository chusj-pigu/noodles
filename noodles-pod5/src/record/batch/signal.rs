// standard

// third party

// local
use crate::io::reader::{Local, Atomic};

mod internal {
    pub(crate) mod backend {
        use crate::io::reader::ConcurrencyMode;

        pub trait SignalBatch<M: ConcurrencyMode>{}

        pub struct SignalBatchCore<M: ConcurrencyMode>{
            concurrency_mode:M,
        }
    }
}

#[cfg(feature = "backend")]
pub(crate) use internal::backend;

pub type SignalBatch = internal::backend::SignalBatchCore<Local>;
pub type ConcurrentSignalBatch = internal::backend::SignalBatchCore<Atomic>;