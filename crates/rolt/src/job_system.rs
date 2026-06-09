use joltc_sys::*;

/// A physics job system — implemented by [`ThreadPoolJobSystem`] and [`SingleThreadedJobSystem`].
pub trait JobSystem {
    fn raw_job_system(&self) -> *mut JPC_JobSystem;
}

/// Fixed-size arena allocator for temporary physics allocations.
///
/// See also: Jolt's [`TempAllocatorImpl`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_temp_allocator_impl.html) class.
pub struct TempAllocator(*mut JPC_TempAllocatorImpl);

impl TempAllocator {
    pub fn new(size_bytes: u32) -> Self {
        Self(unsafe { JPC_TempAllocatorImpl_new(size_bytes) })
    }

    pub(crate) fn raw(&self) -> *mut JPC_TempAllocatorImpl {
        self.0
    }
}

impl Drop for TempAllocator {
    fn drop(&mut self) {
        unsafe { JPC_TempAllocatorImpl_delete(self.0) }
    }
}

/// Multi-threaded job system backed by a thread pool.
///
/// See also: Jolt's [`JobSystemThreadPool`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_job_system_thread_pool.html) class.
pub struct ThreadPoolJobSystem(*mut JPC_JobSystemThreadPool);

impl ThreadPoolJobSystem {
    pub fn new(max_jobs: u32, max_barriers: u32) -> Self {
        Self(unsafe { JPC_JobSystemThreadPool_new2(max_jobs, max_barriers) })
    }

    pub fn new_with_num_threads(max_jobs: u32, max_barriers: u32, num_threads: i32) -> Self {
        Self(unsafe { JPC_JobSystemThreadPool_new3(max_jobs, max_barriers, num_threads) })
    }
}

impl JobSystem for ThreadPoolJobSystem {
    fn raw_job_system(&self) -> *mut JPC_JobSystem {
        self.0.cast::<JPC_JobSystem>()
    }
}

impl Drop for ThreadPoolJobSystem {
    fn drop(&mut self) {
        unsafe { JPC_JobSystemThreadPool_delete(self.0) }
    }
}

/// Single-threaded job system for testing or single-core scenarios.
///
/// See also: Jolt's [`JobSystemSingleThreaded`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_job_system_single_threaded.html) class.
pub struct SingleThreadedJobSystem(*mut JPC_JobSystemSingleThreaded);

impl SingleThreadedJobSystem {
    pub fn new(max_jobs: u32) -> Self {
        Self(unsafe { JPC_JobSystemSingleThreaded_new(max_jobs) })
    }
}

impl JobSystem for SingleThreadedJobSystem {
    fn raw_job_system(&self) -> *mut JPC_JobSystem {
        self.0.cast::<JPC_JobSystem>()
    }
}

impl Drop for SingleThreadedJobSystem {
    fn drop(&mut self) {
        unsafe { JPC_JobSystemSingleThreaded_delete(self.0) }
    }
}

// private alias so DefaultJobSystem's new() has a single consistent signature
// regardless of which concrete type is selected underneath.
#[cfg(all(target_arch = "wasm32", not(target_feature = "atomics")))]
type DefaultJobSystemImpl = SingleThreadedJobSystem;
#[cfg(not(all(target_arch = "wasm32", not(target_feature = "atomics"))))]
type DefaultJobSystemImpl = ThreadPoolJobSystem;

/// platform-appropriate job system with a unified constructor.
///
/// Resolves to [`SingleThreadedJobSystem`] on `wasm32-unknown-emscripten`
/// without `-pthread` (no `target_feature = "atomics"`), and to
/// [`ThreadPoolJobSystem`] everywhere else. Use this instead of constructing
/// either type directly so game code compiles unchanged across targets.
pub struct DefaultJobSystem(DefaultJobSystemImpl);

impl DefaultJobSystem {
    /// create the platform-appropriate job system.
    /// `max_barriers` is silently ignored on single-threaded WASM builds.
    pub fn new(max_jobs: u32, max_barriers: u32) -> Self {
        #[cfg(all(target_arch = "wasm32", not(target_feature = "atomics")))]
        { let _ = max_barriers; return Self(SingleThreadedJobSystem::new(max_jobs)); }
        #[cfg(not(all(target_arch = "wasm32", not(target_feature = "atomics"))))]
        Self(ThreadPoolJobSystem::new(max_jobs, max_barriers))
    }
}

impl JobSystem for DefaultJobSystem {
    fn raw_job_system(&self) -> *mut JPC_JobSystem {
        self.0.raw_job_system()
    }
}
