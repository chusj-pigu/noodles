// standard

// third party

// local
use crate::io::reader::{Local, Atomic};

mod internal {
    pub(crate) mod backend {
        use crate::io::reader::ConcurrencyMode;

        pub trait SignalBuffer<M: ConcurrencyMode>{}

        pub struct SignalBufferCore<M: ConcurrencyMode>{
            concurrency_mode:M,
        }
    }
}

#[cfg(feature = "backend")]
pub(crate) use internal::backend;

pub type SignalBuffer = internal::backend::SignalBufferCore<Local>;
pub type ConcurrentSignalBuffer = internal::backend::SignalBufferCore<Atomic>;