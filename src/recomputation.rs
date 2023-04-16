//! A type erased hashmap which caches type erased computations.
//!
//! This is in essence `HashMap<computation_fingerprint, (arguments_fingerprint, computation result)>`.
//! Every retrieval provides an `arguments_fingerprint` which is compared to the stored one (if it exists) and calls the provided lambda if they don't match.

use graph::storage::DefaultAhashRandomstate;
use std::{
    any::{Any, TypeId},
    collections::{hash_map::Entry, HashMap},
    hash::{BuildHasher, Hash, Hasher},
    ops::Deref,
};

/// A 128 bit hash of something.
///
/// Since Ahash doesn't natively support getting a 128 bit hash, we just run the hasher two times with a different starting state.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ComputationFingerprint([u64; 2]);

impl ComputationFingerprint {
    /// Since we consider hashing to be random, the chance to get a fingerprint of any particular value is close to zero.
    /// This is just some value that can be used to initialize a fingerprint.
    const EMPTY: Self = Self([0; 2]);
    #[inline]
    fn new<F: FnMut(&mut ahash::AHasher)>(mut fun: F) -> Self {
        let mut hasher = (graph::storage::DefaultAhashRandomstate).build_hasher();
        fun(&mut hasher);
        let first = hasher.finish();

        let mut hasher = (graph::storage::DefaultAhashRandomstate).build_hasher();
        hasher.write_u64(first);
        fun(&mut hasher);
        let second = hasher.finish();

        Self([first, second])
    }
}

#[derive(Clone)]
pub struct ComputationResult<T> {
    /// The fingerprint of the computation
    pub fingerprint: ComputationFingerprint,
    /// The (possibly cached) result of the computation
    pub inner: T,
}

impl<T> Hash for ComputationResult<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.fingerprint.hash(state);
    }
}

impl<T> Deref for ComputationResult<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// Type erased entry of a computation
struct ComputationEntry {
    result: Box<dyn Any>,
    arguments_fingerprint: ComputationFingerprint,
}

/// The central cache object.
///
/// Not very flexible, no garbage collection is performed and for every computation only the last result is cached. Use something like salsa if you need more.
pub struct RecomputationCache {
    entries: HashMap<ComputationFingerprint, ComputationEntry, DefaultAhashRandomstate>,
}

