// standard

// third party

// local
use crate::io::reader::{Local, Atomic};

mod internal {
    pub(crate) mod backend {
        use crate::io::reader::ConcurrencyMode;

        pub trait RunInfoIter<M: ConcurrencyMode>{}

        pub struct RunInfoIterCore<M: ConcurrencyMode>{
            concurrency_mode:M,
        }
    }
}

#[cfg(feature = "backend")]
pub(crate) use internal::backend;

pub type RunInfoIter = internal::backend::RunInfoIterCore<Local>;
pub type ConcurrentRunInfoIter = internal::backend::RunInfoIterCore<Atomic>;