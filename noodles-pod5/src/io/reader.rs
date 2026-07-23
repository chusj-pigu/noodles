// standard

// third party

// local

mod internal {
    // standard
    use std::{rc::Rc, sync::Arc, ops::Deref, fmt};
    use std::error::Error;
    use std::fmt::{Display, Formatter};
    //third party
    use arc_swap::Guard;
    // local
    use crate::record::batch::Batch;
    use crate::file::manager::{BatchUsageTracker, Capacity, RcBatchUsageTracker, ArcBatchUsageTracker};

    mod sealed {
        pub trait ConcurrencySeal {}
        pub trait SharedSeal {}

        pub trait BatchAccessSeal {}
    }

    /// A trait used to restrict generic parameters and types to proper refs.
    /// A ref gives read access to the underlying data `T` to any number of readers.
    /// It is implemented for the [`Arc`], the [`Rc`] and the [`ArcGuard`].
    pub trait SharedAccess<M: ConcurrencyMode, T>: Deref<Target = T> + sealed::SharedSeal {}

    /// A trait used to restrict generic parameters and types to proper owned refs.
    /// It owns the data `t` it shares with the readers.
    /// It is implemented for the [`Arc`] and the [`Rc`].
    pub trait RefCounted<M: ConcurrencyMode, T>: SharedAccess<M, T>{}

    impl<T> sealed::SharedSeal for Rc<T> {}

    impl<T> SharedAccess<Local, T> for Rc<T> {}

    impl<T> RefCounted<Local, T> for Rc<T>{}

    impl<T> sealed::SharedSeal for Arc<T> {}

    impl<T> SharedAccess<Atomic, T> for Arc<T> {}

    impl<T> RefCounted<Atomic, T> for Arc<T>{}

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

    impl<B: Batch> SharedAccess<Atomic, B> for ArcGuard<B> {}

    /// A trait used to restrict generic parameters and types to proper views
    /// of [`batches`](Batch). It is implemented for the [`LocalBatchHandle`],
    /// the [`AtomicBatchHandle`], and the [`LeasedBatchRef`].
    pub trait BatchAccess: sealed::BatchAccessSeal {
        /// The [`ConcurrencyMode`] type associated with the `BatchAccess`.
        type ConcurrencyMode: ConcurrencyMode;

        /// The specific [`SharedAccess`] type associated with the `BatchAccess`.
        type SharedAccess<B: Batch>: SharedAccess<Self::ConcurrencyMode, B>;

        /// The [`BatchHandle`] associated with the `BatchAccess`.
        type BatchHandle<B: Batch>: BatchHandle<SharedAccess<B>: RefCounted<Self::ConcurrencyMode, B>, ConcurrencyMode = Self::ConcurrencyMode>;

        /// Converts the [`SharedAccess`] type into the [`RefCounted`] type of
        /// the associated [`BatchHandle`].
        fn upgrade<B: Batch>(
            shared_access: Self::SharedAccess<B>,
        ) -> <Self::BatchHandle<B> as BatchAccess>::SharedAccess<B>;
    }

    /// A trait used to restrict generic parameters and types to initial, thread-bound
    /// [`BatchAccesses`](BatchAccess) before upgrading.
    ///
    /// It is implemented for the [`LocalBatchHandle`] and the [`LeasedBatchRef`].
    pub trait LocalAccess: BatchAccess {}

    /// A trait used to restrict generic parameters and types to upgraded,
    /// owned [`BatchAccesses`](BatchAccess).
    ///
    /// It is implemented for the [`LocalBatchHandle`] and the [`AtomicBatchHandle`]
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


    #[derive(Debug)]
    /// Error raised when a HintThreshold is badly instantiated.
    pub enum HintThresholdError {
        /// The provided value exceeds the maximum allowable threshold.
        OverFlow,

        /// The provided value falls below the minimum allowable threshold.
        UnderFlow,
    }

