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
