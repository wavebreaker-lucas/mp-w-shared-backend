use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use parking_lot::Mutex;
use tauri::PhysicalSize;

pub const STATE_STOPPED: u8 = 0;
pub const STATE_RUNNING: u8 = 1;
pub const STATE_PAUSED: u8 = 2;

pub const COMPACT_WIDTH: u32 = 600;
pub const COMPACT_HEIGHT: u32 = 900;

#[derive(Clone)]
pub struct TrackingState {
    pub state: Arc<AtomicU8>,
}

impl Default for TrackingState {
    fn default() -> Self {
        Self {
            state: Arc::new(AtomicU8::new(0))
        }
    }
}

impl TrackingState {
    pub fn is_running(&self) -> bool {
        self.state.load(Ordering::SeqCst) == STATE_RUNNING
    }

    pub fn set_state(&self, state: u8) {
        self.state.store(state, Ordering::SeqCst)
    }

    pub fn get_state(&self) -> u8 {
        self.state.load(Ordering::SeqCst)
    }
}

#[derive(Default)]
pub struct WindowState {
    pub original_size: Mutex<Option<PhysicalSize<u32>>>,
}

impl WindowState {
    pub fn set_size(&self, size: PhysicalSize<u32>) {
        *self.original_size.lock() = Some(size);
    }

    pub fn get_size(&self) -> Option<PhysicalSize<u32>> {
        *self.original_size.lock()
    }
}