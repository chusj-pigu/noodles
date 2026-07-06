// standard

// third party

// local
use crate::io::reader::{Local, Atomic};

pub(crate) mod internal {
    use crate::io::reader::ConcurrencyMode;

    pub trait ReadRecord<M: ConcurrencyMode>{}

    pub struct ReadRecordCore<M: ConcurrencyMode>{
        concurrency_mode:M,
    }
}
pub type ReadRecord = internal::ReadRecordCore<Local>;
pub type ConcurrentReadRecord = internal::ReadRecordCore<Atomic>;