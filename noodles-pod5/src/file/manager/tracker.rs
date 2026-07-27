use crate::io::reader::ArcGuard;
use crate::io::reader::Atomic;
use std::sync::atomic::{AtomicU64, Ordering};
use std::cell::Cell;
// standard
use std::fmt::Display;
use std::rc::Rc;
use std::sync::Arc;
use arc_swap::ArcSwapOption;
// third party

// local
use crate::record::batch::Batch;
use crate::io::reader::{BatchAccess, ConcurrencyMode, Local};

/// The expected number of times a batch will be acquired.
///
/// This value is computed before iteration begins and is
/// used to initialize [`BatchUsageTrackers`](BatchUsageTracker).
// While a u32 would be sufficient for the count,
// this allows for the value to be a single word,
// and let's the value carry bit flags for the concurrent tracker.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Capacity(u64);
impl Capacity {
    /// The largest usage count representable by this type.
    pub const MAX: u64 = u64::MAX >> 3;

    /// The smallest usage count representable by this type.
    pub const MIN: u64 = 1u64;

    /// Bit mask for the load bit.
    pub const IS_LOADED: u64 = 1 << 63;

    /// Bit mask for the unload bit.
    pub const WAS_UNLOADED: u64 = 1 << 62;

    /// Bit mask for the lock bit.
    pub const IS_LOCKED: u64 = 1 << 61;

    /// Bit mask for the count bits.
    const COUNT_MASK: u64 = Self::MAX;

    /// Returns `Some(Capacity)`, or `None` if `capacity`
    /// exceeds [`Self::MAX`] or is `0`.
    pub fn new(capacity: u64) -> Option<Self> {
        if capacity > Self::MAX {
            return None
        }
        if capacity < Self::MIN {
            return None;
        }
        Some(Self(capacity))
    }

    /// Returns whether the usage was succesfully logged.
    pub fn load(mut self) -> Self {
        self.0 |= Self::IS_LOADED;
        self
    }

    /// Returns whether the usage was succesfully logged.
    pub fn unload(mut self) -> Self {
        self.0 |= Self::WAS_UNLOADED;
        self
    }

    /// Returns whether the usage was succesfully logged.
    pub fn take(mut self) -> Self {
        if self.is_exhausted() {
            return self
        }
        self.0 = self.0.strict_sub(1);
        self
    }

    /// Returns the count of the capacity.
    pub fn count(&self) -> u64 {
        self.0 & Self::COUNT_MASK
    }

    /// Returns wether the data is currently loaded or not.
    pub fn is_loaded(&self) -> bool {
        self.0 & Self::IS_LOADED != 0
    }

    /// Returns wheter the data was ever unloaded or not.
    pub fn was_unloaded(&self) -> bool {
        self.0 & Self::WAS_UNLOADED != 0
    }

    /// Returns whether the lock bit is set or not.
    pub fn is_locked(&self) -> bool {
        self.0 & Self::IS_LOCKED != 0
    }

    /// Returns whether the capacity is exhausted.
    pub fn is_exhausted(&self) -> bool {
        self.count() <= 0
    }
}

impl From<Capacity> for u64 {
    fn from(capacity: Capacity) -> Self {
        capacity.0
    }
}

impl Display for Capacity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "IS_LOADED: {}, WAS_UNLOADED: {}, IS_LOCKED: {}, usage count: {}",
               self.is_loaded(), self.was_unloaded(), self.is_loaded(), self.count()
        )
    }
}

/// A trait defining the functions of the `UsageTrackers` used by the
/// [`BatchManager`](crate::file::manager::BatchManager)
/// to manage [`Batches`](Batch).
pub(crate) trait BatchUsageTracker<M: ConcurrencyMode, B: Batch>{
    /// Creates a new `BatchUsageTracker`.
    fn with_capacity(capacity: Capacity) -> Self;

    /// Sets the `BatchUsageTracker` to track the [`Batch`](Batch)
    /// [`source`](crate::io::reader::BatchHandle)
    /// and returns the
    /// [`Batch accessor`](crate::io::reader::LocalAccess)
    /// as well as the hint indictor.
    ///
    /// # Panics
    ///
    /// Panics if the `BatchUsageTracker` is already set or has previously been set.
    fn set(
        &self,
        batch: <<<M as ConcurrencyMode>::LocalAccess as BatchAccess>::BatchHandle<B> as BatchAccess>::SharedAccess<B>
    ) -> (<<M as ConcurrencyMode>::LocalAccess as BatchAccess>::SharedAccess<B>, Capacity);

    /// Returns `Some` [`accessor`](crate::io::reader::LocalAccess)
    /// as well as the hint indictor if a [`Batch`](Batch) is
    /// currently stored, decreasing the `BatchUsageTracker`'s
    /// capacity by one, or returns `None`.
    ///
    /// # Panics
    ///
    /// Panics if the `BatchUsageTracker` has already exhausted it's capacity.
    fn acquire(
        &self
    ) -> Option<(<<M as ConcurrencyMode>::LocalAccess as BatchAccess>::SharedAccess<B>, Capacity)>;
}

/// A non-thread-safe batch [`UsageTracker`](BatchUsageTracker).
///
/// This type is neither `Send` nor `Sync`.
pub(crate) struct RcBatchUsageTracker<B: Batch>{
    pointer: Cell<Option<Rc<B>>>,
    capacity: Cell<Capacity>,
}

