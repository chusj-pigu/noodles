// standard

// third party

// local
use crate::io::reader::{Local, Atomic};

mod internal {
    pub(crate) mod backend {
        use crate::io::reader::ConcurrencyMode;

        pub trait SignalRecord<M: ConcurrencyMode>{}

        pub struct SignalRecordCore<M: ConcurrencyMode>{
            concurrency_mode:M,
        }
    }
}

#[cfg(feature = "backend")]
pub(crate) use internal::backend;

pub type SignalRecord = internal::backend::SignalRecordCore<Local>;
pub type ConcurrentSignalRecord = internal::backend::SignalRecordCore<Atomic>;