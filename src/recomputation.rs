use std::{
    any::{Any, TypeId},
    collections::{hash_map::Entry, HashMap},
    hash::{BuildHasher, Hash, Hasher},
    ops::Deref,
    rc::Rc,
};

use graph::storage::DefaultAhashRandomstate;
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ComputationFingerprint([u64; 2]);

impl ComputationFingerprint {
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

#[derive(Clone, Copy)]
pub struct ComputationVersion {
    fingerprint: ComputationFingerprint,
    generation: u64,
}

#[derive(Clone)]
pub struct ComputationResult<T> {
    pub fingerprint: ComputationFingerprint,
    pub inner: T,
}

impl<T> ComputationResult<T> {
    pub fn fingerprint(&self) -> ComputationFingerprint {
        self.fingerprint
    }
    pub fn into_inner(self) -> T {
        self.inner
    }
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

pub struct ComputationEntry {
    result: Box<dyn Any>,
    arguments_fingerprint: ComputationFingerprint,
}

pub struct RecomputationCache {
    entries: HashMap<ComputationFingerprint, ComputationEntry, DefaultAhashRandomstate>,
}

impl RecomputationCache {
    pub fn new() -> Self {
        Self {
            entries: Default::default(),
        }
    }
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
                    let recomputed = fun();
                    entry.result = Box::new(recomputed.clone());
                    entry.arguments_fingerprint = arguments_fingerprint;

                    recomputed
                } else {
                    entry.result.downcast_ref::<T>().unwrap().clone()
                };

                result
            }
            Entry::Vacant(v) => {
                // eprintln!(
                //     "Inserting compute_impl {:?} {:?}",
                //     fingerprint.0,
                //     TypeId::of::<T>()
                // );
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
                // eprintln!(
                //     "Inserting get_or_insert_impl {:?} {:?}",
                //     fingerprint.0,
                //     TypeId::of::<T>()
                // );
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
    pub fn compute<T: Clone + 'static, F: FnOnce() -> T, L: Hash + ?Sized, S: Hash + ?Sized>(
        &mut self,
        location_extra: &L,
        input_state: &S,
        fun: F,
    ) -> ComputationResult<T> {
        let location_fingerprint = ComputationFingerprint::new(|state| {
            location_extra.hash(state);
        });

        let arguments_fingerprint = ComputationFingerprint::new(|state| {
            input_state.hash(state);
        });

        self.compute_impl(location_fingerprint, arguments_fingerprint, fun)
    }
    #[track_caller]
    pub fn compute_located<
        T: Clone + 'static,
        F: FnOnce() -> T,
        L: FnMut(&mut ahash::AHasher),
        S: FnMut(&mut ahash::AHasher),
    >(
        &mut self,
        mut computation_identity: L,
        mut computation_arguments: S,
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

fn make_hash<T: Hash>(h: &T) -> u64 {
    let mut hasher = (graph::storage::DefaultAhashRandomstate).build_hasher();
    h.hash(&mut hasher);
    hasher.finish()
}