impl RecomputationCache {
    pub fn new() -> Self {
        Self {
            entries: Default::default(),
        }
    }
    /// The implementation of the caching logic
    // TODO don't make this generic
    fn compute_impl<T: Clone + 'static, F: FnOnce() -> T>(
        &mut self,
        fingerprint: ComputationFingerprint,
        arguments_fingerprint: ComputationFingerprint,
        fun: F,
    ) -> ComputationResult<T> {
        let inner = match self.entries.entry(fingerprint) {
            Entry::Occupied(mut o) => {
                let entry = o.get_mut();
                assert_eq!(TypeId::of::<T>(), (*entry.result).type_id());

                let result = if entry.arguments_fingerprint != arguments_fingerprint {
                    // make sure to first drop the previous value before creating the new one
                    unsafe {
                        std::ptr::drop_in_place(&mut entry.result);
                    }
                    let recomputed = fun();
                    unsafe {
                        std::ptr::write(&mut entry.result, Box::new(recomputed.clone()));
                    }
                    entry.arguments_fingerprint = arguments_fingerprint;

                    recomputed
                } else {
                    entry.result.downcast_ref::<T>().unwrap().clone()
                };

                result
            }
            Entry::Vacant(v) => {
                let result = fun();
                v.insert(ComputationEntry {
                    result: Box::new(result.clone()),
                    arguments_fingerprint,
                });

                result
            }
        };

        ComputationResult { inner, fingerprint }
    }
    /// Retrieves a computed result or computes and inserts it if empty.
    /// Does no arguments_fingerprint checking.
    fn get_or_insert_impl<T: Clone + 'static, F: FnOnce() -> T>(
        &mut self,
        fingerprint: ComputationFingerprint,
        fun: F,
    ) -> ComputationResult<T> {
        let inner = match self.entries.entry(fingerprint) {
            Entry::Occupied(mut o) => {
                let entry = o.get_mut();
                assert_eq!(TypeId::of::<T>(), (*entry.result).type_id());

                let result = entry.result.downcast_ref::<T>().unwrap().clone();

                result
            }
            Entry::Vacant(v) => {
                let result = fun();
                v.insert(ComputationEntry {
                    result: Box::new(result.clone()),
                    arguments_fingerprint: ComputationFingerprint::EMPTY,
                });

                result
            }
        };

        ComputationResult { inner, fingerprint }
    }
    /// Caches some computation.
    ///
    /// A previous computation evaluation is returned if `arguments_identity` matches, otherwise `fun` is called. This only caches the last one, previous ones are discarded.
    ///
    /// * `computation_identity` - A value that uniquely identifies the computation, for example the fully qualified function name.
    /// * `arguments_identity` - A value that uniquely identifies all non-static computation arguments, usually just all of the function arguments.
    /// * `fun` - The computation itself. It must be the same for every invocation.
    pub fn compute<T: Clone + 'static, F: FnOnce() -> T, L: Hash + ?Sized, S: Hash + ?Sized>(
        &mut self,
        computation_identity: &L,
        arguments_identity: &S,
        fun: F,
    ) -> ComputationResult<T> {
        let computation_fingerprint = ComputationFingerprint::new(|state| {
            computation_identity.hash(state);
        });

        let arguments_fingerprint = ComputationFingerprint::new(|state| {
            arguments_identity.hash(state);
        });

        self.compute_impl(computation_fingerprint, arguments_fingerprint, fun)
    }
    /// Like [`Self::compute()`] but appends caller location to the computation_identity.
    ///
    /// This makes it possible to call with an empty `computation_identity`.
    ///
    /// * `computation_identity` - A lambda that hashes everything that uniquely identifies the computation.
    /// * `computation_arguments` - A lambda that hashes everything that uniquely identifies all non-static computation arguments, usually just all of the function arguments.
    /// * `fun` - The computation itself. It must be the same for every invocation.
    #[track_caller]
    pub fn compute_located<
        T: Clone + 'static,
        F: FnOnce() -> T,
        L: FnMut(&mut ahash::AHasher),
        S: FnMut(&mut ahash::AHasher),
    >(
        &mut self,
        mut computation_identity: L,
        computation_arguments: S,
        fun: F,
    ) -> ComputationResult<T> {
        let location = std::panic::Location::caller();
        let location_fingerprint = ComputationFingerprint::new(|state| {
            location.hash(state);
            computation_identity(state);
        });

        let arguments_fingerprint = ComputationFingerprint::new(computation_arguments);

        self.compute_impl(location_fingerprint, arguments_fingerprint, fun)
    }
    /// Like [`Self::compute()`] but uses solely the name for the computation_identity.
    ///
    /// * `name` - A string that uniquely identifies the computation, for example the fully qualified function name.
    /// * `computation_arguments` - A lambda that hashes everything that uniquely identifies all non-static computation arguments, usually just all of the function arguments.
    /// * `fun` - The computation itself. It must be the same for every invocation.
    pub fn compute_named<T: Clone + 'static, F: FnOnce() -> T, S: FnMut(&mut ahash::AHasher)>(
        &mut self,
        name: &str,
        computation_arguments: S,
        fun: F,
    ) -> ComputationResult<T> {
        let location_fingerprint = ComputationFingerprint::new(|state| {
            name.hash(state);
        });

        let arguments_fingerprint = ComputationFingerprint::new(computation_arguments);

        self.compute_impl(location_fingerprint, arguments_fingerprint, fun)
    }
    /// Retrieves a computation result which was created by [`Self::compute_named()`] or [`Self::get_or_insert_named()`].
    pub fn get_named<T: Clone + 'static>(&self, name: &str) -> Option<ComputationResult<T>> {
        let location_fingerprint = ComputationFingerprint::new(|state| {
            name.hash(state);
        });

        let entry = self.entries.get(&location_fingerprint)?;
        let result = entry.result.downcast_ref::<T>().cloned()?;
        Some(ComputationResult {
            fingerprint: location_fingerprint,
            inner: result,
        })
    }
    /// Caches a computation solely identified by a name like [`Self::compute_named()`] but doesn't have a `computation_arguments` argument.
    /// This makes it suitable for one-time computations that have to be computed at runtime, not unlike `lazy_static!`.
    pub fn get_or_insert_named<T: Clone + 'static, F: FnOnce() -> T>(
        &mut self,
        name: &str,
        fun: F,
    ) -> ComputationResult<T> {
        let location_fingerprint = ComputationFingerprint::new(|state| {
            name.hash(state);
        });

        self.get_or_insert_impl(location_fingerprint, fun)
    }
}
