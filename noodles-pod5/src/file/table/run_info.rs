// standard

// third party

// local
use crate::io::reader::{Local, Atomic};

mod internal {
    pub(crate) mod backend {
        use crate::io::reader::ConcurrencyMode;

        pub trait RunInfoTable<M: ConcurrencyMode>{}

        pub struct RunInfoTableCore<M: ConcurrencyMode>{
            concurrency_mode:M,
        }
    }
}

#[cfg(feature = "backend")]
pub(crate) use internal::backend;

pub type RunInfoTable = internal::backend::RunInfoTableCore<Local>;
pub type ConcurrentRunInfoTable = internal::backend::RunInfoTableCore<Atomic>;