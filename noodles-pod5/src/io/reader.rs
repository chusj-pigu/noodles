// standard

// third party

// local

mod internal {
    mod sealed {
        pub trait ConcurrencySeal {}
        pub trait SharedSeal {}

        pub trait HandleSeal {}
    }
    pub trait ConcurrencyMode: sealed::ConcurrencySeal {}

    pub struct Local;

    impl sealed::ConcurrencySeal for Local {}

    impl ConcurrencyMode for Local {}

    pub struct Atomic;

    impl sealed::ConcurrencySeal for Atomic {}

    impl ConcurrencyMode for Atomic {}

    pub trait SharedRef<T>: sealed::SharedSeal {}

    pub trait SharedOwner<T>: SharedRef<T>{}

    pub trait BatchHandle: sealed::HandleSeal {}
}

#[cfg(feature = "backend")]
pub use self::internal::{
    ConcurrencyMode,
    Local,
    Atomic,
};

#[cfg(not(feature = "backend"))]
pub(crate) use self::internal::{
    ConcurrencyMode,
    Local,
    Atomic,
};

pub struct ReaderBuilder;

pub struct Reader;