use joltc_sys::*;

/// Collects serialized physics state for save/restore.
///
/// See also: Jolt's [`StateRecorder`](https://jrouwe.github.io/JoltPhysicsDocs/5.5.0/class_state_recorder.html) class.
pub struct StateRecorder {
    raw: *mut JPC_StateRecorder,
}

impl StateRecorder {
    pub fn new() -> Self {
        Self { raw: unsafe { JPC_StateRecorder_new() } }
    }

    /// Clear all recorded state.
    pub fn clear(&mut self) {
        unsafe { JPC_StateRecorder_Clear(self.raw) }
    }

    /// Rewind the read position back to the start (for restore).
    pub fn rewind(&mut self) {
        unsafe { JPC_StateRecorder_Rewind(self.raw) }
    }

    /// The raw bytes of recorded state.
    pub fn data(&self) -> &[u8] {
        let mut ptr: *const u8 = std::ptr::null();
        let mut len: usize = 0;
        unsafe { JPC_StateRecorder_GetData(self.raw, &mut ptr, &mut len) };
        if ptr.is_null() || len == 0 {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(ptr, len) }
        }
    }

    /// Number of bytes recorded.
    pub fn data_size(&self) -> usize {
        unsafe { JPC_StateRecorder_GetDataSize(self.raw) }
    }

    pub fn with_raw<R>(&self, f: impl FnOnce(*mut JPC_StateRecorder) -> R) -> R {
        f(self.raw)
    }

    pub fn raw(&self) -> *mut JPC_StateRecorder {
        self.raw
    }
}

impl Default for StateRecorder {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for StateRecorder {
    fn drop(&mut self) {
        unsafe { JPC_StateRecorder_delete(self.raw) }
    }
}