    impl Display for HintThresholdError {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                Self::OverFlow => write!(f, "HintThreshold overflow: attempted new with threshold above max"),
                Self::UnderFlow => write!(f, "HintThreshold underflow: attempted new with threshold under min"),
            }
        }
    }

    impl Error for HintThresholdError {}

    /// Represents the number of remaining uses at which point a hint
    /// should be given to start loading the next batch.
    /// The [`Capacity`](crate::file::manager::Capacity) of trackers
    /// is compared against it by the internal
    /// [`BatchManager`](crate::file::manager::BatchManager).
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct HintThreshold(u64);
    impl HintThreshold {
        /// The largest threshold count representable by this type.
        pub const MAX: u64 = Capacity::MAX;

        /// The smallest threshold count representable by this type.
        pub const MIN: u64 = Capacity::MIN;

        /// Returns `Ok(Capacity)`, or
        /// [`Err(HintThresholdError)`](HintThresholdError)
        /// if `threshold` exceeds [`MAX`](Capacity::MAX) or is `0`.
        pub fn new(threshold: u64) -> Result<Self, HintThresholdError> {
            if threshold > Self::MAX {
                return Err(HintThresholdError::OverFlow)
            }
            if threshold < Self::MIN {
                return Err(HintThresholdError::UnderFlow)
            }
            Ok(Self(threshold))
        }
    }

    impl From<HintThreshold> for u64 {
        fn from(threshold: HintThreshold) -> Self {
            threshold.0
        }
    }

    impl From<HintThreshold> for Capacity {
        fn from(threshold: HintThreshold) -> Capacity {
            // HintThreshold enforces identical bounds to Capacity at construction.
            Capacity::new(threshold.into()).unwrap()
        }
    }

    impl TryFrom<u64> for HintThreshold {
        type Error = HintThresholdError;
        fn try_from(threshold: u64) -> Result<Self, Self::Error> {
            HintThreshold::new(threshold)
        }
    }

    pub trait ConcurrencyMode: sealed::ConcurrencySeal + Sized {

        type LocalAccess: LocalAccess<ConcurrencyMode = Self>;
        type RefCounted<T>: RefCounted<Self, T>;
        type Tracker<B: Batch>: BatchUsageTracker<Self, B>;
    }

    pub struct Local;

    impl sealed::ConcurrencySeal for Local {}

    impl ConcurrencyMode for Local {
        type LocalAccess = LocalBatchHandle;
        type RefCounted<T> = Rc<T>;
        type Tracker<B: Batch> = RcBatchUsageTracker<B>;
    }

    pub struct Atomic;

    impl sealed::ConcurrencySeal for Atomic {}

    impl ConcurrencyMode for Atomic {
        type LocalAccess = LeasedBatchRef;
        type RefCounted<T> = Arc<T>;
        type Tracker<B: Batch> = ArcBatchUsageTracker<B>;
    }



}

use std::ops::Add;
#[cfg(feature = "backend")]
pub use self::internal::{
    ConcurrencyMode,
    Local,
    Atomic,
    BatchAccess,
    BatchHandle,
    LocalAccess,
    LocalBatchHandle,
    AtomicBatchHandle,
    LeasedBatchRef,
    ArcGuard,
};

#[cfg(not(feature = "backend"))]
pub(crate) use self::internal::{
    ConcurrencyMode,
    Local,
    Atomic,
    BatchAccess,
    BatchHandle,
    LocalAccess,
    LocalBatchHandle,
    AtomicBatchHandle,
    LeasedBatchRef,
    ArcGuard,
};



#[must_use]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
/// A page-aligned byte offset.
///
/// Values of this type are guaranteed to lie on a page boundary.
pub struct PageOffset(usize);

impl PageOffset {

    /// Creates a `PageOffset` without validating the provided offset.
    ///
    /// The caller must ensure that `offset` is aligned to the page size used by the
    /// corresponding memory mapping.
    fn new(offset: usize) -> Self {
        Self(offset)
    }

    /// Returns the underlying byte offset.
    pub fn into_inner(&self) -> usize {
        self.0
    }
}

/// Failed to determine the operating system's memory page size.
pub struct PageSizeError(String);

#[must_use]
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
/// The operating system's virtual memory page size.
///
/// `PageSize` provides page-aware arithmetic used throughout the crate,
/// including offset alignment, page counting, and page iteration.
pub struct PageSize(usize);

impl PageSize {
    /// Creates a `PageSize` without querying the operating system.
    ///
    /// The provided page size should match the actual page size used by the
    /// operating system.
    ///
    /// Supplying an incorrect value does not cause immediate undefined behavior,
    /// but it invalidates the assumptions made by the paging algorithms and may
    /// result in incorrect alignment, ineffective prefetching, or degraded
    /// performance.
    ///
    /// # Panics
    /// Panics if the provided `size` is not a power of two.
    pub fn custom(page_size: usize) -> Self {
        assert!(
            page_size.is_power_of_two(),
            "page size ({page_size}) must be a power of two"
        );
        assert!(
            page_size != 0,
            "page size ({page_size}) must be a non-zero value"
        );
        Self(page_size)
    }

    /// Returns the greatest page-aligned offset less than or equal to `offset`.
    pub fn align_down(&self, offset: usize) -> PageOffset {
        PageOffset::new(offset - (offset % self.0))
    }

    /// Returns the smallest page-aligned offset greater than or equal to `offset`.
    pub fn align_up(&self, offset: usize) -> PageOffset {
        let alignment = offset % self.0;
        if alignment == 0 {
            PageOffset::new(offset)
        } else {
            PageOffset::new(offset + (self.0 - alignment))
        }
    }

