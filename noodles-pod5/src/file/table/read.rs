// standard

// third party

// local
use crate::io::reader::{Local, Atomic};

mod internal {
    pub(crate) mod backend {
        use crate::io::reader::ConcurrencyMode;

        pub trait ReadTable<M: ConcurrencyMode>{}

        pub struct ReadTableCore<M: ConcurrencyMode>{
            concurrency_mode:M,
        }
    }
}

#[cfg(feature = "backend")]
pub(crate) use internal::backend;

pub type ReadTable = internal::backend::ReadTableCore<Local>;
pub type ConcurrentReadTable = internal::backend::ReadTableCore<Atomic>;