// standard

// third party

// local
use crate::io::reader::{Local, Atomic};

pub(crate) mod internal {
    use crate::io::reader::ConcurrencyMode;

    pub trait SignalBuffer<M: ConcurrencyMode>{}

    pub struct SignalBufferCore<M: ConcurrencyMode>{
        concurrency_mode:M,
    }
}

pub type SignalBuffer = internal::SignalBufferCore<Local>;
pub type ConcurrentSignalBuffer = internal::SignalBufferCore<Atomic>;