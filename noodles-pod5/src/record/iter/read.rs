// standard

// third party

// local
use crate::io::reader::{Local, Atomic};

pub(crate) mod  internal {
    use crate::io::reader::ConcurrencyMode;

    pub trait ReadIter<M: ConcurrencyMode>{}

    pub struct ReadIterCore<M: ConcurrencyMode>{
        concurrency_mode:M,
    }
}

pub type ReadIter = internal::ReadIterCore<Local>;
pub type ConcurrentReadIter = internal::ReadIterCore<Atomic>;