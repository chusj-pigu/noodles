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

        pub trait BatchAccessSeal {}
    }

    /// A trait used to restrict generic parameters and types to proper refs.
    /// A ref gives read access to the underlying data `T` to any number of readers.
    /// It is implemented for the [`Arc`], the [`Rc`] and the [`ArcGuard`].
    pub trait SharedAccess<T>: Deref<Target = T> + sealed::SharedSeal {}

    /// A trait used to restrict generic parameters and types to proper owned refs.
    /// It owns the data `t` it shares with the readers.
    /// It is implemented for the [`Arc`] and the [`Rc`].
    pub trait RefCounted<T>: SharedAccess<T>{}

    impl<T> sealed::SharedSeal for Rc<T> {}

    impl<T> SharedAccess<T> for Rc<T> {}

    impl<T> RefCounted<T> for Rc<T>{}

    impl<T> sealed::SharedSeal for Arc<T> {}

    impl<T> SharedAccess<T> for Arc<T> {}

    impl<T> RefCounted<T> for Arc<T>{}

    /// A temporary, thread-safe local view of a [`Batch`].
    ///
    /// `ArcGuard` wraps an [`arc_swap`](arc_swap)`::`[`Guard<Option<Arc<Batch>>>`](Guard).
    /// It dereferences directly to the underlying `Batch` in the `Option<Arc<Batch>>`,
    /// providing a guaranteed [`Some`] variant for read operations.
    ///
    /// Unlike a raw lock-free guard, this type enforces a design invariant where
    /// the underlying container is never empty.
    ///
    /// # Panics
    ///
    /// Functions and trait implementations on this type will panic if the underlying
    /// pointer unexpectedly evaluates to [`None`].
    pub struct ArcGuard<B: Batch>(Guard<Option<Arc<B>>>);

    impl<B: Batch> ArcGuard<B> {
        /// Creates a new guarded view.
        ///
        /// # Panics
        ///
        /// Panics if the provided guard contains [`None`].
        pub fn new(guard: Guard<Option<Arc<B>>>) -> Self {
            match &*guard {
                Some(_) => ArcGuard(guard),
                None => panic!("Invariant broken: Guard was None!"),
            }
        }

        /// Converts the leased guard into an owned reference-counted pointer.
        ///
        /// # Panics
        ///
        /// Panics if the underlying guard contains [`None`].
        pub fn to_owned(self) -> Arc<B> {
            match Guard::into_inner(self.0) {
                Some(arc) => arc,
                None => panic!("Invariant broken: Guard was None!"),
            }
        }
    }

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

    impl<B: Batch> SharedAccess<B> for ArcGuard<B> {}

    /// A trait used to restrict generic parameters and types to proper views
    /// of [`batches`](Batch). It is implemented for the [`LocalBatchHandle`],
    /// the [`AtomicBatchHandle`], and the [`LeasedBatchRef`].
    pub trait BatchAccess: sealed::BatchAccessSeal {
        /// The [`ConcurrencyMode`] type associated with the `BatchAccess`.
        type ConcurrencyMode: ConcurrencyMode;

        /// The specific [`SharedAccess`] type associated with the `BatchAccess`.
        type SharedAccess<B: Batch>: SharedAccess<B>;

        /// The [`BatchHandle`] associated with the `BatchAccess`.
        type BatchHandle<B: Batch>: BatchHandle<SharedAccess<B>: RefCounted<B>, ConcurrencyMode = Self::ConcurrencyMode>;

        /// Converts the [`SharedAccess`] type into the [`RefCounted`] type of
        /// the associated [`BatchHandle`].
        fn upgrade<B: Batch>(
            shared_access: Self::SharedAccess<B>,
        ) -> <Self::BatchHandle<B> as BatchAccess>::SharedAccess<B>;
    }

    /// A trait used to restrict generic parameters and types to initial, thread-bound
    /// [`BatchAccesses`](BatchAccess) before upgrading.
    /// It is implemented for the [`LocalBatchHandle`] and the [`LeasedBatchRef`].
    pub trait LocalAccess: BatchAccess {}

    /// A trait used to restrict generic parameters and types to upgraded,
    /// owned [`BatchAccesses`](BatchAccess).
    pub trait BatchHandle: BatchAccess {}

    /// A marker configuration for single-threaded, owned batch access.
    /// It lets records use [`Rc`] pointers to share batches.
    ///
    /// It implements [`LocalAccess`] and [`BatchHandle`], meaning it represents both the
    /// initial access type and its own upgraded handle.
    pub struct LocalBatchHandle;

    impl sealed::BatchAccessSeal for LocalBatchHandle {}

    impl BatchAccess for LocalBatchHandle {
        type ConcurrencyMode = Local;
        type SharedAccess<B: Batch> = Rc<B>;
        type BatchHandle<B: Batch> = Self;
        fn upgrade<B: Batch>(shared_access: Rc<B>) -> Rc<B> {
            shared_access
        }
    }

    impl BatchHandle for LocalBatchHandle {}

    impl LocalAccess for LocalBatchHandle {}

    /// A marker configuration for thread-safe, owned batch access.
    /// It lets records use [`Arc`] pointers to share batches.
    ///
    /// It implements [`BatchHandle`], serving as the target upgrade path for temporary
    /// or leased access types like [`LeasedBatchRef`].
    pub struct AtomicBatchHandle;

    impl sealed::BatchAccessSeal for AtomicBatchHandle {}

    impl BatchAccess for AtomicBatchHandle {
        type ConcurrencyMode = Atomic;
        type SharedAccess<B: Batch> = Arc<B>;
        type BatchHandle<B: Batch> = Self;

        fn upgrade<B: Batch>(shared_access: Arc<B>) -> Arc<B> {
            shared_access
        }
    }

    impl BatchHandle for AtomicBatchHandle {}

    /// A marker configuration for thread-safe, leased batch access.
    /// It lets records use lock-free read [`Guards`](ArcGuard) to share batches.
    ///
    /// It implements [`LocalAccess`] but *not* [`BatchHandle`], forcing an explicit
    /// [`upgrade`](BatchAccess::upgrade) path to an [`Arc`] if ownership needs to be extended.
    pub struct LeasedBatchRef;

    impl sealed::BatchAccessSeal for LeasedBatchRef {}

    impl BatchAccess for LeasedBatchRef {
        type ConcurrencyMode = Atomic;
        type SharedAccess<B: Batch> = ArcGuard<B>;
        type BatchHandle<B: Batch> = AtomicBatchHandle;

        fn upgrade<B: Batch>(shared_access: ArcGuard<B>) -> Arc<B> {
            shared_access.to_owned()
        }
    }

    impl LocalAccess for LeasedBatchRef {}

    pub trait ConcurrencyMode: sealed::ConcurrencySeal {
        //restriction may be removable, will need to go back to it during implmentation phase
        type LocalAccess: LocalAccess<ConcurrencyMode = Self>;
    }

    pub struct Local;

    impl sealed::ConcurrencySeal for Local {}

    impl ConcurrencyMode for Local {
        type LocalAccess = LocalBatchHandle;
    }

    pub struct Atomic;

    impl sealed::ConcurrencySeal for Atomic {}

    impl ConcurrencyMode for Atomic {
        type LocalAccess = LeasedBatchRef;
    }
}

#[cfg(feature = "backend")]
pub use self::internal::{
    ConcurrencyMode,
    Local,
    Atomic,
    BatchAccess,
    LocalBatchHandle,
    AtomicBatchHandle,
    LeasedBatchRef,
};

#[cfg(not(feature = "backend"))]
pub(crate) use self::internal::{
    ConcurrencyMode,
    Local,
    Atomic,
    BatchAccess,
    LocalBatchHandle,
    AtomicBatchHandle,
    LeasedBatchRef,
};

pub struct ReaderBuilder;

pub struct Reader;