use std::{
    rc::Rc,
    sync::atomic::{AtomicBool, Ordering},
};

#[derive(Clone)]
pub(crate) struct UnmountedLock(Rc<AtomicBool>);

impl UnmountedLock {
    pub(crate) fn new_mounted() -> Self {
        Self(Rc::new(AtomicBool::new(true)))
    }
    pub(crate) fn is_mounted(&self) -> bool {
        self.0.load(Ordering::Acquire)
    }
    pub(crate) fn unmount(&self) {
        self.0.store(false, Ordering::Acquire)
    }
}
