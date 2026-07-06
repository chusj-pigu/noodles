// standard

// third party

// local
use crate::io::reader::{Local, Atomic};

pub(crate) mod internal {     
    use crate::io::reader::ConcurrencyMode;

    pub trait SignalRecord<M: ConcurrencyMode>{}

    pub struct SignalRecordCore<M: ConcurrencyMode>{
        concurrency_mode:M,
    }
}

pub type SignalRecord = internal::SignalRecordCore<Local>;
pub type ConcurrentSignalRecord = internal::SignalRecordCore<Atomic>;