    /// Returns an iterator over consecutive page-aligned offsets as well as the final offset.
    ///
    /// The iterator begins at `start` and yields `page_count` page offsets.
    ///
    /// # Panics
    ///
    /// The total range covered by the pages must not overflow usize.
    pub fn iter_pages(&self, start: PageOffset, page_count: usize) -> (impl Iterator<Item = PageOffset>, PageOffset) {
        let start = start.into_inner();
        let page_size = self.0;
        let offset = page_count.checked_mul(page_size)
            .expect(&format!("page offset overflowed ({} * {})", page_count, page_size));
        let end = start.checked_add(offset)
            .expect(&format!("page range overflowed ({} + {})", start, offset));
        let iter = (0..page_count).map(move |page_count| {
            let offset = start + page_count * page_size;
            PageOffset::new(offset)
        });
        (iter, PageOffset::new(end))
    }

    /// Returns the number of bytes occupied by `page_count` pages.
    pub fn page_length(&self, page_count: usize) -> usize {
        self.0 * page_count
    }

    /// Returns the number of pages intersected by the byte range.
    ///
    /// Any page containing at least one byte of the range is included in the count.
    pub fn touched_page_count(&self, start: usize, length: usize) -> usize {
        if length == 0 {
            return 0;
        }
        let raw_end = start.checked_add(length)
            .expect(&format!("page range overflowed ({} + {})", start, length));
        let start = self.align_down(start);
        let end = self.align_up(raw_end);
        let span = end.into_inner() - start.into_inner();
        span >> self.0.trailing_zeros()
    }

    /// Returns the number of pages that may safely be reclaimed.
    ///
    /// Unlike [`touched_page_count`], a trailing partially covered page is excluded,
    /// since reclaiming it could discard bytes beyond the requested range.
    pub fn reclaimable_pages_count(&self, start: usize, length: usize) -> usize {
        if length == 0 {
            return 0;
        }
        let raw_end = start.checked_add(length)
            .expect(&format!("page range overflowed ({} + {})", start, length));
        let start = self.align_down(start);
        let end = self.align_down(raw_end);
        let span = end.into_inner() - start.into_inner();
        span >> self.0.trailing_zeros()
    }

    /// Returns whether `offset` lies on a page boundary.
    pub fn is_aligned(&self, offset: usize) -> bool {
        offset % self.0 == 0
    }
}

#[cfg(unix)]
use libc::{
    sysconf,
    _SC_PAGESIZE,
};

#[cfg(unix)]
impl PageSize {

    /// Returns the operating system's memory page size on `Unix`
    /// systems, or PageSizeError if an issue is encountered.
    pub fn initialize() -> Result<Self, PageSizeError> {
        // Safety:
        // Page size is a core value which should be available
        // on all Unix systems, and if it isn't, the error is handled.
        let raw_page_size = unsafe {
            sysconf(_SC_PAGESIZE)
        };
        if raw_page_size < 0 {
            Err(PageSizeError(format!("unknown error: {}", raw_page_size)))
        } else if raw_page_size == 0 {
            Err(PageSizeError("Page size found is invalid (0)".to_owned()))
        } else if !(raw_page_size as usize).is_power_of_two() {
            Err(PageSizeError("Page size found is invalid (not 2^n)".to_owned()))
        } else {
            Ok(Self(raw_page_size as usize))
        }
    }
}

#[cfg(windows)]
use windows_sys::Win32::System::SystemInformation::{
    GetSystemInfo,
    SYSTEM_INFO,
};

#[cfg(windows)]
impl PageSize {
    /// Returns the operating system's memory page size on `Windows`
    /// systems. This method does **not** ever return PageSizeError.
    pub fn initialize() -> Result<Self, PageSizeError> {
        let mut system_info = MaybeUninit::<SYSTEM_INFO>::uninit();
        // Safety:
        // - `system_info.as_mut_ptr()` provides a valid, aligned, writable pointer to
        //   sufficiently allocated memory (`MaybeUninit<SYSTEM_INFO>`).
        // - `GetSystemInfo` is guaranteed by Windows to populate this memory completely,
        //   making `assume_init()` safe to call immediately afterward.
        let page_size = unsafe {
            GetSystemInfo(system_info.as_mut_ptr());
            system_info.assume_init().dwPageSize as usize
        };
        if page_size == 0 {
            Err(PageSizeError("Page size found is invalid (0)".to_owned()))
        } else if !(page_size).is_power_of_two() {
            Err(PageSizeError("Page size found is invalid (not 2^n)".to_owned()))
        }
        Ok(Self(page_size))
    }
}

#[cfg(not(any(unix, windows)))]
impl PageSize {
    /// Returns PageSizeError on non-`Unix` and non-`Windows` systems.
    /// This method does **not** ever return PageSize.
    /// [`custom()`](PageSize::custom) must be used instead.
    pub fn initialize() -> Result<Self, PageSizeError> {
        // On non-Unix/Windows systems, the unsafe custom page size **must** be used.
        Err(PageSizeError("Page size cannot be found, must use custom size.".to_owned()))
    }
}

pub struct ReaderBuilder;

pub struct Reader;