impl<B: Batch> BatchUsageTracker<Local, B> for RcBatchUsageTracker<B> {
    fn with_capacity(capacity: Capacity) -> Self {
        Self {
            pointer: Cell::new(None),
            capacity: Cell::new(capacity)
        }
    }

    fn set(&self, batch: Rc<B>) -> (Rc<B>, Capacity) {
        let capacity = self.capacity.get();
        if capacity.is_loaded() {
            unreachable!("RcBatchUsageTracker state mismanagement: attempted set when already set")
        }
        if !capacity.was_unloaded() {
            unreachable!("RcBatchUsageTracker state mismanagement: attempted set when unloaded")
        }
        self.pointer.replace(Some(batch.clone()));
        self.capacity.update(|capacity| capacity.load());
        (batch, capacity)
    }

    fn acquire(&self) -> Option<(Rc<B>, Capacity)> {
        let capacity = self.capacity.get();
        if capacity.is_exhausted() {
            unreachable!("RcBatchUsageTracker state mismanagement: attempted acquire when capacity exhausted")
        }
        if !capacity.is_loaded() {
            if capacity.was_unloaded() {
                unreachable!("RcBatchUsageTracker state mismanagement: attempted acquire when unloaded")
            }
            return None;
        }
        let pointer;
        if capacity.count() == 1 {
            pointer = self.pointer.replace(None)?;
            self.capacity.update(|capacity| capacity.unload());
        } else {
            pointer = self.pointer.take()?;
            self.pointer.set(Some(pointer.clone()));
        }
        self.capacity.update(|capacity| capacity.take());
        Some((pointer, capacity))
    }
}


/// A thread-safe batch [`UsageTracker`](BatchUsageTracker).
///
/// Provides lock-free reads and atomic replacement of the stored batch
///
/// This type is `Send + Sync`.
pub(crate) struct ArcBatchUsageTracker<B: Batch> {
    pointer: ArcSwapOption<B>,
    capacity: AtomicU64,
}

impl<B: Batch> BatchUsageTracker<Atomic, B> for ArcBatchUsageTracker<B> {
    fn with_capacity(capacity: Capacity) -> Self {
        let count: u64 = capacity.into();
        Self {
            pointer: ArcSwapOption::new(None),
            capacity: AtomicU64::new(count - 1), // the usage from setting the batch is static
        }
    }

    /*
    This is a *simplification*, but the ordering is safe with relaxed
    because the operations are all on the same atomic value, so cannot be reordered.
    The block of operations itself cannot leak over the pointer operations because
    they use SeqCst, which acts like a barrier in both reorderring directions.
    */

    fn set(&self, batch: Arc<B>) -> (ArcGuard<B>, Capacity) {
        let val = self.capacity.load(Ordering::Relaxed);
        let capacity = Capacity::new(val).unwrap();
        if !capacity.is_locked() {
            unreachable!("ArcBatchUsageTracker state mismanagement: attempted set when not locked")
        }
        if capacity.is_loaded() {
            unreachable!("ArcBatchUsageTracker state mismanagement: attempted set when already set")
        }
        if capacity.was_unloaded() {
            unreachable!("ArcBatchUsageTracker state mismanagement: attempted set when unloaded")
        }
        self.pointer.store(Some(batch));
        let guard = ArcGuard::new(self.pointer.load());
        let val = self.capacity.fetch_xor(Capacity::IS_LOADED|Capacity::IS_LOCKED, Ordering::Relaxed);
        let capacity = Capacity::new(val).unwrap();
        if capacity.is_exhausted() {
            self.pointer.store(None);
            self.capacity.fetch_or(Capacity::WAS_UNLOADED, Ordering::Relaxed);
            return (guard, capacity.unload())
        }
        (guard, capacity)
    }

    fn acquire(&self) -> Option<(ArcGuard<B>, Capacity)> {
        let val = self.capacity.load(Ordering::Relaxed);
        let capacity = Capacity::new(val).unwrap();
        if capacity.is_exhausted() {
            unreachable!("ArcBatchUsageTracker state mismanagement: attempted acquire when exhausted")
        }
        if capacity.was_unloaded() {
            unreachable!("ArcBatchUsageTracker state mismanagement: attempted acquire when unloaded")
        }
        if !capacity.is_loaded() {
            if !capacity.is_locked() {
                let val = self.capacity.fetch_or(Capacity::IS_LOCKED, Ordering::Relaxed);
                if !Capacity::new(val).unwrap().is_locked() {
                    return None
                }
            }
            let mut spins = 0;
            while {
                let val = self.capacity.load(Ordering::Relaxed);
                Capacity::new(val).unwrap().is_locked()
            } {
                if spins < 50 {
                    core::hint::spin_loop();
                    spins += 1;
                } else {
                    std::thread::yield_now();
                }
            }
        };
        let guard = ArcGuard::new(self.pointer.load());
        let val = self.capacity.fetch_sub(1, Ordering::Relaxed);
        let capacity = Capacity::new(val).unwrap();
        if capacity.is_exhausted() {
            self.pointer.store(None);
            self.capacity.fetch_or(Capacity::WAS_UNLOADED, Ordering::Relaxed);
            return Some((guard, capacity.unload()))
        }
        Some((guard, capacity))
    }
}