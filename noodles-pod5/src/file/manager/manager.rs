// standard

// third party

// local
use crate::io::reader::ConcurrencyMode;

pub(crate) struct Manager<M: ConcurrencyMode>{
    concurrency_mode: M
}