// standard

// third party

// local
use crate::io::reader::{Local, Atomic};

mod internal {
    pub(crate) mod backend {
        use crate::io::reader::ConcurrencyMode;

        pub trait ReadRecord<M: ConcurrencyMode>{}

        pub struct ReadRecordCore<M: ConcurrencyMode>{
            concurrency_mode:M,
        }
    }
}

#[cfg(feature = "backend")]
pub(crate) use internal::backend;

pub type ReadRecord = internal::backend::ReadRecordCore<Local>;
pub type ConcurrentReadRecord = internal::backend::ReadRecordCore<Atomic>;