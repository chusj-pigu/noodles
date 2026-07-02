// standard

// third party

// local
use crate::io::reader::{Local, Atomic};

mod internal {
    pub(crate) mod backend {
        use crate::io::reader::ConcurrencyMode;

        pub trait Pod5<M: ConcurrencyMode>{}

        pub struct Pod5Core<M: ConcurrencyMode>{
            concurrency_mode:M,
        }
    }
}

#[cfg(feature = "backend")]
pub(crate) use internal::backend;

pub type Pod5 = internal::backend::Pod5Core<Local>;
pub type ConcurrentPod5 = internal::backend::Pod5Core<Atomic>;