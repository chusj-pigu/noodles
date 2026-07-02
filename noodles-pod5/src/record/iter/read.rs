// standard

// third party

// local
use crate::io::reader::{Local, Atomic};

mod internal {
    pub(crate) mod backend {
        use crate::io::reader::ConcurrencyMode;

        pub trait ReadIter<M: ConcurrencyMode>{}

        pub struct ReadIterCore<M: ConcurrencyMode>{
            concurrency_mode:M,
        }
    }
}

#[cfg(feature = "backend")]
pub(crate) use internal::backend;

pub type ReadIter = internal::backend::ReadIterCore<Local>;
pub type ConcurrentReadIter = internal::backend::ReadIterCore<Atomic>;