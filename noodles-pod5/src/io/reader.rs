// standard

// third party

// local

mod internal {
    // standard
    use std::{
        rc::Rc,
        sync::Arc,
        ops::Deref,
    };
    //third party
    use arc_swap::Guard;
    // local
    use crate::record::batch::Batch;

    mod sealed {
        pub trait ConcurrencySeal {}
        pub trait SharedSeal {}

        pub trait HandleSeal {}
    }

    /// A trait used to restrict generic parameters to proper refs.
    /// A ref gives read access to the underlying data `T` to any number of readers.
    /// It is implemented for the [`Arc`], the [`Rc`] and the [`ArcGuard`].
    trait SharedRef<T>: Deref<Target = T> + sealed::SharedSeal {
        /// The owned version of the ref.
        type Owned: SharedOwner<T>;

        /// Returns the owned version of the ref.
        fn to_owned(self) -> Self::Owned;
    }

    /// A trait used to restrict generic parameters to proper owned refs.
    /// It owns the data `t` it shares with the readers.
    /// It is implemented for the [`Arc`] and the [`Rc`].
    trait SharedOwner<T>: SharedRef<T>{}

    impl<T> sealed::SharedSeal for Rc<T> {}

    impl<T> SharedRef<T> for Rc<T> {
        type Owned = Rc<T>;

        fn to_owned(self) -> Rc<T> {
            self
        }
    }

    impl<T> SharedOwner<T> for Rc<T>{}

    impl<T> sealed::SharedSeal for Arc<T> {}

    impl<T> SharedRef<T> for Arc<T> {
        type Owned = Arc<T>;

        fn to_owned(self) -> Arc<T> {
            self
        }
    }

    impl<T> SharedOwner<T> for Arc<T>{}

    /// A temporary, thread-safe local view of a [`Batch`].
    ///
    /// `ArcGuard` wraps an [`arc_swap`](arc_swap)`::`[`Guard<Option<Arc<Batch>>>`](Guard).
    /// It dereferences directly to the underlying `Batch` in the `Option<Arc<Batch>>`,
    /// providing a guaranteed [`Some`] variant for read operations.
    ///
    /// Unlike a raw `Guard<Option<Arc<T>>>`, this type assumes the value is never empty.
    /// It will panic on creation or dereference if the underlying pointer contains [`None`].
    pub struct ArcGuard<B: Batch>(Guard<Option<Arc<B>>>);

    impl<B: Batch> Deref for ArcGuard<B> {
        type Target = B;

        fn deref(&self) -> &B {
            match &*self.0 {
                Some(arc) => arc,
                None => panic!("Invariant broken: Guard was None!"),
            }
        }
    }

    impl<B: Batch> sealed::SharedSeal for ArcGuard<B> {}

    impl<B: Batch> SharedRef<B> for ArcGuard<B> {
        type Owned = Arc<B>;
        fn to_owned(self) -> Arc<B> {
            match Guard::into_inner(self.0) {
                Some(arc) => arc,
                None => panic!("Invariant broken: Guard was None!"),
            }
        }
    }

    pub trait BatchHandle: sealed::HandleSeal {}

    pub trait ConcurrencyMode: sealed::ConcurrencySeal {}

    pub struct Local;

    impl sealed::ConcurrencySeal for Local {}

    impl ConcurrencyMode for Local {}

    pub struct Atomic;

    impl sealed::ConcurrencySeal for Atomic {}

    impl ConcurrencyMode for Atomic {}
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