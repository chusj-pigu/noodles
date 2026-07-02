// standard

// third party

// local
use crate::io::reader::{Local, Atomic};

mod internal {
    pub(crate) mod backend {
        use crate::io::reader::ConcurrencyMode;

        pub trait SignalTable<M: ConcurrencyMode>{}

        pub struct SignalTableCore<M: ConcurrencyMode>{
            concurrency_mode:M,
        }
    }
}

#[cfg(feature = "backend")]
pub(crate) use internal::backend;

pub type SignalTable = internal::backend::SignalTableCore<Local>;
pub type ConcurrentSignalTable = internal::backend::SignalTableCore<Atomic>;